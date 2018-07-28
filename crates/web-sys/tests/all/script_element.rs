use super::websys_project;

#[test]
fn script_element() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_custom_section)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_script_element(element: &web_sys::HtmlScriptElement) {
                    assert_eq!(element.src(), "", "Shouldn't have a src");
                    element.set_src("https://example.com/script.js");
                    assert_eq!(element.src(), "https://example.com/script.js", "Should have a src");

                    assert_eq!(element.type_(), "", "Shouldn't have a type");
                    element.set_type("application/javascript");
                    assert_eq!(element.type_(), "application/javascript", "Should have a type");

                    assert!(!element.no_module(), "Shouldn't be a nomodule");
                    element.set_no_module(true);
                    assert!(element.no_module(), "Should be a nomodule");

                    assert_eq!(element.charset(), "", "Shouldn't have a charset");
                    element.set_charset("UTF-8");
                    assert_eq!(element.charset(), "UTF-8", "Should have a charset");

                    assert!(element.async(), "Should be async");
                    element.set_async(false);
                    assert!(!element.async(), "Shouldn't be a async");

                    assert!(!element.defer(), "Shouldn't be a defer");
                    element.set_defer(true);
                    assert!(element.defer(), "Should be a defer");

                    assert!(element.cross_origin().is_none(), "Shouldn't have a crossorigin");
                    element.set_cross_origin(Some("anonymous"));
                    assert_eq!(element.cross_origin().unwrap(), "anonymous", "Should have a crossorigin");
                    element.set_cross_origin(None);
                    assert!(element.cross_origin().is_none(), "Shouldn't have a crossorigin");

                    assert_eq!(element.text().unwrap(), "", "Shouldn't have text");
                    assert_eq!(element.set_text("text").unwrap(), ());
                    assert_eq!(element.text().unwrap(), "text", "Should have text");

                    assert_eq!(element.event(), "", "Shouldn't have an event");
                    element.set_event("ev");
                    assert_eq!(element.event(), "ev", "Should have an event");

                    assert_eq!(element.html_for(), "", "Shouldn't have an html_for");
                    element.set_html_for("hey");
                    assert_eq!(element.html_for(), "hey", "Should have an html_for");

                    assert_eq!(element.integrity(), "", "Shouldn't have an integrity");
                    element.set_integrity("integrity-val");
                    assert_eq!(element.integrity(), "integrity-val", "Should have an integrity");
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let script = document.createElement("script");
                    wasm.test_script_element(script);
                }
            "#,
        )
        .test();
}
