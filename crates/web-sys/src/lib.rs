#![feature(wasm_custom_section)]

extern crate wasm_bindgen;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
