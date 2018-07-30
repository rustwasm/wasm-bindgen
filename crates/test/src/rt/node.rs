//! Support for printing status information of a test suite in node.js
//!
//! This currently uses the same output as `libtest`, only reimplemented here
//! for node itself.

use wasm_bindgen::prelude::*;
use js_sys::eval;

/// Implementation of the `Formatter` trait for node.js
pub struct Node {
    /// Handle to node's imported `fs` module, imported dynamically because we
    /// can't statically do it as it doesn't work in a browser.
    fs: NodeFs,
}

#[wasm_bindgen]
extern {
    // Type declarations for the `writeSync` function in node
    type NodeFs;
    #[wasm_bindgen(method, js_name = writeSync, structural)]
    fn write_sync(this: &NodeFs, fd: i32, data: &[u8]);

    // Not using `js_sys::Error` because node's errors specifically have a
    // `stack` attribute.
    type NodeError;
    #[wasm_bindgen(method, getter, js_class = "Error", structural)]
    fn stack(this: &NodeError) -> String;
}

impl Node {
    /// Attempts to create a new formatter for node.js, returning `None` if this
    /// is executing in a browser and Node won't work.
    pub fn new() -> Option<Node> {
        if super::detect::is_browser() {
            return None
        }

        // Use `eval` for now as a quick fix around static imports not working
        // for dual browser/node support.
        let import = eval("require(\"fs\")").unwrap();
        Some(Node { fs: import.into() })
    }
}

impl super::Formatter for Node {
    fn writeln(&self, line: &str) {
        super::console_log(line);
    }

    fn log_start(&self, name: &str) {
        let data = format!("test {} ... ", name);
        self.fs.write_sync(2, data.as_bytes());
    }

    fn log_success(&self) {
        self.fs.write_sync(2, b"ok\n");
    }

    fn log_ignored(&self) {
        self.fs.write_sync(2, b"ignored\n");
    }

    fn log_failure(&self, err: JsValue) -> String {
        self.fs.write_sync(2, b"ignored\n");
        // TODO: should do a checked cast to `NodeError`
        NodeError::from(err).stack()
    }
}
