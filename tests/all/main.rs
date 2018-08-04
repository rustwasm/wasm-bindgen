#![cfg(not(target_arch = "wasm32"))]

extern crate wasm_bindgen_test_project_builder as project_builder;

use project_builder::{project, run};

mod classes;
mod closures;
mod comments;
mod dependencies;
mod duplicates;
mod enums;
mod import_class;
mod imports;
mod jsobjects;
mod math;
mod node;
mod non_debug;
mod non_wasm;
mod simple;
mod slice;
mod structural;
mod typescript;
mod u64;
mod validate_prt;
