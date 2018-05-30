extern crate diff;
extern crate proc_macro2;
extern crate syn;
extern crate wasm_bindgen_backend as backend;
extern crate wasm_bindgen_webidl as wb_webidl;

#[macro_use]
mod util;
use util::*;

assert_compile!(Event);
