use super::project;

#[test]
fn c_style_enum() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub enum Color {
                    Green,
                    Yellow,
                    Red,
                }

                #[wasm_bindgen]
                pub fn cycle(color: Color) -> Color {
                    match color {
                        Color::Green => Color::Yellow,
                        Color::Yellow => Color::Red,
                        Color::Red => Color::Green,
                    }
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    assert.strictEqual(wasm.Color.Green, 0);
                    assert.strictEqual(wasm.Color.Yellow, 1);
                    assert.strictEqual(wasm.Color.Red, 2);
                    assert.strictEqual(Object.keys(wasm.Color).length, 3);

                    assert.strictEqual(wasm.cycle(wasm.Color.Green), wasm.Color.Yellow);
                }
            "#,
        )
        .test();
}

#[test]
fn c_style_enum_with_custom_values() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                pub mod inner {
                    use wasm_bindgen::prelude::*;

                    #[wasm_bindgen]
                    pub enum Color {
                        Green = 21,
                        Yellow = 34,
                        Red,
                    }
                }

                use inner::Color;

                #[wasm_bindgen]
                pub fn cycle(color: Color) -> Color {
                    match color {
                        Color::Green => Color::Yellow,
                        Color::Yellow => Color::Red,
                        Color::Red => Color::Green,
                    }
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    assert.strictEqual(wasm.Color.Green, 21);
                    assert.strictEqual(wasm.Color.Yellow, 34);
                    assert.strictEqual(wasm.Color.Red, 2);
                    assert.strictEqual(Object.keys(wasm.Color).length, 3);

                    assert.strictEqual(wasm.cycle(wasm.Color.Green), wasm.Color.Yellow);
                }
            "#,
        )
        .test();
}
