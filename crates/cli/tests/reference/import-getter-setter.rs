use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Bar, getter = bar)]
    fn get_foo() -> u32;
    #[wasm_bindgen(js_namespace = Bar, setter = bar)]
    fn set_foo(value: u32);
    #[wasm_bindgen(js_namespace = Bar, getter, js_name = bar2)]
    fn get_foo2() -> u32;
    #[wasm_bindgen(js_namespace = Bar, setter, js_name = bar2)]
    fn set_foo2(value: u32);

    #[wasm_bindgen]
    #[derive(Debug, Clone, PartialEq)]
    pub type SomeClass;

    #[wasm_bindgen(method, getter, js_class = "SomeClass", js_name = signal)]
    pub fn signal(this: &SomeClass) -> u32;
    #[wasm_bindgen(method, setter, js_class = "SomeClass", js_name = signal)]
    pub fn set_signal(this: &SomeClass, value: u32);

    #[wasm_bindgen(method, getter, js_class = "SomeClass")]
    pub fn some_prop(this: &SomeClass) -> u32;
    #[wasm_bindgen(method, setter, js_class = "SomeClass")]
    pub fn set_some_prop(this: &SomeClass, value: u32);

    #[wasm_bindgen(method, getter = prop2, js_class = "SomeClass")]
    pub fn another(this: &SomeClass) -> u32;
    #[wasm_bindgen(method, setter = prop2, js_class = "SomeClass")]
    pub fn set_another(this: &SomeClass, value: u32);

    // #[wasm_bindgen(getter, js_class = "SomeClass")]
    // pub fn static_controller() -> SomeClass;
    // #[wasm_bindgen(getter, js_class = "SomeClass")]
    // pub fn set_static_controller(value: &SomeClass);

    #[wasm_bindgen(constructor, js_class = "SomeClass")]
    pub fn new() -> SomeClass;

    // js_name conflicts with the getter/setter name
    #[wasm_bindgen(method, getter = a, js_class = "SomeClass", js_name = b)]
    pub fn c(this: &SomeClass) -> u32;
    #[wasm_bindgen(method, setter = a, js_class = "SomeClass", js_name = b)]
    pub fn set_c(this: &SomeClass, value: u32);
}

#[wasm_bindgen]
pub fn exported() {
    set_foo(get_foo());
    set_foo2(get_foo2());

    let a = SomeClass::new();
    a.set_signal(a.signal());
    a.set_some_prop(a.some_prop());
    a.set_another(a.another());
    a.set_c(a.c());
    // let _ = static_signal();
}
