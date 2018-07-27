use super::websys_project;

#[test]
fn html_element() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_custom_section)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_html_element(element: &web_sys::HtmlElement) {
                    assert_eq!(element.title(), "", "Shouldn't have a title");
                    element.set_title("boop");
                    assert_eq!(element.title(), "boop", "Should have a title");

                    assert_eq!(element.lang(), "", "Shouldn't have a lang");
                    element.set_lang("en-us");
                    assert_eq!(element.lang(), "en-us", "Should have a lang");

                    assert_eq!(element.dir(), "", "Shouldn't have a dir");
                    element.set_dir("ltr");
                    assert_eq!(element.dir(), "ltr", "Should have a dir");

                    assert_eq!(element.inner_text(), "", "Shouldn't have inner_text");
                    element.set_inner_text("hey");
                    assert_eq!(element.inner_text(), "hey", "Should have inner_text");

                    assert!(!element.hidden(), "Shouldn't be hidden");
                    element.set_hidden(true);
                    assert!(element.hidden(), "Should be hidden");

                    // TODO add a click handler here
                    element.click();

                    assert_eq!(element.tab_index(), -1, "Shouldn't be tab_index");
                    element.set_tab_index(1);
                    assert_eq!(element.tab_index(), 1, "Should be tab_index");

                    // TODO add a focus handler here
                    assert_eq!(element.focus().unwrap(), (), "No result");

                    // TODO add a blur handler here
                    assert_eq!(element.blur().unwrap(), (), "No result");

                    assert_eq!(element.access_key(), "", "Shouldn't have a access_key");
                    element.set_access_key("a");
                    assert_eq!(element.access_key(), "a", "Should have a access_key");

                    // TODO add test for access_key_label

                    assert!(!element.draggable(), "Shouldn't be draggable");
                    element.set_draggable(true);
                    assert!(element.draggable(), "Should be draggable");

                    assert_eq!(element.content_editable(), "inherit", "Shouldn't have a content_editable");
                    element.set_content_editable("true");
                    assert_eq!(element.content_editable(), "true", "Should be content_editable");
                    assert!(element.is_content_editable(), "Should be content_editable");

                    // TODO verify case where menu is passed
                    match element.context_menu() {
                        None => assert!(true, "Shouldn't have a custom menu set"),
                        _ => assert!(false, "Shouldn't have a custom menu set")
                    };

                    assert!(!element.spellcheck(), "Shouldn't be spellchecked");
                    element.set_spellcheck(true);
                    assert!(element.spellcheck(), "Should be dragspellcheckedgable");

                    // TODO verify case where we have an offset_parent
                    match element.offset_parent() {
                        None => assert!(true, "Shouldn't have an offset_parent set"),
                        _ => assert!(false, "Shouldn't have a offset_parent set")
                    };

                    // TODO verify when we have offsets
                    assert_eq!(element.offset_top(), 0, "Shouldn't have an offset_top yet");
                    assert_eq!(element.offset_left(), 0, "Shouldn't have an offset_left yet");
                    assert_eq!(element.offset_width(), 0, "Shouldn't have an offset_width yet");
                    assert_eq!(element.offset_height(), 0, "Shouldn't have an offset_height yet");
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let html = document.createElement("html");
                    wasm.test_html_element(html);
                }
            "#,
        )
        .test();
}
