#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;
extern crate wasm_bindgen_backend as backend;

use proc_macro::TokenStream;
use quote::ToTokens;

#[proc_macro_attribute]
pub fn wasm_bindgen(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = syn::parse::<syn::Item>(input.clone()).expect("expected a valid Rust item");
    let context = backend::ast::BindgenAttrContext::from_item_toplevel(&item);
    let opts = backend::ast::BindgenAttrs::parse(attr.into(), context);

    let mut ret = proc_macro2::TokenStream::new();
    let mut program = backend::ast::Program::default();
    program.push_item(item, Some(opts), &mut ret);
    program.to_tokens(&mut ret);

    if cfg!(feature = "xxx_debug_only_print_generated_code") {
        println!("{}", ret);
    }

    ret.into()
}
