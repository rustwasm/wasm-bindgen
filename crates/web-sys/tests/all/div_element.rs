use super::websys_project;

#[test]
fn div_element() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_custom_section)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_div_element(element: &web_sys::HtmlDivElement) {
                    assert_eq!(element.align(), "", "Shouldn't have a align");
                    element.set_align("right");
                    assert_eq!(element.align(), "right", "Should have a align");
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let div = document.createElement("div");
                    wasm.test_div_element(div);
                }
            "#,
        )
        .test();
}
