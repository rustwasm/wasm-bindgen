extern crate diff;
extern crate proc_macro2;
extern crate syn;
extern crate wasm_bindgen_backend as backend;
extern crate wasm_bindgen_webidl as wb_webidl;

#[macro_use]
mod util;
use util::*;

/// Tests for parsing WebIDL into an expected wasm-bindgen AST.
mod parse {
    use super::*;

    assert_parse!(empty, backend::ast::Program::default());

    assert_parse!(
        Event,
        backend::ast::Program {
            exports: vec![],
            imports: vec![backend::ast::Import {
                module: None,
                version: None,
                js_namespace: None,
                kind: backend::ast::ImportKind::Type(backend::ast::ImportType {
                    vis: syn::Visibility::Public(syn::VisPublic {
                        pub_token: Default::default(),
                    }),
                    name: syn::Ident::new("Event", proc_macro2::Span::call_site()),
                }),
            }],
            enums: vec![],
            structs: vec![],
        }
    );
}

/// Tests for compiling WebIDL into Rust bindings.
mod compile {
    assert_compile!(Event);
}
