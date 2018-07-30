use super::websys_project;

#[test]
fn xpath_result() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_xpath_result(xpath_result: &web_sys::XPathResult) {
                    assert_eq!(xpath_result.result_type(), web_sys::XPathResult::UNORDERED_NODE_ITERATOR_TYPE);
                    assert_eq!(xpath_result.invalid_iterator_state(), false);
                    assert_eq!(xpath_result.iterate_next().unwrap().unwrap().text_content().unwrap(), "tomato");
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let xmlDoc = new DOMParser().parseFromString("<root><value>tomato</value></root>", "application/xml");
                    let xpathResult =  xmlDoc.evaluate("/root//value", xmlDoc, null, XPathResult.ANY_TYPE, null);
                    wasm.test_xpath_result(xpathResult);
                }
            "#,
        )
        .test();
}
