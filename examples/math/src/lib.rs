#![feature(use_extern_macros, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn log2(a: f64) -> f64;
    #[wasm_bindgen(js_namespace = Math)]
    fn sin(a: f64) -> f64;

    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! println {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn run() {
    println!("Math.log2(10.0) = {}", log2(10.0));
    println!("Math.sin(1.2) = {}", sin(1.2));
}
