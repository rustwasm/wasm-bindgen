use super::websys_project;

#[test]
fn response() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_response(response: &web_sys::Response) {
                    assert!(!response.ok());
                    assert!(!response.redirected());
                    assert_eq!(response.status(), 501);
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let response = new Response(null, {status: 501});
                    wasm.test_response(response);
                }
            "#,
        )
        .test();
}
