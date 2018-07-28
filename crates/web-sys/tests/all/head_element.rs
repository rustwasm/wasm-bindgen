use super::websys_project;

#[test]
fn head_element() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_custom_section)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_head_element(_element: &web_sys::HtmlHeadElement) {
                    assert!(true, "Head doesn't have an interface");
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let head = document.createElement("head");
                    wasm.test_head_element(head);
                }
            "#,
        )
        .test();
}
