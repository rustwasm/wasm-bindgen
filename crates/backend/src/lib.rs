#![recursion_limit = "256"]
#![cfg_attr(feature = "extra-traits", deny(missing_debug_implementations))]

extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate serde_json;

extern crate wasm_bindgen_shared as shared;

pub mod ast;
mod codegen;
pub mod defined;
pub mod util;
