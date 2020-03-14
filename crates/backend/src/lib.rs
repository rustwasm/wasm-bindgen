#![recursion_limit = "256"]
#![cfg_attr(feature = "extra-traits", deny(missing_debug_implementations))]
#![doc(html_root_url = "https://docs.rs/wasm-bindgen-backend/0.2")]

pub use crate::codegen::TryToTokens;
pub use crate::error::Diagnostic;

#[macro_use]
mod error;

pub mod ast;
mod codegen;
mod encode;
pub mod util;
