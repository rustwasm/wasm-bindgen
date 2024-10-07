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

#[wasm_bindgen]
#[derive(PartialEq, Debug)]
pub enum ColorName {
    Green = "green",
    Yellow = "yellow",
    Red = "red",
}

#[wasm_bindgen]
pub fn get_name(color: Color) -> ColorName {
    match color {
        Color::Red => ColorName::Red,
        Color::Green => ColorName::Green,
        Color::Yellow => ColorName::Yellow,
    }
}

#[wasm_bindgen]
pub fn option_string_enum_echo(color: Option<ColorName>) -> Option<ColorName> {
    color
}
