use std::collections::BTreeSet;

use proc_macro2::Span;
use quote::{Tokens, ToTokens};
use shared;
use syn;

#[derive(Default)]
pub struct Program {
    pub exports: Vec<Export>,
    pub imports: Vec<Import>,
    pub imported_types: Vec<(syn::Visibility, syn::Ident)>,
    pub structs: Vec<Struct>,
}

pub struct Export {
    pub class: Option<syn::Ident>,
    pub method: bool,
    pub mutable: bool,
    pub function: Function,
}

pub struct Import {
    pub module: Option<String>,
    pub kind: ImportKind,
    pub function: Function,
}

pub enum ImportKind {
    Method { class: String, ty: syn::Type },
    Static { class: String, ty: syn::Type },
    JsConstructor { class: String, ty: syn::Type },
    Normal,
}

pub struct Function {
    pub name: syn::Ident,
    pub arguments: Vec<Type>,
    pub ret: Option<Type>,
    pub opts: BindgenAttrs,
    pub rust_attrs: Vec<syn::Attribute>,
    pub rust_decl: Box<syn::FnDecl>,
    pub rust_vis: syn::Visibility,
}

pub struct Struct {
    pub name: syn::Ident,
}

pub enum Type {
    // special
    BorrowedStr,
    String,

    ByRef(syn::Type),
    ByMutRef(syn::Type),
    ByValue(syn::Type),
}

impl Program {
    pub fn push_item(&mut self,
                     item: syn::Item,
                     opts: Option<BindgenAttrs>,
                     tokens: &mut Tokens) {
        match item {
            syn::Item::Fn(mut f) => {
                let opts = opts.unwrap_or_else(|| BindgenAttrs::find(&mut f.attrs));

                let no_mangle = f.attrs.iter()
                    .enumerate()
                    .filter_map(|(i, m)| m.interpret_meta().map(|m| (i, m)))
                    .find(|&(_, ref m)| m.name() == "no_mangle");
                match no_mangle {
                    Some((i, _)) => { f.attrs.remove(i); }
                    None => {
                        panic!("#[wasm_bindgen] can only be applied to #[no_mangle] \
                                functions, or those that would otherwise be exported")
                    }
                }
                f.to_tokens(tokens);
                self.exports.push(Export {
                    class: None,
                    method: false,
                    mutable: false,
                    function: Function::from(f, opts),
                });
            }
            syn::Item::Struct(mut s) => {
                let opts = opts.unwrap_or_else(|| BindgenAttrs::find(&mut s.attrs));
                s.to_tokens(tokens);
                self.structs.push(Struct::from(s, opts));
            }
            syn::Item::Impl(mut i) => {
                let opts = opts.unwrap_or_else(|| BindgenAttrs::find(&mut i.attrs));
                i.to_tokens(tokens);
                self.push_impl(i, opts);
            }
            syn::Item::ForeignMod(mut f) => {
                let opts = opts.unwrap_or_else(|| BindgenAttrs::find(&mut f.attrs));
                self.push_foreign_mod(f, opts);
            }
            _ => panic!("#[wasm_bindgen] can only be applied to a function, \
                         struct, impl, or extern block"),
        }
    }

    pub fn push_impl(&mut self, item: syn::ItemImpl, _opts: BindgenAttrs) {
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
        for item in item.items.into_iter() {
            self.push_impl_item(name, item);
        }
    }

