#![allow(non_snake_case)]

use project;

#[test]
fn get_canonical_locales() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_canonical_locales(v: &JsValue) -> js::Array {
                js::Intl::get_canonical_locales(v)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let locales = ["EN-US", "Fr"];
                let canonical_locales = wasm.get_canonical_locales(locales);
                assert.deepStrictEqual(canonical_locales, ["en-US", "fr"]);

                let single_locale = wasm.get_canonical_locales("EN-US");
                assert.equal(single_locale, "en-US");
            }
        "#)
        .test()
}