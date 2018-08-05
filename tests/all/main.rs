#![cfg(not(target_arch = "wasm32"))]

extern crate wasm_bindgen_test_project_builder as project_builder;

use project_builder::{project, run};

mod comments;
mod dependencies;
mod import_class;
mod imports;
mod js_objects;
mod node;
mod non_debug;
mod non_wasm;
mod simple;
mod typescript;
