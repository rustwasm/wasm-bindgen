use proc_macro2::Span;
use quote::{Tokens, ToTokens};
use shared;
use syn;

pub struct Program {
    pub structs: Vec<Struct>,
    pub free_functions: Vec<Function>,
    pub imports: Vec<Import>,
    pub imported_structs: Vec<ImportStruct>,
}

pub struct Function {
    pub name: syn::Ident,
    pub arguments: Vec<Type>,
    pub ret: Option<Type>,
}

pub struct Import {
    pub module: String,
    pub function: ImportFunction,
}

pub struct ImportFunction {
    pub ident: syn::Ident,
    pub wasm_function: Function,
    pub rust_decl: Box<syn::FnDecl>,
    pub rust_vis: syn::Visibility,
    pub rust_attrs: Vec<syn::Attribute>,
}

pub struct ImportStruct {
    pub module: Option<String>,
    pub name: syn::Ident,
    pub functions: Vec<(ImportFunctionKind, ImportFunction)>,
}

pub enum ImportFunctionKind {
    Method,
    Static,
    JsConstructor,
}

pub enum Type {
    // special
    BorrowedStr,
    String,

    ByRef(syn::Type),
    ByMutRef(syn::Type),
    ByValue(syn::Type),
}

pub struct Struct {
    pub name: syn::Ident,
    pub methods: Vec<Method>,
    pub functions: Vec<Function>,
}

pub struct Method {
    pub mutable: bool,
    pub function: Function,
}

impl Program {
    pub fn push_impl(&mut self, item: &syn::ItemImpl) {
        if item.defaultness.is_some() {
            panic!("default impls are not supported");
        }
        if item.unsafety.is_some() {
            panic!("unsafe impls are not supported");
        }
        if item.trait_.is_some() {
            panic!("trait impls are not supported");
        }
        if item.generics.params.len() > 0 {
            panic!("generic impls aren't supported");
        }
        let name = match *item.self_ty {
            syn::Type::Path(syn::TypePath { qself: None, ref path }) => {
                match extract_path_ident(path) {
                    Some(ident) => ident,
                    None => panic!("unsupported self type in impl"),
                }
            }
            _ => panic!("unsupported self type in impl"),
        };
        let dst = self.structs
            .iter_mut()
            .find(|s| s.name == name)
            .expect(&format!("failed to definition of struct for impl of `{}`", name));
        for item in item.items.iter() {
            dst.push_item(item);
        }
    }

