use super::super::AssemblyBuilder;
use crate::__steps__::wasm_bindgen_test_runner::Sandbox;
use crate::__steps__::Context;
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    static ref ASSEMBLY: PathBuf =
        AssemblyBuilder::new("assembly_with_one_successful_and_one_failing_level_1_tests")
            .file(
                "src/lib.rs",
                r#"
#[cfg(test)]
mod level_1 {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn pass() {
        assert_eq!(1, 1);
    }

    #[wasm_bindgen_test]
    fn fail() {
        assert_eq!(1, 2);
    }
}
"#,
            )
            .build();
}

pub fn given_there_is_an_assembly_with_one_successful_and_one_failing_level_1_tests(
    context: &mut Context,
) {
    context.sandbox_set(Sandbox::new(ASSEMBLY.clone()));
}
