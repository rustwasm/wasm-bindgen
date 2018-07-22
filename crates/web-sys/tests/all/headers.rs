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
                pub fn test_headers(headers: &web_sys::Headers) {
                    assert_eq!(headers.get("foo").unwrap(), None);
                    assert_eq!(
                        headers.get("content-type").unwrap(),
                        Some("text/plain".to_string()),
                    );
                    assert_eq!(
                        headers.get("Content-Type").unwrap(),
                        Some("text/plain".to_string()),
                    );
                    assert!(headers.get("").is_err());
                    assert!(headers.set("", "").is_err());
                    assert!(headers.set("x", "").is_ok());
                    assert_eq!(headers.get("x").unwrap(), Some(String::new()));
                    assert!(headers.delete("x").is_ok());
                    assert_eq!(headers.get("x").unwrap(), None);
                    assert!(headers.append("a", "y").is_ok());
                    assert!(headers.append("a", "z").is_ok());
                    assert_eq!(headers.get("a").unwrap(), Some("y, z".to_string()));
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