    pub fn push_foreign_mod(&mut self, f: &syn::ItemForeignMod) {
        match f.abi.name {
            Some(ref l) if l.value() == "JS" => {}
            _ => panic!("only foreign mods with the `JS` ABI are allowed"),
        }
        let module = f.attrs.iter()
            .filter_map(|f| f.interpret_meta())
            .filter_map(|i| {
                match i {
                    syn::Meta::NameValue(i) => {
                        if i.ident == "wasm_module" {
                            Some(i.lit)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            })
            .next()
            .and_then(|lit| {
                match lit {
                    syn::Lit::Str(v) => Some(v.value()),
                    _ => None,
                }
            })
            .expect("must specify `#[wasm_module = ...]` for module to import from");
        for item in f.items.iter() {
            let import = self.gen_foreign_item(item, false).0;
            self.imports.push(Import {
                module: module.clone(),
                function: import,
            });
        }
    }

    pub fn gen_foreign_item(&mut self,
                            f: &syn::ForeignItem,
                            allow_self: bool) -> (ImportFunction, bool) {
        let f = match *f {
            syn::ForeignItem::Fn(ref f) => f,
            _ => panic!("only foreign functions allowed for now, not statics"),
        };

        let (wasm, mutable) = Function::from_decl(f.ident, &f.decl, allow_self);
        let is_method = match mutable {
            Some(false) => true,
            None => false,
            Some(true) => {
                panic!("mutable self methods not allowed in extern structs");
            }
        };

        (ImportFunction {
            rust_attrs: f.attrs.clone(),
            rust_vis: f.vis.clone(),
            rust_decl: f.decl.clone(),
            ident: f.ident.clone(),
            wasm_function: wasm,
        }, is_method)
    }

    pub fn push_extern_class(&mut self, class: &ExternClass) {
        let functions = class.functions.iter()
            .map(|f| {
                let (f, method) = self.gen_foreign_item(f, true);
                let kind = if method {
                    ImportFunctionKind::Method
                } else {
                    let new = f.rust_attrs.iter()
                        .filter_map(|a| a.interpret_meta())
                        .filter_map(|m| {
                            match m {
                                syn::Meta::List(i) => {
                                    if i.ident == "wasm_bindgen" {
                                        Some(i.nested)
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            }
                        })
                        .flat_map(|a| a)
                        .filter_map(|a| {
                            match a {
                                syn::NestedMeta::Meta(a) => Some(a),
                                _ => None,
                            }
                        })
                        .any(|a| {
                            match a {
                                syn::Meta::Word(a) => a == "constructor",
                                _ => false,
                            }
                        });
                    if new {
                        ImportFunctionKind::JsConstructor
                    } else {
                        ImportFunctionKind::Static
                    }
                };
                (kind, f)
            })
            .collect();
        self.imported_structs.push(ImportStruct {
            module: class.module.as_ref().map(|s| s.value()),
            name: class.name,
            functions,
        });
    }

    pub fn wbg_literal(&self, dst: &mut Tokens) -> usize {
        let mut a = LiteralBuilder {
            dst,
            cnt: 0,
        };
        a.append("wbg:");
        a.fields(&[
            ("structs", &|a| a.list(&self.structs, Struct::wbg_literal)),
            ("free_functions", &|a| a.list(&self.free_functions, Function::wbg_literal)),
            ("imports", &|a| a.list(&self.imports, Import::wbg_literal)),
            ("imported_structs", &|a| a.list(&self.imported_structs, ImportStruct::wbg_literal)),
            ("custom_type_names", &|a| {
                a.list(&self.structs, |s, a| {
                    let val = shared::name_to_descriptor(s.name.as_ref());
                    a.fields(&[
                        ("descriptor", &|a| a.char(val)),
                        ("name", &|a| a.str(s.name.as_ref()))
                    ]);
                })
            }),
        ]);
        return a.cnt
    }
}

impl Function {
    pub fn from(input: &syn::ItemFn) -> Function {
        match input.vis {
            syn::Visibility::Public(_) => {}
            _ => panic!("can only bindgen public functions"),
        }
        if input.constness.is_some() {
            panic!("can only bindgen non-const functions");
        }
        if input.unsafety.is_some() {
            panic!("can only bindgen safe functions");
        }
        if !input.abi.is_none() {
            panic!("can only bindgen Rust ABI functions")
        }
        if !input.abi.is_none() {
            panic!("can only bindgen Rust ABI functions")
        }

        Function::from_decl(input.ident, &input.decl, false).0
    }

    pub fn from_decl(name: syn::Ident,
                     decl: &syn::FnDecl,
                     allow_self: bool) -> (Function, Option<bool>) {
        if decl.variadic.is_some() {
            panic!("can't bindgen variadic functions")
        }
        if decl.generics.params.len() > 0 {
            panic!("can't bindgen functions with lifetime or type parameters")
        }

        let mut mutable = None;
        let arguments = decl.inputs.iter()
            .filter_map(|arg| {
                match *arg {
                    syn::FnArg::Captured(ref c) => Some(c),
                    syn::FnArg::SelfValue(_) => {
                        panic!("by-value `self` not yet supported");
                    }
                    syn::FnArg::SelfRef(ref a) if allow_self => {
                        assert!(mutable.is_none());
                        mutable = Some(a.mutability.is_some());
                        None
                    }
                    _ => panic!("arguments cannot be `self` or ignored"),
                }
            })
            .map(|arg| Type::from(&arg.ty))
            .collect::<Vec<_>>();

        let ret = match decl.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ref t) => Some(Type::from(t)),
        };

        (Function { name, arguments, ret }, mutable)
    }

    pub fn free_function_export_name(&self) -> syn::LitStr {
        let name = shared::free_function_export_name(self.name.as_ref());
        syn::LitStr::new(&name, Span::def_site())
    }

    pub fn struct_function_export_name(&self, s: syn::Ident) -> syn::LitStr {
        let name = shared::struct_function_export_name(
            s.as_ref(),
            self.name.as_ref(),
        );
        syn::LitStr::new(&name, Span::def_site())
    }

    pub fn rust_symbol(&self, namespace: Option<syn::Ident>) -> syn::Ident {
        let mut generated_name = format!("__wasm_bindgen_generated");
        if let Some(ns) = namespace {
            generated_name.push_str("_");
            generated_name.push_str(ns.as_ref());
        }
        generated_name.push_str("_");
        generated_name.push_str(self.name.as_ref());
        syn::Ident::from(generated_name)
    }

    fn wbg_literal(&self, a: &mut LiteralBuilder) {
        a.fields(&[
            ("name", &|a| a.str(self.name.as_ref())),
            ("arguments", &|a| a.list(&self.arguments, Type::wbg_literal)),
            ("ret", &|a| {
                match self.ret {
                    Some(ref s) => s.wbg_literal(a),
                    None => a.append("null"),
                }
            }),
        ]);
    }
}

pub fn extract_path_ident(path: &syn::Path) -> Option<syn::Ident> {
    if path.leading_colon.is_some() {
        return None
    }
    if path.segments.len() != 1 {
        return None
    }
    match path.segments.first().unwrap().value().arguments {
        syn::PathArguments::None => {}
        _ => return None,
    }
    path.segments.first().map(|v| v.value().ident)
}

impl Type {
    pub fn from(ty: &syn::Type) -> Type {
        match *ty {
            syn::Type::Reference(ref r) => {
                match *r.elem {
                    syn::Type::Path(syn::TypePath { qself: None, ref path }) => {
                        let ident = extract_path_ident(path);
                        match ident.as_ref().map(|s| s.as_ref()) {
                            Some("str") => return Type::BorrowedStr,
                            _ => {}
                        }
                    }
                    _ => {}
                }
                return if r.mutability.is_some() {
                    Type::ByMutRef((*r.elem).clone())
                } else {
                    Type::ByRef((*r.elem).clone())
                }
            }
            syn::Type::Path(syn::TypePath { qself: None, ref path }) => {
                let ident = extract_path_ident(path);
                match ident.as_ref().map(|s| s.as_ref()) {
                    Some("String") => return Type::String,
                    _ => {}
                }
            }
            _ => {}
        }

        Type::ByValue(ty.clone())
    }

    fn wbg_literal(&self, a: &mut LiteralBuilder) {
        match *self {
            Type::BorrowedStr => a.char(shared::TYPE_BORROWED_STR),
            Type::String => a.char(shared::TYPE_STRING),
            Type::ByValue(ref t) => {
                a.as_char(my_quote! {
                    <#t as ::wasm_bindgen::convert::WasmBoundary>::DESCRIPTOR
                });
            }
            Type::ByRef(ref ty) |
            Type::ByMutRef(ref ty) => {
                a.as_char(my_quote! {
                    (<#ty as ::wasm_bindgen::convert::WasmBoundary>::DESCRIPTOR |
                        ::wasm_bindgen::convert::DESCRIPTOR_CUSTOM_REF_FLAG)
                });
            }
        }
    }
}

impl Struct {
    pub fn from(s: &syn::ItemStruct) -> Struct {
        Struct {
            name: s.ident,
            methods: Vec::new(),
            functions: Vec::new(),
        }
    }

    pub fn free_function(&self) -> syn::Ident {
        syn::Ident::from(shared::free_function(self.name.as_ref()))
    }

    pub fn push_item(&mut self, item: &syn::ImplItem) {
        let method = match *item {
            syn::ImplItem::Const(_) => panic!("const definitions aren't supported"),
            syn::ImplItem::Type(_) => panic!("type definitions in impls aren't supported"),
            syn::ImplItem::Method(ref m) => m,
            syn::ImplItem::Macro(_) => panic!("macros in impls aren't supported"),
            syn::ImplItem::Verbatim(_) => panic!("unparsed impl item?"),
        };
        match method.vis {
            syn::Visibility::Public(_) => {}
            _ => return,
        }
        if method.defaultness.is_some() {
            panic!("default methods are not supported");
        }
        if method.sig.constness.is_some() {
            panic!("can only bindgen non-const functions");
        }
        if method.sig.unsafety.is_some() {
            panic!("can only bindgen safe functions");
        }

        let (function, mutable) = Function::from_decl(method.sig.ident,
                                                      &method.sig.decl,
                                                      true);
        match mutable {
            Some(mutable) => {
                self.methods.push(Method { mutable, function });
            }
            None => {
                self.functions.push(function);
            }
        }
    }

    fn wbg_literal(&self, a: &mut LiteralBuilder) {
        a.fields(&[
            ("name", &|a| a.str(self.name.as_ref())),
            ("functions", &|a| a.list(&self.functions, Function::wbg_literal)),
            ("methods", &|a| a.list(&self.methods, Method::wbg_literal)),
        ]);
    }
}

impl Method {
    fn wbg_literal(&self, a: &mut LiteralBuilder) {
        a.fields(&[
            ("mutable", &|a| a.bool(self.mutable)),
            ("function", &|a| self.function.wbg_literal(a)),
        ]);
    }
}

impl ImportStruct {
    fn wbg_literal(&self, a: &mut LiteralBuilder) {
        a.fields(&[
            ("module", &|a| {
                match self.module {
                    Some(ref s) => a.str(s),
                    None => a.append("null"),
                }
            }),
            ("name", &|a| a.str(self.name.as_ref())),
            ("functions", &|a| {
                a.list(&self.functions,
                       |&(ref kind, ref f), a| {
                           let (method, new) = match *kind {
                               ImportFunctionKind::Method => (true, false),
                               ImportFunctionKind::JsConstructor => (false, true),
                               ImportFunctionKind::Static => (false, false),
                           };
                           a.fields(&[
                                ("method", &|a| a.bool(method)),
                                ("js_new", &|a| a.bool(new)),
                                ("function", &|a| f.wasm_function.wbg_literal(a)),
                           ]);
                       })
            }),
        ]);
    }
}

impl Import {
    fn wbg_literal(&self, a: &mut LiteralBuilder) {
        a.fields(&[
            ("module", &|a| a.str(&self.module)),
            ("function", &|a| self.function.wasm_function.wbg_literal(a)),
        ]);
    }
}

pub struct File {
    pub items: Vec<MyItem>,
}

impl syn::synom::Synom for File {
    named!(parse -> Self, map!(many0!(syn!(MyItem)), |items| File { items }));
}

pub enum MyItem {
    Normal(syn::Item),
    ExternClass(ExternClass),
}

impl syn::synom::Synom for MyItem {
    named!(parse -> Self, alt!(
        syn!(syn::Item) => { MyItem::Normal }
        |
        syn!(ExternClass) => { MyItem::ExternClass }
    ));
}

pub struct ExternClass {
    name: syn::Ident,
    module: Option<syn::LitStr>,
    functions: Vec<syn::ForeignItem>,
}

impl syn::synom::Synom for ExternClass {
    named!(parse -> Self, do_parse!(
        module: option!(do_parse!(
            punct!(#) >>
            name: brackets!(do_parse!(
                call!(term, "wasm_module") >>
                punct!(=) >>
                val: syn!(syn::LitStr) >>
                (val)
            )) >>
            (name.1)
        )) >>
        keyword!(extern) >>
        keyword!(struct) >>
        name: syn!(syn::Ident) >>
        items: braces!(many0!(syn!(syn::ForeignItem))) >>
        (ExternClass {
            name,
            module,
            functions: items.1,
        })
    ));
}

fn term<'a>(cursor: syn::buffer::Cursor<'a>, name: &str)
    -> syn::synom::PResult<'a, ()>
{
    if let Some((_span, term, next)) = cursor.term() {
        if term.as_str() == name {
            return Ok(((), next))
        }
    }
    syn::parse_error()
}

struct LiteralBuilder<'a> {
    dst: &'a mut Tokens,
    cnt: usize,
}

impl<'a> LiteralBuilder<'a> {
    fn char_lit(&mut self, c: char) {
        if self.cnt > 0 {
            ::syn::token::Comma::default().to_tokens(self.dst);
        }
        self.cnt += 1;
        (c as u32).to_tokens(self.dst);
    }

    fn append(&mut self, s: &str) {
        for c in s.chars() {
            self.char_lit(c);
        }
    }

    fn str(&mut self, s: &str) {
        self.append("\"");
        self.append(s);
        self.append("\"");
    }

    fn bool(&mut self, v: bool) {
        if v {
            self.append("true")
        } else {
            self.append("false")
        }
    }

    fn char(&mut self, s: char) {
        self.append("\"");
        self.char_lit(s);
        self.append("\"");
    }

    fn as_char(&mut self, tokens: Tokens) {
        self.append("\"");
        ::syn::token::Comma::default().to_tokens(self.dst);
        tokens.to_tokens(self.dst);
        self.cnt += 1;
        self.append("\"");
    }

    fn fields(&mut self, fields: &[(&str, &Fn(&mut Self))]) {
        self.append("{");
        for (i, &(field, cb)) in fields.iter().enumerate() {
            if i > 0 {
                self.append(",");
            }
            self.str(field);
            self.append(":");
            cb(self);
        }
        self.append("}");
    }

    fn list<T, F>(&mut self, list: T, mut cb: F)
        where F: FnMut(T::Item, &mut Self),
              T: IntoIterator,
    {
        self.append("[");
        for (i, element) in list.into_iter().enumerate() {
            if i > 0 {
                self.append(",");
            }
            cb(element, self);
        }
        self.append("]");
    }
}
