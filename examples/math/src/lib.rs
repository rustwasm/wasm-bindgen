#![feature(proc_macro)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(namespace = Math)]
    fn log2(a: f64) -> f64;
    #[wasm_bindgen(namespace = Math)]
    fn sin(a: f64) -> f64;

    #[wasm_bindgen(namespace = console)]
    fn log(a: &str);
}

macro_rules! println {
    ($($t:tt)*) => (console::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn run() {
    println!("Math.log2(10.0) = {}", Math::log2(10.0));
    println!("Math.sin(1.2) = {}", Math::sin(1.2));
}
