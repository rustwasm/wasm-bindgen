//! This crate contains the part of the implementation of the `#[wasm_bindgen]` optsibute that is
//! not in the shared backend crate.

#![doc(html_root_url = "https://docs.rs/wasm-bindgen-macro-support/0.2")]

extern crate proc_macro2;
extern crate quote;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate wasm_bindgen_backend as backend;
extern crate wasm_bindgen_shared as shared;

use backend::{Diagnostic, TryToTokens};
pub use parser::BindgenAttrs;
use parser::MacroParse;
use proc_macro2::TokenStream;

mod parser;

/// Takes the parsed input from a `#[wasm_bindgen]` macro and returns the generated bindings
pub fn expand(attr: TokenStream, input: TokenStream) -> Result<TokenStream, Diagnostic> {
    let item = syn::parse2::<syn::Item>(input)?;
    let opts = syn::parse2(attr)?;

    let mut tokens = proc_macro2::TokenStream::new();
    let mut program = backend::ast::Program::default();
    item.macro_parse(&mut program, (Some(opts), &mut tokens))?;
    program.try_to_tokens(&mut tokens)?;
    Ok(tokens)
}
