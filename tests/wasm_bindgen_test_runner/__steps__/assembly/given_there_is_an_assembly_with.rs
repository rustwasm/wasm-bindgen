use crate::__steps__::assembly::AssemblyBuilder;
use crate::__steps__::package::prepare_tests;
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
                .file("src/lib.rs", &prepare_tests(content))
                .build()
        })
        .clone();

    context.sandbox_set(Sandbox::new(assembly_path));
}
