use std::sync::atomic::{AtomicU16, Ordering};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

static COUNTER: AtomicU16 = AtomicU16::new(12);

#[wasm_bindgen_test]
pub fn test_atomic_var_after_changed() {
    assert_eq!(COUNTER.load(Ordering::SeqCst), 15);
}

#[wasm_bindgen_test]
pub fn test_atomic_var_access() {
    assert_eq!(COUNTER.load(Ordering::SeqCst), 12);
    COUNTER.store(15, Ordering::SeqCst);
}
