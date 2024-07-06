use crate::__steps__::assembly::AssemblyBuilder;
use crate::__steps__::wasm_bindgen_test_runner::Sandbox;
use crate::__steps__::Context;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

lazy_static! {
    static ref ASSEMBLY_CACHE: Mutex<HashMap<String, PathBuf>> = Mutex::new(HashMap::new());
    static ref ASSEMBLY: PathBuf = AssemblyBuilder::new("assembly_with_one_successful_test")
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
        )
        .build();
}

pub fn given_there_is_an_assembly_with(context: &mut Context, content: &str) {
    let mut cache = ASSEMBLY_CACHE.lock().unwrap();

    let assembly_path = cache
        .entry(content.to_string())
        .or_insert_with(|| {
            // If the assembly is not in the cache, compile it and insert into the cache
            AssemblyBuilder::new("assembly_with_one_successful_test")
                .file("src/lib.rs", &generate_content(content))
                .build()
        })
        .clone();

    context.sandbox_set(Sandbox::new(assembly_path));
}

fn generate_content(content: &str) -> String {
    format!(
        r#"#[cfg(test)]
use wasm_bindgen_test::*;

#[cfg(test)]
{}
"#,
        content
    )
}
