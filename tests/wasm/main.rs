#![cfg(target_arch = "wasm32")]
#![feature(use_extern_macros)]

extern crate wasm_bindgen_test;
extern crate wasm_bindgen;

pub mod api;
pub mod char;
pub mod classes;
pub mod option;
pub mod optional_primitives;
