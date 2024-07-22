use super::AssemblyBuilder;
use crate::__steps__::wasm_bindgen_test_runner::Sandbox;
use crate::__steps__::Context;
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    static ref ASSEMBLY: PathBuf = AssemblyBuilder::new("assembly_without_anything")
        .file(
            "src/lib.rs",
            r#"
            "#,
        )
        .build();
}

pub fn given_there_is_an_assembly_without_anything(context: &mut Context) {
    context.sandbox_set(Sandbox::new(ASSEMBLY.clone()));
}
