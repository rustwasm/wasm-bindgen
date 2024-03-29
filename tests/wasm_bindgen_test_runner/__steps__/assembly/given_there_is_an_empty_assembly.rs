use crate::__steps__::wasm_bindgen_test_runner::Sandbox;
use crate::__steps__::Context;
use crate::__steps__::Project;
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    static ref PROJECT: (Project, PathBuf) = get_project();
}

fn get_project() -> (Project, PathBuf) {
    let mut project = Project::new("empty_assembly");

    let path = project
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .build();

    (project, path)
}

pub fn given_there_is_an_empty_assembly(context: &mut Context) {
    context.sandbox_set(Sandbox::new(PROJECT.1.clone()));
}
