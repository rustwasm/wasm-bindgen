//! Support for printing status information of a test suite in node.js
//!
//! This currently uses the same output as `libtest`, only reimplemented here
//! for node itself.

use wasm_bindgen::prelude::*;

/// Implementation of the `Formatter` trait for node.js
pub struct Node {}

#[wasm_bindgen]
extern "C" {
    // Not using `js_sys::Error` because node's errors specifically have a
    // `stack` attribute.
    type NodeError;
    #[wasm_bindgen(method, getter, js_class = "Error", structural)]
    fn stack(this: &NodeError) -> String;
}

impl Node {
    /// Attempts to create a new formatter for node.js
    pub fn new() -> Node {
        Node {}
    }
}

impl super::Formatter for Node {
    fn writeln(&self, line: &str) {
        super::js_console_log(line);
    }

    fn log_test(&self, name: &str, result: &Result<(), JsValue>) {
        let s = if result.is_ok() { "ok" } else { "FAIL" };
        self.writeln(&format!("test {} ... {}", name, s));
    }

    fn stringify_error(&self, err: &JsValue) -> String {
        // TODO: should do a checked cast to `NodeError`
        NodeError::from(err.clone()).stack()
    }
}
