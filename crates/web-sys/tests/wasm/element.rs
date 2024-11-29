use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use web_sys::Element;

#[wasm_bindgen(module = "/tests/wasm/element.js")]
extern "C" {
    fn new_div() -> Element;
}

#[wasm_bindgen_test]
fn element() {
    /* Tests needed for:
      namespace_uri
    */
    let element = new_div();

    #[allow(clippy::eq_op)]
    {
        assert_eq!(element, element);
    }

    assert_eq!(element.prefix(), None, "Shouldn't have a prefix");
    assert_eq!(element.local_name(), "div", "Should have a div local name");
    assert_eq!(element.tag_name(), "DIV", "Should be a div tag");
    assert!(!element.has_attribute("id"), "Shouldn't have an id");
    element.set_id("beep");
    assert_eq!(element.id(), "beep", "Should have an id of 'beep'");

    // must_use is set on this result?
    // Should set id
    let _: () = element.set_attribute("id", "beep").unwrap();
    assert!(element.has_attribute("id"), "Should now have an id");
    // Should return nothing if removed
    let _: () = element.remove_attribute("id").unwrap();

    assert_eq!(element.class_name(), "", "Shouldn't have a class name");
    element.set_class_name("test thing");
    assert_eq!(
        element.class_name(),
        "test thing",
        "Should have a class name"
    );
    assert_eq!(
        element.get_attribute("class").unwrap(),
        "test thing",
        "Should have a class name"
    );
    // Should return nothing if removed
    let _: () = element.remove_attribute("class").unwrap();
    /* Tests needed for:
    get_attribute_ns
    */

    assert!(!element.has_attribute("disabled"), "Should not be disabled");
    assert!(
        element.toggle_attribute("disabled").unwrap(),
        "Should return true when attribute is set"
    );
    assert!(element.has_attribute("disabled"), "Should be disabled");
    assert!(
        element
            .toggle_attribute_with_force("disabled", true)
            .unwrap(),
        "Should return true when attribute is set"
    );
    assert!(element.has_attribute("disabled"), "Should be disabled");
    assert!(
        !element
            .toggle_attribute_with_force("disabled", false)
            .unwrap(),
        "Should return false when attribute is not set"
    );
    assert!(!element.has_attribute("disabled"), "Should not be disabled");

    assert!(!element.has_attribute("title"), "Should not have a title");
    // Should return nothing if set correctly
    let _: () = element.set_attribute("title", "boop").unwrap();
    assert!(element.has_attribute("title"), "Should have a title");
    assert_eq!(
        element.get_attribute("title").unwrap(),
        "boop",
        "Title should be 'boop'"
    );
    // Should return nothing if removed
    let _: () = element.remove_attribute("title").unwrap();
    assert!(!element.has_attribute("title"), "Should not have a title");
    /* Tests needed for:
    set_attribute_ns
    */

    assert!(!element.has_attributes(), "Should not have any attributes");
    // Should return nothing if set correctly
    let _: () = element.set_attribute("title", "boop").unwrap();
    assert!(element.has_attributes(), "Should have attributes");
    // Should return nothing if removed
    let _: () = element.remove_attribute("title").unwrap();
    /* Tests needed for:
    remove_attribute_ns
    has_attribute_ns
    closest
    */

    assert!(
        !element.matches(".this-is-a-thing").unwrap(),
        "Should not match selector"
    );
    assert!(
        !element.webkit_matches_selector(".this-is-a-thing").unwrap(),
        "Should not match selector"
    );
    element.set_class_name("this-is-a-thing");
    assert!(
        element.matches(".this-is-a-thing").unwrap(),
        "Should match selector"
    );
    assert!(
        element.webkit_matches_selector(".this-is-a-thing").unwrap(),
        "Should match selector"
    );
    // Should return nothing if removed
    let _: () = element.remove_attribute("class").unwrap();

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
    assert_eq!(
        element.inner_html(),
        "<strong>Hey!</strong><em>Web!</em>",
        "Should return HTML content"
    );
    assert_eq!(
        element.query_selector_all("strong").unwrap().length(),
        1,
        "Should return one element"
    );
    assert!(
        element.query_selector("strong").unwrap().is_some(),
        "Should return an element"
    );
    element.set_inner_html("");
    assert_eq!(element.inner_html(), "", "Should return no content");

    /* Tests needed for:
    outer_html
    set_outer_html
    insert_adjacent_html
    */

    assert!(
        element.query_selector(".none-existent").unwrap().is_none(),
        "Should return no results"
    );
    assert_eq!(
        element
            .query_selector_all(".none-existent")
            .unwrap()
            .length(),
        0,
        "Should return no results"
    );
    /* Tests needed for:
      slot
      set_slot
      request_fullscreen
      request_pointer_lock
    */

    let child = new_div();
    assert_eq!(
        element.get_elements_by_tag_name("div").length(),
        0,
        "Element should not contain any div child"
    );
    element.append_child(&child).unwrap();
    assert_eq!(
        element.get_elements_by_tag_name("div").length(),
        1,
        "Element should contain one div child"
    );
    assert_eq!(
        element.get_elements_by_class_name("foo").length(),
        0,
        "Element should not have children with class foo"
    );
    child.class_list().add_1("foo").unwrap();
    assert_eq!(
        element.get_elements_by_class_name("foo").length(),
        1,
        "Element should have one child with class foo"
    );
    element.remove_child(&child).unwrap();
}
