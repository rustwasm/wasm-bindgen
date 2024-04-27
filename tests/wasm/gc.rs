use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/gc.js")]
extern "C" {
    fn drop_callback();

    async fn owned_methods();
    async fn gc_builder();
    async fn gc_constructor();
}

#[wasm_bindgen]
pub struct OwnedValue {
    pub n: f64,
}

#[wasm_bindgen]
impl OwnedValue {
    #[wasm_bindgen(constructor)]
    pub fn new(n: f64) -> Self {
        Self { n }
    }

    pub fn build(n: f64) -> Self {
        Self::new(n)
    }

    #[allow(clippy::should_implement_trait)] // traits unsupported by wasm_bindgen
    pub fn add(self, other: OwnedValue) -> Self {
        Self {
            n: self.n + other.n,
        }
    }

    pub fn n(self) -> f64 {
        self.n
    }
}

impl Drop for OwnedValue {
    fn drop(&mut self) {
        drop_callback();
    }
}

#[wasm_bindgen_test]
async fn test_owned_methods() {
    owned_methods().await
}

#[wasm_bindgen_test]
async fn test_gc_builder() {
    gc_builder().await
}

#[wasm_bindgen_test]
async fn test_gc_constructor() {
    gc_constructor().await
}
