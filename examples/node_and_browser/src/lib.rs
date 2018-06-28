#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate pulldown_cmark;
extern crate wasm_bindgen;
use pulldown_cmark::{Parser, Options, html::push_html};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// This function will take a string formatted
/// using the CommonMark syntax and returns a
/// HTML string to match the input
pub fn parse_markdown(md: &str) -> String {
    let opts = Options::all();
    let p = Parser::new_ext(md, opts);
    let mut ret = String::new();
    push_html(&mut ret, p);
    ret
}