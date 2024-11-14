use wasm_bindgen::prelude::*;

#[wasm_bindgen(auto_rename)]
extern "C" {
    #[wasm_bindgen(js_name = foo_bar)]
    fn foo_bar();
    fn qux_corge();
    #[wasm_bindgen(js_namespace = document)]
    fn query_selector(query: &str) -> JsValue;
    #[wasm_bindgen(getter, js_namespace = document)]
    fn document_element() -> JsValue;
    #[wasm_bindgen(getter, js_namespace = document)]
    fn DOCUMENT_NODE() -> u32;

    #[wasm_bindgen(thread_local, js_namespace = Number)]
    static MAX_SAFE_INTEGER: f64;
    #[wasm_bindgen(thread_local, js_namespace = Number)]
    static i_do_not_exist: f64;
}

#[wasm_bindgen]
pub fn export_from_rust(a: u32) -> u32 {
    foo_bar();
    qux_corge();
    query_selector(".class");
    document_element();
    assert!(MAX_SAFE_INTEGER.with(Clone::clone) + i_do_not_exist.with(Clone::clone) > 100.0);

    a
}

#[wasm_bindgen(auto_rename)]
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

#[wasm_bindgen(auto_rename)]
impl RustStruct {
    // methods

    pub fn increment_foo(&mut self, amount: Option<u32>) {}
    pub fn set_foo(&mut self, foo: u32) {}

    #[wasm_bindgen(js_name = get_another)]
    pub fn another(&self) -> u32 {
        self.another_field
    }

    pub fn static_method(a: u32) {}

    pub fn IHaveA_funky_name(a: u32) {}

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
