extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2D, ImageData};

mod fractal;
use fractal::get_julia_set;
use fractal::complex::Complex;

#[wasm_bindgen]
extern "C" {
    pub type Uint8ClampedArray;

    #[wasm_bindgen(constructor)]
    pub fn new(arr: &[u8]) -> Uint8ClampedArray;
}

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2D, width: u32, height: u32, real: f64, imaginary: f64) {
    let c = Complex { real, imaginary };
    let data = get_julia_set(width, height, c);
    let uint8_array = Uint8ClampedArray::new(&data);

    ctx.put_image_data(
        &ImageData::with_data_and_sw_and_sh_with_sh(&data, width, height).unwrap(),
        0.0, 0.0);
}
