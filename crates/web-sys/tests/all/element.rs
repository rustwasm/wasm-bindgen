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
/* Tests needed for:
  namespace_uri
*/
                    assert_eq!(element.prefix(), None, "Shouldn't have a prefix");
                    assert_eq!(element.local_name(), "div", "Should have a div local name");
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
                    assert_eq!(element.get_attribute("class").unwrap(), "test thing", "Should have a class name");
                    assert_eq!(element.remove_attribute("class").unwrap(), (), "Should return nothing if removed");
/* Tests needed for:
  get_attribute_ns
*/

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
/* Tests needed for:
  set_attribute_ns
*/

                    assert!(!element.has_attributes(), "Should not have any attributes");
                    assert_eq!(element.set_attribute("title", "boop").unwrap(), (), "Should return nothing if set correctly");
                    assert!(element.has_attributes(), "Should have attributes");
                    assert_eq!(element.remove_attribute("title").unwrap(), (), "Should return nothing if removed");
/* Tests needed for:
  remove_attribute_ns
  has_attribure_ns
  closest
*/

                    assert_eq!(element.matches(".this-is-a-thing").unwrap(), false, "Should not match selector");
                    assert_eq!(element.webkit_matches_selector(".this-is-a-thing").unwrap(), false, "Should not match selector");
                    element.set_class_name("this-is-a-thing");
                    assert_eq!(element.matches(".this-is-a-thing").unwrap(), true, "Should match selector");
                    assert_eq!(element.webkit_matches_selector(".this-is-a-thing").unwrap(), true, "Should match selector");
                    assert_eq!(element.remove_attribute("class").unwrap(), (), "Should return nothing if removed");

// TODO non standard moz_matches_selector should we even support?

/* Tests needed for:
  insert_adjacent_element
  insert_adjacent_text
  set_pointer_capture
  release_pointer_capture
  has_pointer_capture
  set_capture
  release_capture
  scroll_top
  set_scroll_top
  scroll_left
  set_scroll_left
  scroll_width
  scroll_height
  scroll,
  scroll_to
  scroll_by
  client_top
  client_left
  client_width
  client_height
  scroll_top_max
  scroll_left_max
*/
                    assert_eq!(element.inner_html(), "", "Should return no content");
                    element.set_inner_html("<strong>Hey!</strong><em>Web!</em>");
                    assert_eq!(element.inner_html(), "<strong>Hey!</strong><em>Web!</em>", "Should return HTML conent");
                    assert_eq!(element.query_selector_all("strong").unwrap().length(), 1, "Should return one element");
                    assert!(element.query_selector("strong").unwrap().is_some(), "Should return an element");
                    element.set_inner_html("");
                    assert_eq!(element.inner_html(), "", "Should return no content");

/* Tests needed for:
  outer_html
  set_outer_html
  insert_adjacent_html
*/

                    assert!(element.query_selector(".none-existant").unwrap().is_none(), "Should return no results");
                    assert_eq!(element.query_selector_all(".none-existant").unwrap().length(), 0, "Should return no results");
/* Tests needed for:
  slot
  set_slot
  request_fullscreen
  request_pointer_lock
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
