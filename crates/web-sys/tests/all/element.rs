use super::websys_project;

#[test]
fn element() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_custom_section)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_element(element: &web_sys::Element) {
                    assert_eq!(element.local_name(), "div", "Shouldn't have a div local name");
                    assert_eq!(element.tag_name(), "div", "Should be a div tag");
                    assert!(!element.has_attribute("id"), "Shouldn't have an id");
                    element.set_id("beep");
                    assert_eq!(element.id(), "beep", "Should have an id of 'beep'");

                    // must_use is set on this result?
                    assert_eq!(element.set_attribute("id", "beep").unwrap(), (), "Should set id");
                    assert!(element.has_attribute("id"), "Should now have an id");
                    assert_eq!(element.remove_attribute("id").unwrap(), (), "Should return nothing if removed");

                    assert_eq!(element.class_name(), "", "Shouldn't have a class name");
                    element.set_class_name("test thing");
                    assert_eq!(element.class_name(), "test thing", "Should have a class name");
                    assert_eq!(element.remove_attribute("class").unwrap(), (), "Should return nothing if removed");

/*TODO should we enable toggle_attribute tests? (Firefox Nightly + Chrome canary only)
                    // TODO toggle_attribute should permit a single argument when optional arguments are supported
                    assert!(!element.has_attribute("disabled"), "Should not be disabled");
                    assert!(element.toggle_attribute("disabled", true).unwrap(), "Should return true when attribute is set");
                    assert!(element.has_attribute("disabled"), "Should be disabled");
                    assert!(!element.toggle_attribute("disabled", false).unwrap(), "Should return false when attribute is not set");
                    assert!(!element.has_attribute("disabled"), "Should not be disabled");
*/

                    assert!(!element.has_attribute("title"), "Should not have a title");
                    assert_eq!(element.set_attribute("title", "boop").unwrap(), (), "Should return nothing if set correctly");
                    assert!(element.has_attribute("title"), "Should have a title");
                    // TODO check get_attribute here when supported
                    assert_eq!(element.remove_attribute("title").unwrap(), (), "Should return nothing if removed");
                    assert!(!element.has_attribute("title"), "Should not have a title");

                    assert!(!element.has_attributes(), "Should not have any attributes");
                    assert_eq!(element.set_attribute("title", "boop").unwrap(), (), "Should return nothing if set correctly");
                    assert!(element.has_attributes(), "Should have attributes");
                    assert_eq!(element.remove_attribute("title").unwrap(), (), "Should return nothing if removed");

                    assert_eq!(element.matches(".this-is-a-thing").unwrap(), false, "Should not match selector");
                    assert_eq!(element.webkit_matches_selector(".this-is-a-thing").unwrap(), false, "Should not match selector");
                    element.set_class_name("this-is-a-thing");
                    assert_eq!(element.matches(".this-is-a-thing").unwrap(), true, "Should match selector");
                    assert_eq!(element.webkit_matches_selector(".this-is-a-thing").unwrap(), true, "Should match selector");
                    assert_eq!(element.remove_attribute("class").unwrap(), (), "Should return nothing if removed");


// TODO non standard moz_matches_selector should we even support?
/* Tests needed for:
  insert_adjacent_text
  set_pointer_capture
  release_pointer_capture
  has_pointer_capture
  set_capture
  release_capture
*/
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let document = new Document();
                    let el = document.createElement("div");
                    wasm.test_element(el);
                }
            "#,
        )
        .test();
}
