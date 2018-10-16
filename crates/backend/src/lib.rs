#![recursion_limit = "256"]
#![cfg_attr(
    feature = "extra-traits",
    deny(missing_debug_implementations)
)]
#![doc(html_root_url = "https://docs.rs/wasm-bindgen-backend/0.2")]

#[macro_use]
extern crate log;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate wasm_bindgen_shared as shared;

pub use codegen::TryToTokens;
pub use error::Diagnostic;

#[macro_use]
mod error;

pub mod ast;
mod codegen;
mod encode;
pub mod defined;
pub mod util;
