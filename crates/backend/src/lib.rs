#![recursion_limit = "256"]

extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate wasm_bindgen_shared as shared;

macro_rules! my_quote {
    ($($t:tt)*) => (quote_spanned!(Span::call_site() => $($t)*))
}

pub mod ast;
mod codegen;
mod literal;
