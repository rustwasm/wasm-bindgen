use super::websys_project;

#[test]
fn history() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_history(history: &web_sys::History) {
                    assert_eq!(history.length().unwrap(), 2);

                    assert!(history.go(1).is_ok());
                    assert!(history.back().is_ok());
                    assert!(history.forward().is_ok());
                    assert!(history.go(-1).is_ok());

                    history.set_scroll_restoration(web_sys::ScrollRestoration::Manual).expect("failure to set scroll restoration");
                    assert_eq!(history.scroll_restoration().unwrap(), web_sys::ScrollRestoration::Manual);

                    history.set_scroll_restoration(web_sys::ScrollRestoration::Auto).expect("failure to set scroll restoration");
                    assert_eq!(history.scroll_restoration().unwrap(), web_sys::ScrollRestoration::Auto);
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    window.history.pushState({}, "I am a title", "part/of/some/url");
                    wasm.test_history(window.history);
                }
            "#,
        )
        .test();
}
