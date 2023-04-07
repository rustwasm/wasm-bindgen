#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen;
extern crate wasm_bindgen_test;
extern crate js_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::{wasm_bindgen_test_configure, wasm_bindgen_test};

wasm_bindgen_test_configure!(run_in_worker);

pub mod memory;
pub mod modules;

// should not be executed
#[wasm_bindgen(start)]
fn start() {
    panic!();
}
