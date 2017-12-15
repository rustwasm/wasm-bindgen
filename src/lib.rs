#![feature(use_extern_macros)]

extern crate wasm_bindgen_macro;

pub mod prelude {
    pub use wasm_bindgen_macro::wasm_bindgen;
    pub use Object;
}

pub struct Object {
    idx: u32,
}
