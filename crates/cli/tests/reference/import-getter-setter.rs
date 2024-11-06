use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Foo, getter = bar)]
    fn get_foo() -> u32;
    #[wasm_bindgen(js_namespace = Foo, setter = bar)]
    fn set_foo(value: u32);
    #[wasm_bindgen(js_namespace = Foo, getter, js_name = bar2)]
    fn get_foo2() -> u32;
    #[wasm_bindgen(js_namespace = Foo, setter, js_name = bar2)]
    fn set_foo2(value: u32);

    #[wasm_bindgen]
    #[derive(Debug, Clone, PartialEq)]
    pub type SomeClass;
    #[wasm_bindgen(method, getter, js_class = "SomeClass", js_name = signal)]
    pub fn signal(this: &SomeClass) -> u32;
    #[wasm_bindgen(method, setter, js_class = "SomeClass", js_name = signal)]
    pub fn set_signal(this: &SomeClass, value: u32);
    // #[wasm_bindgen(getter, js_class = "SomeClass", js_name = static_signal)]
    // pub fn static_controller() -> SomeClass;
    #[wasm_bindgen(constructor, js_class = "SomeClass")]
    pub fn new() -> SomeClass;
}

#[wasm_bindgen]
pub fn exported() {
    set_foo(get_foo());
    set_foo2(get_foo2());

    let a = SomeClass::new();
    a.set_signal(a.signal());
    // let _ = static_signal();
}
