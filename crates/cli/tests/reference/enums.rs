use wasm_bindgen::prelude::*;

/// A color.
#[wasm_bindgen]
#[derive(PartialEq, Debug)]
pub enum Color {
    /// Green as a leaf.
    Green,
    /// Yellow as the sun.
    Yellow,
    /// Red as a rose.
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

/// The name of a color.
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

/// An unused string enum.
#[wasm_bindgen(js_name = "FooBar")]
pub enum UnusedStringEnum {
    Foo = "foo",
    Bar = "bar",
}

#[wasm_bindgen]
enum PrivateStringEnum {
    Foo = "foo",
    Bar = "bar",
}

#[wasm_bindgen]
pub enum ImplicitDiscriminant {
    A,
    B,
    C = 42,
    D,
}

/// A C-style enum with negative discriminants.
#[wasm_bindgen]
pub enum Ordering {
    Less = -1,
    Equal = 0,
    Greater = 1,
}

#[wasm_bindgen]
pub fn option_order(order: Option<Ordering>) -> Option<Ordering> {
    order
}
