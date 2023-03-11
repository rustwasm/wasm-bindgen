//! Support for printing status information of a test suite in a browser.
//!
//! Currently this is quite simple, rendering the same as the console tests in
//! node.js. Output here is rendered in a `pre`, however.

use js_sys::Error;
use wasm_bindgen::prelude::*;

/// Implementation of `Formatter` for browsers.
///
/// Routes all output to a `pre` on the page currently. Eventually this probably
/// wants to be a pretty table with colors and folding and whatnot.
pub struct Worker {}

#[wasm_bindgen]
extern "C" {
    type WorkerError;
    #[wasm_bindgen(method, getter, structural)]
    fn stack(this: &WorkerError) -> JsValue;

    #[wasm_bindgen(js_name="postMessage")]
    fn post_message(data: Vec<JsValue>);
}

impl Worker {
    /// Attempts to create a new formatter for web worker
    pub fn new() -> Worker {
        post_message(vec![
            JsValue::from(String::from("__wbgtest_start"))
        ]);
        Worker {}
    }
}

impl super::Formatter for Worker {
    fn writeln(&self, line: &str) {
        post_message(vec![
            JsValue::from(String::from("__wbgtest_output")),
            JsValue::from(String::from(line)),
        ]);
    }

    fn log_test(&self, name: &str, result: &Result<(), JsValue>) {
        let s = if result.is_ok() { "ok" } else { "FAIL" };
        self.writeln(&format!("test {} ... {}", name, s));
    }

    fn stringify_error(&self, err: &JsValue) -> String {
        // TODO: this should be a checked cast to `Error`
        let err = Error::from(err.clone());
        let name = String::from(err.name());
        let message = String::from(err.message());
        let err = WorkerError::from(JsValue::from(err));
        let stack = err.stack();

        let header = format!("{}: {}", name, message);
        let stack = match stack.as_string() {
            Some(stack) => stack,
            None => return header,
        };

        // If the `stack` variable contains the name/message already, this is
        // probably a chome-like error which is already rendered well, so just
        // return this info
        if stack.contains(&header) {
            return stack;
        }

        // Fallback to make sure we don't lose any info
        format!("{}\n{}", header, stack)
    }
}
