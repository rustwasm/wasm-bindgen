#![feature(use_extern_macros)]
#![cfg(target_arch = "wasm32")]

extern crate futures;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate wasm_bindgen_test;
extern crate web_sys;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub mod anchor_element;
pub mod body_element;
pub mod br_element;
pub mod button_element;
pub mod div_element;
pub mod element;
pub mod event;
pub mod head_element;
pub mod heading_element;
pub mod headers;
pub mod history;
pub mod hr_element;
pub mod html_element;
pub mod html_html_element;
pub mod input_element;
pub mod mod_elements;
pub mod olist_element;
pub mod optgroup_element;
pub mod option_element;
pub mod options_collection;
pub mod output_element;
pub mod paragraph_element;
pub mod param_element;
pub mod pre_element;
pub mod progress_element;
pub mod quote_element;
pub mod response;
pub mod select_element;
pub mod script_element;
pub mod slot_element;
pub mod span_element;
pub mod style_element;
pub mod table_element;
pub mod title_element;
pub mod xpath_result;
