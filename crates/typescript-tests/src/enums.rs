use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Shape {
    Circle, Rectangle, Triangle
}

#[wasm_bindgen]
pub fn accepts_enum(_: Shape) {}

#[wasm_bindgen]
pub fn take_and_return_shape(s: Shape) -> Shape {
    s
}
