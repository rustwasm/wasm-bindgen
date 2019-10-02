use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/validate_prt.js")]
extern "C" {
    fn js_works();
}

#[wasm_bindgen]
pub struct Fruit {
    name: String,
}

#[wasm_bindgen]
impl Fruit {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> WasmType<Self> {
        instantiate! { Fruit { name } }
    }

    pub fn rot(_self: WasmType<Fruit>) {
        drop(_self);
    }
}

#[wasm_bindgen]
pub fn eat(_: WasmType<Fruit>) {}

#[wasm_bindgen_test]
fn works() {
    js_works();
}