    fn push_impl_item(&mut self, class: syn::Ident, item: syn::ImplItem) {
        let mut method = match item {
            syn::ImplItem::Const(_) => panic!("const definitions aren't supported"),
            syn::ImplItem::Type(_) => panic!("type definitions in impls aren't supported"),
            syn::ImplItem::Method(m) => m,
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

        let opts = BindgenAttrs::find(&mut method.attrs);

        let (function, mutable) = Function::from_decl(method.sig.ident,
                                                      Box::new(method.sig.decl),
                                                      method.attrs,
                                                      opts,
                                                      method.vis,
                                                      true);
        self.exports.push(Export {
            class: Some(class),
            method: mutable.is_some(),
            mutable: mutable.unwrap_or(false),
            function,
        });
    }

    pub fn push_foreign_mod(&mut self, f: syn::ItemForeignMod, opts: BindgenAttrs) {
        match f.abi.name {
            Some(ref l) if l.value() == "C" => {}
            None => {}
            _ => panic!("only foreign mods with the `C` ABI are allowed"),
        }
        for item in f.items.into_iter() {
            match item {
                syn::ForeignItem::Fn(f) => self.push_foreign_fn(f, &opts),
                syn::ForeignItem::Type(t) => self.push_foreign_ty(t, &opts),
                _ => panic!("only foreign functions/types allowed for now"),
            }
        }
    }

    pub fn push_foreign_fn(&mut self,
                           mut f: syn::ForeignItemFn,
                           module_opts: &BindgenAttrs) {
        let opts = BindgenAttrs::find(&mut f.attrs);

        let mut wasm = Function::from_decl(f.ident,
                                           f.decl,
                                           f.attrs,
                                           opts,
                                           f.vis,
                                           false).0;
        if wasm.opts.catch() {
            // TODO: this assumes a whole bunch:
            //
            // * The outer type is actually a `Result`
            // * The error type is a `JsValue`
            // * The actual type is the first type parameter
            //
            // should probably fix this one day...
            wasm.ret = extract_first_ty_param(wasm.ret.as_ref())
                .expect("can't `catch` without returning a Result");
        }

        let kind = if wasm.opts.method() {
            let class = wasm.arguments.get(0)
                .expect("methods must have at least one argument");
            let class = match *class {
                Type::ByRef(ref t) |
                Type::ByValue(ref t) => t,
                Type::ByMutRef(_) => {
                    panic!("first method argument cannot be mutable ref")
                }
                Type::String | Type::BorrowedStr => {
                    panic!("method receivers cannot be strings")
                }
            };
            let class_name = match *class {
                syn::Type::Path(syn::TypePath { qself: None, ref path }) => path,
                _ => panic!("first argument of method must be a path"),
            };
            let class_name = extract_path_ident(class_name)
                .expect("first argument of method must be a bare type");

            ImportKind::Method {
                class: class_name.as_ref().to_string(),
                ty: class.clone(),
            }
        } else if wasm.opts.constructor() {
            let class = match wasm.ret {
                Some(Type::ByValue(ref t)) => t,
                _ => panic!("constructor returns must be bare types"),
            };
            let class_name = match *class {
                syn::Type::Path(syn::TypePath { qself: None, ref path }) => path,
                _ => panic!("first argument of method must be a path"),
            };
            let class_name = extract_path_ident(class_name)
                .expect("first argument of method must be a bare type");

            ImportKind::JsConstructor {
                class: class_name.as_ref().to_string(),
                ty: class.clone(),
            }

        } else if let Some(class) = wasm.opts.static_receiver() {
            let class_name = match *class {
                syn::Type::Path(syn::TypePath { qself: None, ref path }) => path,
                _ => panic!("first argument of method must be a path"),
            };
            let class_name = extract_path_ident(class_name)
                .expect("first argument of method must be a bare type");
            ImportKind::Static {
                class: class_name.to_string(),
                ty: class.clone(),
            }
        } else {
            ImportKind::Normal
        };

        self.imports.push(Import {
            module: module_opts.module().map(|s| s.to_string()),
            kind,
            function: wasm,
        });
    }

    pub fn push_foreign_ty(&mut self,
                           f: syn::ForeignItemType,
                           _module_opts: &BindgenAttrs) {
        self.imported_types.push((f.vis, f.ident));
    }

    pub fn wbg_literal(&self, dst: &mut Tokens) -> usize {
        let mut tmp = Tokens::new();
        let cnt = {
            let mut a = LiteralBuilder {
                dst: &mut tmp,
                cnt: 0,
            };
            a.fields(&[
                ("exports", &|a| a.list(&self.exports, Export::wbg_literal)),
                ("imports", &|a| a.list(&self.imports, Import::wbg_literal)),
                ("custom_type_names", &|a| {
                    let names = self.exports.iter()
                        .filter_map(|e| e.class)
                        .chain(self.structs.iter().map(|s| s.name))
                        .collect::<BTreeSet<_>>();
                    a.list(&names, |s, a| {
                        let val = shared::name_to_descriptor(s.as_ref());
                        a.fields(&[
                            ("descriptor", &|a| a.char(val)),
                            ("name", &|a| a.str(s.as_ref()))
                        ]);
                    })
                }),
            ]);
            a.cnt
        };
        let cnt = cnt as u32;
        (quote! {
            0x30d97887,
            0xd4182f61,
            #cnt,
        }).to_tokens(dst);
        tmp.to_tokens(dst);
        (cnt as usize) + 3
    }
}

impl Function {
    pub fn from(input: syn::ItemFn, opts: BindgenAttrs) -> Function {
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
        if input.abi.is_none() {
            panic!("can only bindgen `extern` ABI functions, or those that \
                    would otherwise be exported")
        }

        Function::from_decl(input.ident,
                            input.decl,
                            input.attrs,
                            opts,
                            input.vis,
                            false).0
    }

