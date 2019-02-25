extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "./foo.js")]
extern {
    fn wut();
}

#[wasm_bindgen(module = "../foo.js")]
extern {
    fn wut();
}

fn main() {}
