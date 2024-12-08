//! Runtime detection of whether we're in node.js or a browser.

use alloc::string::String;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type This;
    #[wasm_bindgen(method, getter, structural, js_name = self)]
    fn self_(me: &This) -> Option<Scope>;

    type Scope;
    #[wasm_bindgen(method, getter, structural)]
    fn constructor(me: &Scope) -> Constructor;

    #[wasm_bindgen(method, getter, structural, js_name = Deno)]
    fn deno(me: &Scope) -> Option<Deno>;

    type Deno;

    type Constructor;
    #[wasm_bindgen(method, getter, structural)]
    fn name(me: &Constructor) -> String;
}

/// Detecting the current JS scope
pub fn detect() -> Runtime {
    // Test whether we're in a browser/worker by seeing if the `self` property is
    // defined on the global object and it is not equal to a WorkerScope, which should in turn
    // only be true in browsers.
    match js_sys::global().unchecked_into::<This>().self_() {
        Some(scope) => match scope.constructor().name().as_str() {
            "DedicatedWorkerGlobalScope"
            | "SharedWorkerGlobalScope"
            | "ServiceWorkerGlobalScope" => Runtime::Worker,
            _ => match scope.deno() {
                Some(_) => Runtime::Node,
                _ => Runtime::Browser,
            },
        },
        None => Runtime::Node,
    }
}

/// Current runtime environment
pub enum Runtime {
    /// Current scope is a browser scope
    Browser,
    /// Current scope is a node scope
    Node,
    /// Current scope is a worker scope
    Worker,
}
