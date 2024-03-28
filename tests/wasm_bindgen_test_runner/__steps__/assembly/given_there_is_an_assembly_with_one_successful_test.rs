use crate::__steps__::Context;
use crate::__steps__::Project;

pub fn given_there_is_an_assembly_with_one_successful_test(context: &mut Context) {
    let path = Project::new("assembly_with_one_successful_test")
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

    context.assembly_set(path);
}
