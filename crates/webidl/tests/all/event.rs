use super::backend;
use proc_macro2;
use syn;

assert_parse!(
    event,
    include_str!("./Event.webidl"),
    backend::ast::Program {
        exports: vec![],
        imports: vec![backend::ast::Import {
            module: None,
            version: None,
            js_namespace: None,
            kind: backend::ast::ImportKind::Type(backend::ast::ImportType {
                vis: syn::Visibility::Public(syn::VisPublic {
                    pub_token: syn::token::Pub(proc_macro2::Span::call_site()),
                }),
                name: syn::Ident::new("Event", proc_macro2::Span::call_site()),
            }),
        }],
        enums: vec![],
        structs: vec![],
    }
);
