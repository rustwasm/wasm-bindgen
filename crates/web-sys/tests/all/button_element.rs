use super::websys_project;

#[test]
fn button_element() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_custom_section)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_button_element(element: &web_sys::HtmlButtonElement, location: String) {
                    assert!(!element.autofocus(), "Shouldn't have autofocus");
                    element.set_autofocus(true);
                    assert!(element.autofocus(), "Should have autofocus");

                    assert!(!element.disabled(), "Shouldn't be disabled");
                    element.set_disabled(true);
                    assert!(element.disabled(), "Should be disabled");

                    match element.form() {
                        None => assert!(true, "Shouldn't have a form"),
                        _ => assert!(false, "Shouldn't have a form"),
                    };

                    assert_eq!(element.form_action(), location, "Should have the pages location");
                    element.set_form_action("http://boop.com/");
                    assert_eq!(element.form_action(), "http://boop.com/", "Should have a form_action");

                    assert_eq!(element.form_enctype(), "", "Should have no enctype");
                    element.set_form_enctype("text/plain");
                    assert_eq!(element.form_enctype(), "text/plain", "Should have a plain text enctype");

                    assert_eq!(element.form_method(), "", "Should have no method");
                    element.set_form_method("POST");
                    assert_eq!(element.form_method(), "post", "Should have a POST method");

                    assert!(!element.form_no_validate(), "Should validate");
                    element.set_form_no_validate(true);
                    assert!(element.form_no_validate(), "Should not validate");

                    assert_eq!(element.form_target(), "", "Should have no target");
                    element.set_form_target("_blank");
                    assert_eq!(element.form_target(), "_blank", "Should have a _blank target");

                    assert_eq!(element.name(), "", "Shouldn't have a name");
                    element.set_name("button-name");
                    assert_eq!(element.name(), "button-name", "Should have a name");

                    assert_eq!(element.type_(), "submit", "Shouldn't have a type");
                    element.set_type("reset");
                    assert_eq!(element.type_(), "reset", "Should have a reset type");

                    assert_eq!(element.value(), "", "Shouldn't have a value");
                    element.set_value("value1");
                    assert_eq!(element.value(), "value1", "Should have a value");

                    assert_eq!(element.will_validate(), false, "Shouldn't validate");
                    assert_eq!(element.validation_message().unwrap(), "", "Shouldn't have a value");
                    assert_eq!(element.check_validity(), true, "Should be valid");
                    assert_eq!(element.report_validity(), true, "Should be valid");
                    element.set_custom_validity("Boop"); // Method exists but doesn't impact validity
                    assert_eq!(element.check_validity(), true, "Should be valid");
                    assert_eq!(element.report_validity(), true, "Should be valid");

                    assert_eq!(element.labels().length(), 0, "Should return a node list with no elements");
                }

                #[wasm_bindgen]
                pub fn test_button_element_in_form(element: &web_sys::HtmlButtonElement) {
                    match element.form() {
                        None => assert!(false, "Should have a form"),
                        Some(form) => {
                            assert!(true, "Should have a form");
                            assert_eq!(form.name(), "test-form", "Form should have a name of test-form");
                        },
                    };
                    assert_eq!(element.type_(), "reset", "Should have a type");
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let button = document.createElement("button");
                    wasm.test_button_element(button, document.location.href);

                    let button2 = document.createElement("button");
                    button2.type = "reset";
                    let form = document.createElement("form");
                    form.name = "test-form";
                    form.appendChild(button2);
                    wasm.test_button_element_in_form(button2);
                }
            "#,
        )
        .test();
}
