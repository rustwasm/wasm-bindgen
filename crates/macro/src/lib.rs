#![feature(proc_macro)]

extern crate syn;
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
extern crate serde_json;
extern crate wasm_bindgen_backend as backend;
extern crate wasm_bindgen_shared as shared;

use proc_macro::TokenStream;
use quote::{ToTokens, Tokens};

#[proc_macro_attribute]
pub fn wasm_bindgen(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = syn::parse::<syn::Item>(input.clone()).expect("expected a valid Rust item");
    let opts = syn::parse::<backend::ast::BindgenAttrs>(attr)
        .expect("invalid arguments to #[wasm_bindgen]");

    let mut ret = Tokens::new();
    let mut program = backend::ast::Program::default();
    program.push_item(item, Some(opts), &mut ret);
    program.to_tokens(&mut ret);

    // println!("{}", ret);

    ret.into()
}
