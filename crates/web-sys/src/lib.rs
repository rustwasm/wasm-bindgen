#![feature(wasm_import_module)]
#![doc(html_root_url = "https://docs.rs/web-sys/0.2")]

extern crate wasm_bindgen;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
