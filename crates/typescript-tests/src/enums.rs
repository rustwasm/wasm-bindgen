use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Shape {
    Circle, Rectangle, Triangle
}

#[wasm_bindgen]
pub fn accepts_enum(_: Shape) {}

#[wasm_bindgen]
pub fn take_and_return_enum(s: Shape) -> Shape {
    s
}


#[wasm_bindgen]
pub fn accepts_option_enum(_: Option<Shape>) {}

#[wasm_bindgen]
pub fn take_and_return_option_enum(s: Option<Shape>) -> Option<Shape> {
    s
}
