use self::inner::ColorWithCustomValues;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/enums.js")]
extern "C" {
    fn js_c_style_enum();
    fn js_c_style_enum_with_custom_values();
}

#[wasm_bindgen]
pub enum Color {
    Green,
    Yellow,
    Red,
}

pub mod inner {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub enum ColorWithCustomValues {
        Green = 21,
        Yellow = 34,
        Red,
    }
}

#[wasm_bindgen]
pub fn enum_cycle(color: Color) -> Color {
    match color {
        Color::Green => Color::Yellow,
        Color::Yellow => Color::Red,
        Color::Red => Color::Green,
    }
}

#[wasm_bindgen]
pub fn enum_with_custom_values_cycle(color: ColorWithCustomValues) -> ColorWithCustomValues {
    match color {
        ColorWithCustomValues::Green => ColorWithCustomValues::Yellow,
        ColorWithCustomValues::Yellow => ColorWithCustomValues::Red,
        ColorWithCustomValues::Red => ColorWithCustomValues::Green,
    }
}

#[wasm_bindgen_test]
fn c_style_enum() {
    js_c_style_enum();
}

#[wasm_bindgen_test]
fn c_style_enum_with_custom_values() {
    js_c_style_enum_with_custom_values();
}
