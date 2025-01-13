use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RustStruct {
    data: u32,
}

#[wasm_bindgen]
impl RustStruct {
    pub fn instance_method(&self) {}
    fn priv_instance_method(&self) {}
    pub fn static_method() {}

    #[wasm_bindgen(constructor)]
    pub async fn new() -> Self {
        Self { data: 0 }
    }

    #[wasm_bindgen(getter)]
    pub fn prop(self) -> u32 {
        32
    }
    #[wasm_bindgen(setter)]
    pub fn set_prop(self, _value: u32) {}

    #[wasm_bindgen(getter)]
    pub fn static_prop() -> u32 {
        32
    }
    #[wasm_bindgen(setter)]
    pub fn set_static_prop(_value: u32) {}

    #[wasm_bindgen(indexing_getter)]
    pub fn indexing_getter(self) -> u32 {
        32
    }
    #[wasm_bindgen(indexing_setter)]
    pub fn indexing_setter(self, _value: u32) {}
    #[wasm_bindgen(indexing_deleter)]
    pub fn indexing_deleter(self, _value: u32) {}
}

#[wasm_bindgen]
pub enum RustEnum {
    A = 0,
    B = 1,
}

#[wasm_bindgen]
impl RustEnum {
    pub fn instance_method(self) {}
    fn priv_instance_method(self) {}
    pub fn static_method() {}

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::A
    }

    #[wasm_bindgen(getter)]
    pub fn prop(self) -> u32 {
        32
    }
    #[wasm_bindgen(setter)]
    pub fn set_prop(self, _value: u32) {}

    #[wasm_bindgen(getter)]
    pub fn static_prop() -> u32 {
        32
    }
    #[wasm_bindgen(setter)]
    pub fn set_static_prop(_value: u32) {}
}

pub struct NonWasmType;

#[wasm_bindgen]
impl NonWasmType {
    pub fn static_method() {}
}

fn main() {}
