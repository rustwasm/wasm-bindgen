use super::websys_project;

#[test]
fn br_element() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_custom_section)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_br_element(element: &web_sys::HtmlBrElement) {
                    // Legacy clear method
                    assert_eq!(element.clear(), "", "Shouldn't have a clear");
                    element.set_clear("boop");
                    assert_eq!(element.clear(), "boop", "Should have a clear");
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let br = document.createElement("br");
                    wasm.test_br_element(br);
                }
            "#,
        )
        .test();
}
