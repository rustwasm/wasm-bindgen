use super::websys_project;

#[test]
fn event() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_event(event: &web_sys::Event) {
                    // These should match `new Event`.
                    assert!(event.bubbles());
                    assert!(event.cancelable());
                    assert!(event.composed());

                    // The default behavior not initially prevented, but after
                    // we call `prevent_default` it better be.
                    assert!(!event.default_prevented());
                    event.prevent_default();
                    assert!(event.default_prevented());
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export async function test() {
                    await new Promise(resolve => {
                        window.addEventListener("test-event", e => {
                          wasm.test_event(e);
                          resolve();
                        });

                        window.dispatchEvent(new Event("test-event", {
                            bubbles: true,
                            cancelable: true,
                            composed: true,
                        }));
                    });
                }
            "#,
        )
        .test();
}
