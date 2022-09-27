#![doc(html_root_url = "https://docs.rs/wasm-bindgen-macro/0.2")]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn wasm_bindgen(attr: TokenStream, input: TokenStream) -> TokenStream {
    match wasm_bindgen_macro_support::expand(attr.into(), input.into()) {
        Ok(tokens) => {
            if cfg!(feature = "xxx_debug_only_print_generated_code") {
                println!("{}", tokens);
            }
            tokens.into()
        }
        Err(diagnostic) => (quote! { #diagnostic }).into(),
    }
}

/// This macro adds a linked module by module, raw_module or inline_js attribute.
/// It expands to a String containing a URL to that module. This URL can be used
/// to create workers or worklets, for example:
/// ```no_run
/// use web_sys::Worker;
/// let worker = Worker::new(&wasm_bindgen::link_to!(module = "/src/worker.js"));
/// ```
#[proc_macro]
pub fn link_to(input: TokenStream) -> TokenStream {
    match wasm_bindgen_macro_support::expand_link_to(input.into()) {
        Ok(tokens) => {
            if cfg!(feature = "xxx_debug_only_print_generated_code") {
                println!("{}", tokens);
            }
            tokens.into()
        }
        Err(diagnostic) => (quote! { String::clone(#diagnostic) }).into(),
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
