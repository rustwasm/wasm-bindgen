#![cfg(target_arch = "wasm32")]
#![feature(use_extern_macros, wasm_import_module)]
#![allow(non_snake_case)]

extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_test;

pub mod Array;
pub mod ArrayBuffer;
pub mod ArrayIterator;
pub mod Boolean;
pub mod DataView;
pub mod Date;
pub mod Error;
pub mod Function;
pub mod Generator;
