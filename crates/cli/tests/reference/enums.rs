use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(PartialEq, Debug)]
pub enum Color {
    Green,
    Yellow,
    Red,
}

#[wasm_bindgen]
pub fn enum_echo(color: Color) -> Color {
    color
}

#[wasm_bindgen]
pub fn option_enum_echo(color: Option<Color>) -> Option<Color> {
    color
}
