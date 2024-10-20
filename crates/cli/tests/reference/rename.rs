use wasm_bindgen::prelude::*;

#[wasm_bindgen(rename_all = "camelCase")]
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

#[wasm_bindgen(rename_all = "camelCase")]
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

#[wasm_bindgen(rename_all = "camelCase")]
impl RustStruct {
    pub fn increment_foo(&mut self, amount: Option<u32>) {
        self.foo += amount.unwrap_or(1);
    }

    pub fn set_foo(&mut self, foo: u32) {
        self.foo = foo;
    }

    #[wasm_bindgen(js_name = get_another)]
    pub fn another(&self) -> u32 {
        self.another_field
    }
}
