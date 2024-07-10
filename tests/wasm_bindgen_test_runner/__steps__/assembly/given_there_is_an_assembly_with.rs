use crate::__steps__::assembly::AssemblyBuilder;
use crate::__steps__::wasm_bindgen_test_runner::Sandbox;
use crate::__steps__::Context;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

lazy_static! {
    static ref ASSEMBLY_CACHE: Mutex<HashMap<String, PathBuf>> = Mutex::new(HashMap::new());
}

pub fn given_there_is_an_assembly_with(context: &mut Context, content: &str) {
    let mut cache = ASSEMBLY_CACHE.lock().unwrap();

    let assembly_path = cache
        .entry(content.to_string())
        .or_insert_with(|| {
            AssemblyBuilder::new("assembly")
                .file("src/lib.rs", &generate_content(content))
                .build()
        })
        .clone();

    context.sandbox_set(Sandbox::new(assembly_path));
}

fn generate_content(content: &str) -> String {
    let mut final_content = String::from(
        r#"
#[cfg(test)]
use wasm_bindgen_test::*;
"#,
    );

    for line in content.lines() {
        if line.starts_with("#[wasm_bindgen_test]") || line.starts_with("mod") {
            final_content.push_str("#[cfg(test)]\n");
        }
        final_content.push_str(line);
        final_content.push_str("\n");
    }

    final_content
}
