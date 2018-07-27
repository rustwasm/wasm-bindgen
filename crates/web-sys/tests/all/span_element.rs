use super::websys_project;

#[test]
fn span_element() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_custom_section)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_span_element(_element: &web_sys::HtmlSpanElement) {
                    assert!(true, "Span doesn't have an interface");
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let span = document.createElement("span");
                    wasm.test_span_element(span);
                }
            "#,
        )
        .test();
}
