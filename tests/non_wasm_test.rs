#![cfg(not(target_family = "wasm"))]

use std::sync::{Arc, Condvar, Mutex};

use once_cell::sync::Lazy;
use wasm_bindgen_test::wasm_bindgen_test;

static TEST: Lazy<Arc<(Mutex<bool>, Condvar)>> =
    Lazy::new(|| Arc::new((Mutex::new(false), Condvar::new())));

#[wasm_bindgen_test(unsupported = test)]
fn test_success() {
    let mut success = TEST.0.lock().unwrap();
    *success = true;
    // We notify the condvar that the value has changed.
    TEST.1.notify_one();
}

#[test]
fn test() {
    let mut success = TEST.0.lock().unwrap();
    while !*success {
        success = TEST.1.wait(success).unwrap();
    }
}
