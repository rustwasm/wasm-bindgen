use proc_macro2::Span;
use syn;
use wasm_bindgen_shared as shared;

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
    pub functions: Vec<(bool, ImportFunction)>,
}

pub enum Type {
    Integer(syn::Ident),
    BorrowedStr,
    String,
    ByValue(syn::Ident),
    ByRef(syn::Ident),
    ByMutRef(syn::Ident),
    RawMutPtr(syn::Ident),
    RawConstPtr(syn::Ident),
    JsObject,
    JsObjectRef,
    Boolean,
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
        let name = match Type::from(&item.self_ty) {
            Type::ByValue(ident) => ident,
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
                (method, f)
            })
            .collect();
        self.imported_structs.push(ImportStruct {
            module: class.module.as_ref().map(|s| s.value()),
            name: class.name,
            functions,
        });
    }

    pub fn shared(&self) -> shared::Program {
        shared::Program {
            structs: self.structs.iter().map(|s| s.shared()).collect(),
            free_functions: self.free_functions.iter().map(|s| s.shared()).collect(),
            imports: self.imports.iter()
                .map(|i| (i.module.clone(), i.function.wasm_function.shared()))
                .collect(),
            imported_structs: self.imported_structs.iter()
                .map(|i| i.shared())
                .collect(),
        }
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
        let name = self.shared().free_function_export_name();
        syn::LitStr::new(&name, Span::def_site())
    }

    pub fn struct_function_export_name(&self, s: syn::Ident) -> syn::LitStr {
        let name = self.shared().struct_function_export_name(s.as_ref());
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

    pub fn shared(&self) -> shared::Function {
        shared::Function {
            name: self.name.as_ref().to_string(),
            arguments: self.arguments.iter().map(|t| t.shared()).collect(),
            ret: self.ret.as_ref().map(|t| t.shared()),
        }
    }
}

pub fn extract_path_ident(path: &syn::Path) -> syn::Ident {
    if path.leading_colon.is_some() {
        panic!("unsupported leading colon in path")
    }
    if path.segments.len() != 1 {
        panic!("unsupported path that needs name resolution")
    }
    match path.segments.first().unwrap().value().arguments {
        syn::PathArguments::None => {}
        _ => panic!("unsupported path that has path arguments")
    }
    path.segments.first().unwrap().value().ident
}

impl Type {
    pub fn from(ty: &syn::Type) -> Type {
        match *ty {
            syn::Type::Reference(ref r) => {
                if r.lifetime.is_some() {
                    panic!("can't have lifetimes on references yet");
                }
                let mutable = r.mutability.is_some();
                match *r.elem {
                    syn::Type::Path(syn::TypePath { qself: None, ref path }) => {
                        let ident = extract_path_ident(path);
                        match ident.as_ref() {
                            "str" => {
                                if mutable {
                                    panic!("mutable strings not allowed");
                                }
                                Type::BorrowedStr
                            }
                            "JsObject" if !mutable => Type::JsObjectRef,
                            "JsObject" if mutable => {
                                panic!("can't have mutable js object refs")
                            }
                            _ if mutable => Type::ByMutRef(ident),
                            _ => Type::ByRef(ident),
                        }
                    }
                    _ => panic!("unsupported reference type"),
                }
            }
            syn::Type::Ptr(ref p) => {
                let mutable = p.const_token.is_none();
                let ident = match *p.elem {
                    syn::Type::Path(syn::TypePath { qself: None, ref path }) => {
                        extract_path_ident(path)
                    }
                    _ => panic!("unsupported reference type"),
                };
                if mutable {
                    Type::RawMutPtr(ident)
                } else {
                    Type::RawConstPtr(ident)
                }
            }
            syn::Type::Path(syn::TypePath { qself: None, ref path }) => {
                let ident = extract_path_ident(path);
                match ident.as_ref() {
                    "i8" |
                    "u8" |
                    "u16" |
                    "i16" |
                    "u32" |
                    "i32" |
                    "isize" |
                    "usize" |
                    "f32" |
                    "f64" => {
                        Type::Integer(ident)
                    }
                    "bool" => Type::Boolean,
                    "String" => Type::String,
                    "JsObject" => Type::JsObject,
                    _ => Type::ByValue(ident),
                }
            }
            _ => panic!("unsupported type"),
        }
    }

    fn shared(&self) -> shared::Type {
        match *self {
            Type::Integer(_) |
            Type::RawConstPtr(_) |
            Type::RawMutPtr(_) => shared::Type::Number,
            Type::BorrowedStr => shared::Type::BorrowedStr,
            Type::String => shared::Type::String,
            Type::ByValue(n) => shared::Type::ByValue(n.to_string()),
            Type::ByRef(n) => shared::Type::ByRef(n.to_string()),
            Type::ByMutRef(n) => shared::Type::ByMutRef(n.to_string()),
            Type::JsObject => shared::Type::JsObject,
            Type::JsObjectRef => shared::Type::JsObjectRef,
            Type::Boolean => shared::Type::Boolean,
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
        syn::Ident::from(self.shared().free_function())
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

    pub fn shared(&self) -> shared::Struct {
        shared::Struct {
            name: self.name.to_string(),
            functions: self.functions.iter().map(|f| f.shared()).collect(),
            methods: self.methods.iter().map(|f| f.shared()).collect(),
        }
    }
}

impl Method {
    pub fn shared(&self) -> shared::Method {
        shared::Method {
            mutable: self.mutable,
            function: self.function.shared(),
        }
    }
}

impl ImportStruct {
    fn shared(&self) -> shared::ImportStruct {
        shared::ImportStruct {
            module: self.module.clone(),
            name: self.name.to_string(),
            functions: self.functions.iter()
                .map(|&(b, ref f)| (b, f.wasm_function.shared()))
                .collect(),
        }
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
