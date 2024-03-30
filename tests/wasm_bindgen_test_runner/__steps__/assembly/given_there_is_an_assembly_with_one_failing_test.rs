use super::AssemblyBuilder;
use crate::__steps__::wasm_bindgen_test_runner::Sandbox;
use crate::__steps__::Context;
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    static ref ASSEMBLY: PathBuf = AssemblyBuilder::new("assembly_with_one_failing_test")
        .file(
            "src/lib.rs",
            r#"#[cfg(test)]
use wasm_bindgen_test::*;

#[cfg(test)]
#[wasm_bindgen_test]
fn fail() {
    assert_eq!(1, 2);
}
            "#,
        )
        .build();
}

pub fn given_there_is_an_assembly_with_one_failing_test(context: &mut Context) {
    context.sandbox_set(Sandbox::new(ASSEMBLY.clone()));
}