    pub fn from_decl(name: syn::Ident,
                     decl: Box<syn::FnDecl>,
                     attrs: Vec<syn::Attribute>,
                     opts: BindgenAttrs,
                     vis: syn::Visibility,
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

        (Function {
            name,
            arguments,
            ret,
            opts,
            rust_vis: vis,
            rust_decl: decl,
            rust_attrs: attrs,
        }, mutable)
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

impl Export {
    pub fn rust_symbol(&self) -> syn::Ident {
        let mut generated_name = format!("__wasm_bindgen_generated");
        if let Some(class) = self.class {
            generated_name.push_str("_");
            generated_name.push_str(class.as_ref());
        }
        generated_name.push_str("_");
        generated_name.push_str(self.function.name.as_ref());
        syn::Ident::from(generated_name)
    }

    pub fn export_name(&self) -> syn::LitStr {
        let name = match self.class {
            Some(class) => {
                shared::struct_function_export_name(
                    class.as_ref(),
                    self.function.name.as_ref(),
                )
            }
            None => {
                shared::free_function_export_name(self.function.name.as_ref())
            }
        };
        syn::LitStr::new(&name, Span::def_site())
    }

    fn wbg_literal(&self, a: &mut LiteralBuilder) {
        a.fields(&[
            ("class", &|a| {
                match self.class {
                    Some(ref s) => a.str(s.as_ref()),
                    None => a.append("null"),
                }
            }),
            ("method", &|a| a.bool(self.method)),
            ("function", &|a| self.function.wbg_literal(a)),
        ]);
    }
}

impl Import {
    fn wbg_literal(&self, a: &mut LiteralBuilder) {
        let mut method = false;
        let mut js_new = false;
        let mut statik = false;
        let mut class_name = None;
        match self.kind {
            ImportKind::Method { ref class, .. } => {
                method = true;
                class_name = Some(class);
            }
            ImportKind::JsConstructor { ref class, .. } => {
                js_new = true;
                class_name = Some(class);
            }
            ImportKind::Static { ref class, .. } => {
                statik = true;
                class_name = Some(class);
            }
            ImportKind::Normal => {}
        }

        let mut getter = None;
        let mut setter = None;

        if self.function.opts.getter() {
            getter = Some(self.infer_getter_property());
        }
        if self.function.opts.setter() {
            setter = Some(self.infer_setter_property());
        }
        a.fields(&[
            ("module", &|a| {
                match self.module {
                    Some(ref s) => a.str(s),
                    None => a.append("null"),
                }
            }),
            ("catch", &|a| a.bool(self.function.opts.catch())),
            ("method", &|a| a.bool(method)),
            ("js_new", &|a| a.bool(js_new)),
            ("statik", &|a| a.bool(statik)),
            ("getter", &|a| {
                match getter {
                    Some(ref s) => a.str(s),
                    None => a.append("null"),
                }
            }),
            ("setter", &|a| {
                match setter {
                    Some(ref s) => a.str(s),
                    None => a.append("null"),
                }
            }),
            ("function", &|a| self.function.wbg_literal(a)),
            ("class", &|a| {
                match class_name {
                    Some(s) => a.str(s),
                    None => a.append("null"),
                }
            }),
        ]);
    }

    fn infer_getter_property(&self) -> String {
        self.function.name.as_ref().to_string()
    }

    fn infer_setter_property(&self) -> String {
        let name = self.function.name.as_ref();
        assert!(name.starts_with("set_"), "setters must start with `set_`");
        name[4..].to_string()
    }
}

impl Struct {
    fn from(s: syn::ItemStruct, _opts: BindgenAttrs) -> Struct {
        Struct { name: s.ident }
    }
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

#[derive(Default)]
pub struct BindgenAttrs {
    attrs: Vec<BindgenAttr>,
}

impl BindgenAttrs {
    pub fn find(attrs: &mut Vec<syn::Attribute>) -> BindgenAttrs {
        let pos = attrs.iter()
            .enumerate()
            .find(|&(_, ref m)| m.path.segments[0].ident == "wasm_bindgen")
            .map(|a| a.0);
        let pos = match pos {
            Some(i) => i,
            None => return BindgenAttrs::default(),
        };
        syn::parse(attrs.remove(pos).tts.into())
            .expect("malformed #[wasm_bindgen] attribute")
    }

    fn module(&self) -> Option<&str> {
        self.attrs.iter()
            .filter_map(|a| {
                match *a {
                    BindgenAttr::Module(ref s) => Some(&s[..]),
                    _ => None,
                }
            })
            .next()
    }

    pub fn catch(&self) -> bool {
        self.attrs.iter()
            .any(|a| {
                match *a {
                    BindgenAttr::Catch => true,
                    _ => false,
                }
            })
    }

    fn constructor(&self) -> bool {
        self.attrs.iter()
            .any(|a| {
                match *a {
                    BindgenAttr::Constructor => true,
                    _ => false,
                }
            })
    }

    fn method(&self) -> bool {
        self.attrs.iter()
            .any(|a| {
                match *a {
                    BindgenAttr::Method => true,
                    _ => false,
                }
            })
    }

    fn static_receiver(&self) -> Option<&syn::Type> {
        self.attrs.iter()
            .filter_map(|a| {
                match *a {
                    BindgenAttr::Static(ref s) => Some(s),
                    _ => None,
                }
            })
            .next()
    }

    fn getter(&self) -> bool {
        self.attrs.iter()
            .any(|a| {
                match *a {
                    BindgenAttr::Getter => true,
                    _ => false,
                }
            })
    }

    fn setter(&self) -> bool {
        self.attrs.iter()
            .any(|a| {
                match *a {
                    BindgenAttr::Setter => true,
                    _ => false,
                }
            })
    }
}

impl syn::synom::Synom for BindgenAttrs {
    named!(parse -> Self, alt!(
        do_parse!(
            opts: parens!(call!(
                syn::punctuated::Punctuated::<_, syn::token::Comma>::parse_terminated
            )) >>
            (BindgenAttrs {
                attrs: opts.1.into_iter().collect(),
            })
        ) => { |s| s }
        |
        epsilon!() => { |_| BindgenAttrs { attrs: Vec::new() } }
    ));
}

enum BindgenAttr {
    Catch,
    Constructor,
    Method,
    Static(syn::Type),
    Module(String),
    Getter,
    Setter,
}

impl syn::synom::Synom for BindgenAttr {
    named!(parse -> Self, alt!(
        call!(term, "catch") => { |_| BindgenAttr::Catch }
        |
        call!(term, "constructor") => { |_| BindgenAttr::Constructor }
        |
        call!(term, "method") => { |_| BindgenAttr::Method }
        |
        call!(term, "getter") => { |_| BindgenAttr::Getter }
        |
        call!(term, "setter") => { |_| BindgenAttr::Setter }
        |
        do_parse!(
            call!(term, "static") >>
            punct!(=) >>
            s: syn!(syn::Type) >>
            (s)
        )=> { BindgenAttr::Static }
        |
        do_parse!(
            call!(term, "module") >>
            punct!(=) >>
            s: syn!(syn::LitStr) >>
            (s.value())
        )=> { BindgenAttr::Module }
    ));
}

fn extract_first_ty_param(ty: Option<&Type>) -> Option<Option<Type>> {
    let ty = match ty {
        Some(t) => t,
        None => return Some(None)
    };
    let ty = match *ty {
        Type::ByValue(ref t) => t,
        _ => return None,
    };
    let path = match *ty {
        syn::Type::Path(syn::TypePath { qself: None, ref path }) => path,
        _ => return None,
    };
    let seg = path.segments.last()?.into_value();
    let generics = match seg.arguments {
        syn::PathArguments::AngleBracketed(ref t) => t,
        _ => return None,
    };
    let ty = match *generics.args.first()?.into_value() {
        syn::GenericArgument::Type(ref t) => t,
        _ => return None,
    };
    match *ty {
        syn::Type::Tuple(ref t) if t.elems.len() == 0 => return Some(None),
        _ => {}
    }
    Some(Some(Type::from(ty)))
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
