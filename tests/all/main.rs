#![cfg(not(target_arch = "wasm32"))]

extern crate wasm_bindgen_test_project_builder as project_builder;

use project_builder::project;

mod comments;
