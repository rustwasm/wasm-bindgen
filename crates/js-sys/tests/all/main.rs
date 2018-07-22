#![cfg(not(target_arch = "wasm32"))]

extern crate wasm_bindgen_test_project_builder as project_builder;

fn project() -> project_builder::Project {
    let mut p = project_builder::project();
    p.add_local_dependency("js-sys", env!("CARGO_MANIFEST_DIR"));
    return p
}

// Keep these tests in alphabetical order, just like the imports in `src/js.rs`.

mod ArrayIterator;
