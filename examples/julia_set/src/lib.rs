#![feature(use_extern_macros, wasm_import_module, js_globals)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod fractal;
use fractal::get_julia_set;
use fractal::complex::Complex;

#[wasm_bindgen]
extern "C" {
    pub type ImageData;

    #[wasm_bindgen(constructor)]
    pub fn new(arr: &Uint8ClampedArray, width: u32, height: u32) -> ImageData;
}

#[wasm_bindgen]
extern "C" {
    pub type Uint8ClampedArray;

    #[wasm_bindgen(constructor)]
    pub fn new(arr: &[u8]) -> Uint8ClampedArray;
}

#[wasm_bindgen]
extern "C" {
    pub type CanvasRenderingContext2D;

    #[wasm_bindgen(method, js_name = putImageData)]
    pub fn put_image_data(this: &CanvasRenderingContext2D, image_data: &ImageData, p_1: i32, p_2: i32);
}

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2D, width: u32, height: u32, real: f64, imaginary: f64) {
    let c = Complex { real, imaginary };
    let data = get_julia_set(width, height, c);
    let uint8_array = Uint8ClampedArray::new(&data);

    ctx.put_image_data(&ImageData::new(&uint8_array, width, height), 0, 0);
}
