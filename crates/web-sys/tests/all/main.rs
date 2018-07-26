extern crate wasm_bindgen_test_project_builder as project_builder;
use project_builder::{project, Project};

mod event;
mod headers;
mod response;
mod element;
mod history;

fn websys_project() -> Project {
    project()
        .add_local_dependency("web-sys", env!("CARGO_MANIFEST_DIR"))
        .headless(true)
        .clone()
}
