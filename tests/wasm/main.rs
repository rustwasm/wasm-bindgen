#![cfg(target_arch = "wasm32")]

extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_test;
extern crate wasm_bindgen_test_crate_a;
extern crate wasm_bindgen_test_crate_b;

#[cfg(feature = "serde-serialize")]
#[macro_use]
extern crate serde_derive;

pub mod api;
pub mod char;
pub mod classes;
pub mod closures;
pub mod comments;
pub mod duplicate_deps;
pub mod duplicates;
pub mod enums;
pub mod host_binding;
pub mod import_class;
pub mod imports;
pub mod js_objects;
pub mod jscast;
pub mod math;
pub mod node;
pub mod option;
pub mod optional_primitives;
pub mod rethrow;
pub mod simple;
pub mod slice;
pub mod structural;
pub mod u64;
pub mod validate_prt;
pub mod variadic;
pub mod vendor_prefix;
