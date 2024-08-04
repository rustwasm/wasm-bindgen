#![cfg(target_arch = "wasm32")]

extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

pub mod common;
