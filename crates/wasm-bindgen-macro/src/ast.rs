use proc_macro2::Literal;
use syn;
use wasm_bindgen_shared as shared;

pub struct Program {
    pub structs: Vec<Struct>,
    pub free_functions: Vec<Function>,
}

pub struct Function {
    pub name: syn::Ident,
    pub arguments: Vec<Type>,
    pub ret: Option<Type>,
}

pub enum Type {
    Integer(syn::Ident),
    BorrowedStr,
    String,
    ByValue(syn::Ident),
    ByRef(syn::Ident),
    ByMutRef(syn::Ident),
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
        match item.defaultness {
            syn::Defaultness::Final => {}
            _ => panic!("default impls are not supported"),
        }
        match item.unsafety {
            syn::Unsafety::Normal => {}
            _ => panic!("unsafe impls are not supported"),
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

    pub fn shared(&self) -> shared::Program {
        shared::Program {
            structs: self.structs.iter().map(|s| s.shared()).collect(),
            free_functions: self.free_functions.iter().map(|s| s.shared()).collect(),
        }
    }
}

impl Function {
    pub fn from(input: &syn::ItemFn) -> Function {
        match input.vis {
            syn::Visibility::Public(_) => {}
            _ => panic!("can only bindgen public functions"),
        }
        match input.constness {
            syn::Constness::NotConst => {}
            _ => panic!("can only bindgen non-const functions"),
        }
        match input.unsafety {
            syn::Unsafety::Normal => {}
            _ => panic!("can only bindgen safe functions"),
        }
        if !input.abi.is_none() {
            panic!("can only bindgen Rust ABI functions")
        }
        if !input.abi.is_none() {
            panic!("can only bindgen Rust ABI functions")
        }

        if input.decl.variadic {
            panic!("can't bindgen variadic functions")
        }
        if input.decl.generics.params.len() > 0 {
            panic!("can't bindgen functions with lifetime or type parameters")
        }

        let arguments = input.decl.inputs.iter()
            .map(|i| i.into_item())
            .map(|arg| {
                match *arg {
                    syn::FnArg::Captured(ref c) => c,
                    _ => panic!("arguments cannot be `self` or ignored"),
                }
            })
            .map(|arg| Type::from(&arg.ty))
            .collect::<Vec<_>>();

        let ret = match input.decl.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(ref t, _) => Some(Type::from(t)),
        };

        Function { name: input.ident, arguments, ret }
    }

    pub fn free_function_export_name(&self) -> syn::Lit {
        let name = self.shared().free_function_export_name();
        syn::Lit {
            value: syn::LitKind::Other(Literal::string(&name)),
            span: Default::default(),
        }
    }

    pub fn struct_function_export_name(&self, s: syn::Ident) -> syn::Lit {
        let name = self.shared().struct_function_export_name(s.sym.as_str());
        syn::Lit {
            value: syn::LitKind::Other(Literal::string(&name)),
            span: Default::default(),
        }
    }

    pub fn rust_symbol(&self, namespace: Option<syn::Ident>) -> syn::Ident {
        let mut generated_name = format!("__wasm_bindgen_generated");
        if let Some(ns) = namespace {
            generated_name.push_str("_");
            generated_name.push_str(ns.sym.as_str());
        }
        generated_name.push_str("_");
        generated_name.push_str(self.name.sym.as_str());
        syn::Ident::from(generated_name)
    }

    fn shared(&self) -> shared::Function {
        shared::Function {
            name: self.name.sym.as_str().to_string(),
            arguments: self.arguments.iter().map(|t| t.shared()).collect(),
            ret: self.ret.as_ref().map(|t| t.shared()),
        }
    }
}

impl Type {
    pub fn from(ty: &syn::Type) -> Type {
        let extract_path_ident = |path: &syn::Path| {
            if path.leading_colon.is_some() {
                panic!("unsupported leading colon in path")
            }
            if path.segments.len() != 1 {
                panic!("unsupported path that needs name resolution")
            }
            match path.segments.get(0).item().arguments {
                syn::PathArguments::None => {}
                _ => panic!("unsupported path that has path arguments")
            }
            path.segments.get(0).item().ident
        };
        match *ty {
            syn::Type::Reference(ref r) => {
                if r.lifetime.is_some() {
                    panic!("can't have lifetimes on references yet");
                }
                let mutable = match r.ty.mutability {
                    syn::Mutability::Immutable => false,
                    syn::Mutability::Mutable(_) => true,
                };
                match r.ty.ty {
                    syn::Type::Path(syn::TypePath { qself: None, ref path }) => {
                        let ident = extract_path_ident(path);
                        match ident.sym.as_str() {
                            "str" => {
                                if mutable {
                                    panic!("mutable strings not allowed");
                                }
                                Type::BorrowedStr
                            }
                            _ if mutable => Type::ByMutRef(ident),
                            _ => Type::ByRef(ident),
                        }
                    }
                    _ => panic!("unsupported reference type"),
                }
            }
            syn::Type::Path(syn::TypePath { qself: None, ref path }) => {
                let ident = extract_path_ident(path);
                match ident.sym.as_str() {
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
                    "String" => Type::String,
                    _ => Type::ByValue(ident),
                }
            }
            _ => panic!("unsupported type"),
        }
    }

    fn shared(&self) -> shared::Type {
        match *self {
            Type::Integer(_) => shared::Type::Number,
            Type::BorrowedStr => shared::Type::BorrowedStr,
            Type::String => shared::Type::String,
            Type::ByValue(n) => shared::Type::ByValue(n.to_string()),
            Type::ByRef(n) => shared::Type::ByRef(n.to_string()),
            Type::ByMutRef(n) => shared::Type::ByMutRef(n.to_string()),
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
        };
        match method.vis {
            syn::Visibility::Public(_) => {}
            _ => return,
        }
        match method.defaultness {
            syn::Defaultness::Final => {}
            _ => panic!("default methods are not supported"),
        }
        match method.sig.constness {
            syn::Constness::NotConst => {}
            _ => panic!("can only bindgen non-const functions"),
        }
        match method.sig.unsafety {
            syn::Unsafety::Normal => {}
            _ => panic!("can only bindgen safe functions"),
        }

        if method.sig.decl.variadic {
            panic!("can't bindgen variadic functions")
        }
        if method.sig.decl.generics.params.len() > 0 {
            panic!("can't bindgen functions with lifetime or type parameters")
        }

        let mut mutable = None;
        let arguments = method.sig.decl.inputs.iter()
            .map(|i| i.into_item())
            .filter_map(|arg| {
                match *arg {
                    syn::FnArg::Captured(ref c) => Some(c),
                    syn::FnArg::SelfValue(_) => {
                        panic!("by-value `self` not yet supported");
                    }
                    syn::FnArg::SelfRef(ref a) => {
                        assert!(mutable.is_none());
                        mutable = Some(match a.mutbl {
                            syn::Mutability::Mutable(_) => true,
                            syn::Mutability::Immutable => false,
                        });
                        None
                    }
                    _ => panic!("arguments cannot be `self` or ignored"),
                }
            })
            .map(|arg| Type::from(&arg.ty))
            .collect::<Vec<_>>();

        let ret = match method.sig.decl.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(ref t, _) => Some(Type::from(t)),
        };

        let function = Function { name: method.sig.ident, arguments, ret };
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
