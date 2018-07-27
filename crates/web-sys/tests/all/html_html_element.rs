use super::websys_project;

#[test]
fn html_html_element() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_custom_section)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_html_html_element(element: &web_sys::HtmlHtmlElement) {
                    assert_eq!(element.version(), "", "Shouldn't have a version");
                    element.set_version("4");
                    assert_eq!(element.version(), "4", "Should have a version");
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let html = document.createElement("html");
                    wasm.test_html_html_element(html);
                }
            "#,
        )
        .test();
}
