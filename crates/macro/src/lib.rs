#![doc(html_root_url = "https://docs.rs/wasm-bindgen-macro/0.2")]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;

#[proc_macro_attribute]
pub fn wasm_bindgen(attr: TokenStream, input: TokenStream) -> TokenStream {
    match wasm_bindgen_macro_support::expand(attr.into(), input.into()) {
        Ok(tokens) => {
            if cfg!(feature = "xxx_debug_only_print_generated_code") {
                println!("{}", tokens);
            }
            // merely returning `tokens.into()` loses span info in some situations
            // whereas extending a new TokenStream with it does not !?
            // possibly related to https://github.com/rust-lang/rust/issues/43081 ?
            let mut result = TokenStream::new();
            result.extend::<TokenStream>(tokens.into());
            result
        }
        Err(diagnostic) => (quote! { #diagnostic }).into(),
    }
}

#[proc_macro_attribute]
pub fn __wasm_bindgen_class_marker(attr: TokenStream, input: TokenStream) -> TokenStream {
    match wasm_bindgen_macro_support::expand_class_marker(attr.into(), input.into()) {
        Ok(tokens) => {
            if cfg!(feature = "xxx_debug_only_print_generated_code") {
                println!("{}", tokens);
            }
            tokens.into()
        }
        Err(diagnostic) => (quote! { #diagnostic }).into(),
    }
}

// proc_macro_hack used here because this macro is unhygienic in order to
// transparently access the callback argument injected into constructors
#[proc_macro_hack]
pub fn instantiate(input: TokenStream) -> TokenStream {
    match wasm_bindgen_macro_support::expand_instantiate(input.into()) {
        Ok(tokens) => {
            if cfg!(feature = "xxx_debug_only_print_generated_code") {
                println!("{}", tokens);
            }
            tokens.into()
        }
        Err(diagnostic) => (quote! { #diagnostic }).into(),
    }
}