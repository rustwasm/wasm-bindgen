#![cfg(not(target_arch = "wasm32"))]

extern crate wasm_bindgen_test_project_builder as project_builder;

use project_builder::{project, run};

mod comments;
mod dependencies;
mod js_objects;
mod imports;
mod simple;
mod node;
mod non_debug;
mod non_wasm;
mod typescript;
