use crate::__steps__::Context;
use crate::__steps__::Project;
use crate::__steps__::wasm_bindgen_test_runner::Sandbox;

pub fn given_there_is_an_assembly_with_one_successful_test(context: &mut Context) {
    let assembly = Project::new("assembly_with_one_successful_test")
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

    context.sandbox_set(Sandbox::new(assembly));
}
