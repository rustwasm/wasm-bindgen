#![cfg(target_arch = "wasm32")]
#![feature(use_extern_macros)]

extern crate wasm_bindgen_test;
extern crate wasm_bindgen;

pub mod api;
pub mod char;
pub mod classes;
pub mod closures;
pub mod duplicates;
pub mod enums;
pub mod imports;
pub mod import_class;
pub mod js_objects;
pub mod math;
pub mod option;
pub mod optional_primitives;
pub mod simple;
pub mod slice;
pub mod structural;
pub mod u64;
pub mod validate_prt;
