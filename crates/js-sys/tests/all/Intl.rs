#![allow(non_snake_case)]

use project;

#[test]
fn get_canonical_locales() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn get_canonical_locales(v: &JsValue) -> js_sys::Array {
                js_sys::Intl::get_canonical_locales(v)
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