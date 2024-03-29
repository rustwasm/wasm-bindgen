use crate::__steps__::Context;
use crate::__steps__::Project;
use crate::__steps__::wasm_bindgen_test_runner::Sandbox;
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    static ref PROJECT: (Project, PathBuf) = get_project();
}

fn get_project() -> (Project, PathBuf) {
    let mut project = Project::new("assembly_with_one_successful_test");

    let assembly = project
    .file(
        "src/lib.rs",
        r#"#[cfg(test)]
use wasm_bindgen_test::*;

#[cfg(test)]
#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1, 1);
}
            "#,
    ).build();

    (project, assembly)
}

pub fn given_there_is_an_assembly_with_one_successful_test(context: &mut Context) {
    context.sandbox_set(Sandbox::new(PROJECT.1.clone()));
}
