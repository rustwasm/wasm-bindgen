#![feature(wasm_import_module)]

extern crate wasm_bindgen;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
