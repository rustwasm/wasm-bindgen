use super::websys_project;

#[test]
fn body_element() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_custom_section)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_body_element(element: &web_sys::HtmlBodyElement) {
                    assert_eq!(element.text(), "", "Shouldn't have a text");
                    element.set_text("boop");
                    assert_eq!(element.text(), "boop", "Should have a text");

                    // Legacy color setting
                    assert_eq!(element.link(), "", "Shouldn't have a link");
                    element.set_link("blue");
                    assert_eq!(element.link(), "blue", "Should have a link");

                    assert_eq!(element.v_link(), "", "Shouldn't have a v_link");
                    element.set_v_link("purple");
                    assert_eq!(element.v_link(), "purple", "Should have a v_link");

                    assert_eq!(element.a_link(), "", "Shouldn't have a a_link");
                    element.set_a_link("purple");
                    assert_eq!(element.a_link(), "purple", "Should have a a_link");

                    assert_eq!(element.bg_color(), "", "Shouldn't have a bg_color");
                    element.set_bg_color("yellow");
                    assert_eq!(element.bg_color(), "yellow", "Should have a bg_color");

                    assert_eq!(element.background(), "", "Shouldn't have a background");
                    element.set_background("image");
                    assert_eq!(element.background(), "image", "Should have a background");
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let body = document.createElement("body");
                    wasm.test_body_element(body);
                }
            "#,
        )
        .test();
}
