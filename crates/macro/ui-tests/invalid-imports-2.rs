use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn f() -> &'static u32;
}

fn main() {}
