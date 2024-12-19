//! Support for printing status information of a test suite in node.js
//!
//! This currently uses the same output as `libtest`, only reimplemented here
//! for node itself.

use alloc::format;
use alloc::string::String;
use wasm_bindgen::prelude::*;

use super::TestResult;

/// Implementation of the `Formatter` trait for node.js
pub struct Node {}

#[wasm_bindgen]
extern "C" {
    // Not using `js_sys::Error` because node's errors specifically have a
    // `stack` attribute.
    type NodeError;
    #[wasm_bindgen(method, getter, js_class = "Error", structural)]
    fn stack(this: &NodeError) -> String;
    #[wasm_bindgen(js_name = __wbgtest_og_console_log)]
    fn og_console_log(s: &str);
}

impl Node {
    /// Attempts to create a new formatter for node.js
    pub fn new() -> Node {
        Node {}
    }
}

impl super::Formatter for Node {
    fn writeln(&self, line: &str) {
        og_console_log(line);
    }

    fn log_test(&self, name: &str, result: &TestResult) {
        self.writeln(&format!("test {} ... {}", name, result));
    }

    fn stringify_error(&self, err: &JsValue) -> String {
        // TODO: should do a checked cast to `NodeError`
        NodeError::from(err.clone()).stack()
    }
}

/// Path to use for coverage data.
#[wasm_bindgen]
pub fn __wbgtest_coverage_path(
    env: Option<String>,
    pid: u32,
    temp_dir: &str,
    module_signature: u64,
) -> String {
    wasm_bindgen_test_shared::coverage_path(env.as_deref(), pid, temp_dir, module_signature)
}
