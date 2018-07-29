//! This crate contains the part of the implementation of the `#[wasm_bindgen]` optsibute that is
//! not in the shared backend crate.

#![doc(html_root_url = "https://docs.rs/wasm-bindgen-macro-support/0.2")]

extern crate proc_macro2;
extern crate quote;
#[macro_use]
extern crate syn;
extern crate wasm_bindgen_backend as backend;
extern crate wasm_bindgen_shared as shared;

pub use parser::BindgenAttrs;
use parser::MacroParse;
use quote::ToTokens;

mod parser;

/// Takes the parsed input from a `#[wasm_bindgen]` macro and returns the generated bindings
pub fn expand(item: syn::Item, opts: parser::BindgenAttrs) -> proc_macro2::TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();
    let mut program = backend::ast::Program::default();
    item.macro_parse(&mut program, (Some(opts), &mut tokens));
    program.to_tokens(&mut tokens);
    tokens
}
