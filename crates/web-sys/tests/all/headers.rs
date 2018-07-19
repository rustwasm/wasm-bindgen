use super::websys_project;

#[test]
fn headers() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_headers(_headers: &web_sys::Headers) {
                    // empty for now...
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let headers = new Headers({'Content-Type': 'text/plain'});
                    wasm.test_headers(headers);
                }
            "#,
        )
        .test();
}
