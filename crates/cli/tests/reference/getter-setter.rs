use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct Foo {
    pub x: u32,
    pub y: Option<u32>,
    z: Option<u32>,
}

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(getter)]
    pub fn z(&self) -> Option<u32> {
        self.z
    }
    #[wasm_bindgen(setter)]
    pub fn set_z(&mut self, z: Option<u32>) {
        self.z = z;
    }

    #[wasm_bindgen(getter)]
    pub fn lone_getter(&self) -> Option<u32> {
        self.z
    }

    #[wasm_bindgen(setter)]
    pub fn set_lone_setter(&mut self, value: Option<u32>) {}

    /// You will only read numbers.
    #[wasm_bindgen(getter)]
    pub fn weird(&self) -> u32 {
        42
    }
    /// But you must write strings.
    ///
    /// Yes, this is totally fine in JS.
    #[wasm_bindgen(setter)]
    pub fn set_weird(&mut self, value: Option<String>) {}

    /// There can be static getters and setters too, and they can even have the
    /// same name as instance getters and setters.
    #[wasm_bindgen(getter = x)]
    pub fn x_static() -> Option<bool> {
        None
    }
    #[wasm_bindgen(setter = x)]
    pub fn set_x_static(value: Option<bool>) {}
}
