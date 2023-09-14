use std::sync::atomic::{AtomicI32, Ordering};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct OpaqueObject {
    val: AtomicI32,
}

#[wasm_bindgen]
impl OpaqueObject {
    #[wasm_bindgen(constructor)]
    pub fn new(val: i32) -> Self {
        Self {
            val: AtomicI32::new(val),
        }
    }

    #[wasm_bindgen]
    pub fn get(&self) -> i32 {
        self.val.load(Ordering::SeqCst)
    }

    #[wasm_bindgen]
    pub fn add(&self, num: i32) -> i32 {
        self.val.fetch_add(num, Ordering::SeqCst)
    }
}
