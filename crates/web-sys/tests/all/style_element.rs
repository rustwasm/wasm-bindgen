use super::websys_project;

#[test]
fn style_element() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_custom_section)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_style_element(element: &web_sys::HtmlStyleElement) {
                    assert!(!element.disabled(), "Should be disabled");
                    element.set_disabled(true);
                    assert!(!element.disabled(), "Should be disabled"); // Not sure why this is but Chrome in Firefox behabe the same

                    assert_eq!(element.type_(), "", "Shouldn't have a type");
                    element.set_type("text/css");
                    assert_eq!(element.type_(), "text/css", "Should have a type");

                    assert_eq!(element.media(), "", "Shouldn't have a media");
                    element.set_media("screen, print");
                    assert_eq!(element.media(), "screen, print", "Should have a media");
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let style = document.createElement("style");
                    wasm.test_style_element(style);
                }
            "#,
        )
        .test();
}
