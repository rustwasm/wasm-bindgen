use proc_macro2::Literal;
use quote::{ToTokens, Tokens};
use serde_json;
use syn;
use wasm_bindgen_shared as shared;

pub struct Function {
    pub name: syn::Ident,
    pub arguments: Vec<Type>,
    pub ret: Option<Type>,
}

pub enum Type {
    Integer(syn::Ident),
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

        Function {
            name: input.ident,
            arguments,
            ret,
        }
    }

    pub fn export_name(&self) -> syn::Lit {
        syn::Lit {
            value: syn::LitKind::Other(Literal::string(self.name.sym.as_str())),
            span: Default::default(),
        }
    }

    pub fn rust_symbol(&self) -> syn::Ident {
        let generated_name = format!("__wasm_bindgen_generated_{}",
                                     self.name.sym.as_str());
        syn::Ident::from(generated_name)
    }

    pub fn generated_static_name(&self) -> syn::Ident {
        let generated_name = format!("__WASM_BINDGEN_GENERATED_{}",
                                     self.name.sym.as_str());
        syn::Ident::from(generated_name)
    }

    pub fn generate_static(&self) -> Vec<u8> {
        let mut prefix = String::from("wbg:");
        prefix.push_str(&serde_json::to_string(&self.shared()).unwrap());
        prefix.into_bytes()
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
        match *ty {
            // syn::Type::Reference(ref r) => {
            // }
            syn::Type::Path(syn::TypePath { qself: None, ref path }) => {
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
                let ident = path.segments.get(0).item().ident;
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
                    s => panic!("unsupported type: {}", s),
                }
            }
            _ => panic!("unsupported type"),
        }
    }

    fn shared(&self) -> shared::Type {
        match *self {
            Type::Integer(_) => shared::Type::Number,
        }
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self {
            Type::Integer(i) => i.to_tokens(tokens),
        }
    }
}
