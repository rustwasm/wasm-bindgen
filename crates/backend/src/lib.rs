#![recursion_limit = "256"]

extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate serde_json;

extern crate wasm_bindgen_shared as shared;

pub mod ast;
mod codegen;
