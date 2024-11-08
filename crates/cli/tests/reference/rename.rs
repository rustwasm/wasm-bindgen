use wasm_bindgen::prelude::*;

#[wasm_bindgen(experimental_auto_camel_case)]
extern "C" {
    #[wasm_bindgen(js_name = foo_bar)]
    fn foo_bar();
    fn qux_corge();
    #[wasm_bindgen(js_namespace = Baz)]
    fn yes_no();
}

#[wasm_bindgen]
pub fn export_from_rust(a: u32) -> u32 {
    foo_bar();
    qux_corge();
    yes_no();

    a
}

#[wasm_bindgen(experimental_auto_camel_case)]
pub struct RustStruct {
    pub foo: u32,
    pub some_cool_field: u32,
    #[wasm_bindgen(js_name = "another_field_for_you")]
    pub another_field: u32,
}

#[wasm_bindgen]
impl RustStruct {
    pub fn i_dont_get_renamed() {}
}

#[wasm_bindgen(experimental_auto_camel_case)]
impl RustStruct {
    // methods

    pub fn increment_foo(&mut self, amount: Option<u32>) {}
    pub fn set_foo(&mut self, foo: u32) {}

    #[wasm_bindgen(js_name = get_another)]
    pub fn another(&self) -> u32 {
        self.another_field
    }

    // getters/setters

    #[wasm_bindgen(getter)]
    pub fn some_other_prop(&self) -> u32 {
        0
    }
    #[wasm_bindgen(setter)]
    pub fn set_some_other_prop(&self, value: u32) {}

    #[wasm_bindgen(getter = my_own_name)]
    pub fn some_different_prop(&self) -> u32 {
        0
    }
    #[wasm_bindgen(setter = my_own_name)]
    pub fn set_some_different_prop(&self, value: u32) {}

    #[wasm_bindgen(getter, js_name = my_unique_name)]
    pub fn some_unique_prop(&self) -> u32 {
        0
    }
    #[wasm_bindgen(setter, js_name = my_unique_name)]
    pub fn set_some_unique_prop(&self, value: u32) {}

    #[wasm_bindgen(getter)]
    pub fn some_static_prop() -> u32 {
        0
    }
    #[wasm_bindgen(setter)]
    pub fn set_some_static_prop(value: u32) {}

}
