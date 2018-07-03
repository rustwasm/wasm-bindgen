extern crate wasm_bindgen_cli_support;

mod project_builder;
use project_builder::{project, run};

mod api;
mod char;
mod classes;
mod closures;
mod comments;
mod dependencies;
mod enums;
mod import_class;
mod imports;
#[cfg(feature = "js_globals")]
mod js_globals;
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
mod webidl;
