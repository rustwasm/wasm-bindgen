use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "/globals.js")]
extern "C" {
    fn noop();
}

#[wasm_bindgen]
pub fn hello_there() {}

// This is to ensure that the file is actually loaded
#[wasm_bindgen_test]
fn keep() {
    noop();
}

mod generated {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

pub mod array;
pub mod array_buffer;
pub mod callbacks;
pub mod consts;
pub mod dictionary;
pub mod enums;
pub mod global;
pub mod namespace;
pub mod no_interface;
pub mod promise;
pub mod simple;
pub mod throws;
pub mod unstable;
