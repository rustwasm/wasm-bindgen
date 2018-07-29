#![doc(html_root_url = "https://docs.rs/wasm-bindgen-macro/0.2")]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
extern crate wasm_bindgen_macro_support as macro_support;

use macro_support::BindgenAttrs;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn wasm_bindgen(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = syn::parse::<syn::Item>(input.clone()).expect("expected a valid Rust item");
    let opts = syn::parse::<BindgenAttrs>(attr).expect("invalid arguments to #[wasm_bindgen]");

    let tokens = macro_support::expand(item, opts);

    if cfg!(feature = "xxx_debug_only_print_generated_code") {
        println!("{}", tokens);
    }

    tokens.into()
}
