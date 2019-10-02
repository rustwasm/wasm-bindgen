use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/getters_and_setters.js")]
extern "C" {
    fn _1_js(rules: WasmType<Rules>) -> WasmType<Rules>;
    fn _2_js(rules: WasmType<Rules>) -> WasmType<Rules>;
    fn _3_js(rules: WasmType<Rules>) -> WasmType<Rules>;
    fn _4_js(rules: WasmType<Rules>) -> WasmType<Rules>;
    fn _5_js(rules: WasmType<Rules>) -> WasmType<Rules>;
    fn _6_js(rules: WasmType<Rules>) -> WasmType<Rules>;
    fn _7_js(rules: WasmType<Rules>) -> WasmType<Rules>;

    fn test_getter_compute(x: WasmType<GetterCompute>);
    fn test_setter_compute(x: WasmType<SetterCompute>);
}

// Each getter/setter combination is derived
// from https://github.com/rustwasm/wasm-bindgen/pull/1440#issuecomment-487113564
#[wasm_bindgen]
pub struct Rules {
    pub field: i32,
}

#[wasm_bindgen]
#[allow(non_snake_case)]
impl Rules {
    #[wasm_bindgen(constructor)]
    pub fn new(field: i32) -> WasmType<Rules> {
        instantiate! { Rules { field } }
    }

    #[wasm_bindgen]
    pub fn no_js_name__no_getter_with_name__no_getter_without_name(&self) -> i32 {
        self.field
    }
    #[wasm_bindgen]
    pub fn set_no_js_name__no_setter_with_name__no_setter_without_name(&mut self, field: i32) {
        self.field = field;
    }

    #[wasm_bindgen(getter)]
    pub fn no_js_name__no_getter_with_name__getter_without_name(&self) -> i32 {
        self.field
    }
    #[wasm_bindgen(setter)]
    pub fn set_no_js_name__no_setter_with_name__setter_without_name(&mut self, field: i32) {
        self.field = field;
    }

    #[wasm_bindgen(getter = new_no_js_name__getter_with_name__getter_without_name)]
    pub fn no_js_name__getter_with_name__getter_without_name(&self) -> i32 {
        self.field
    }
    #[wasm_bindgen(setter = new_no_js_name__setter_with_name__setter_without_name)]
    pub fn set_no_js_name__setter_with_name__setter_without_name(&mut self, field: i32) {
        self.field = field;
    }

    #[wasm_bindgen(js_name = new_js_name__no_getter_with_name__no_getter_without_name)]
    pub fn js_name__no_getter_with_name__no_getter_without_name(&self) -> i32 {
        self.field
    }
    #[wasm_bindgen(js_name = new_js_name__no_setter_with_name__no_setter_without_name)]
    pub fn set_js_name__no_setter_with_name__no_setter_without_name(&mut self, field: i32) {
        self.field = field;
    }

    #[wasm_bindgen(getter, js_name = new_js_name__no_getter_with_name__getter_without_name)]
    pub fn js_name__no_getter_with_name__getter_without_name(&self) -> i32 {
        self.field
    }
    #[wasm_bindgen(js_name = new_js_name__no_setter_with_name__setter_without_name, setter)]
    pub fn set_js_name__no_setter_with_name__setter_without_name(&mut self, field: i32) {
        self.field = field;
    }

    #[wasm_bindgen(
        getter = new_js_name__getter_with_name__no_getter_without_name_for_field,
        js_name = new_js_name__getter_with_name__no_getter_without_name_for_method
    )]
    pub fn js_name__getter_with_name__no_getter_without_name(&self) -> i32 {
        self.field
    }
    #[wasm_bindgen(
        js_name = new_js_name__setter_with_name__no_setter_without_name_for_method,
        setter = new_js_name__setter_with_name__no_setter_without_name_for_field
    )]
    pub fn set_js_name__setter_with_name__no_setter_without_name_for_field(&mut self, field: i32) {
        self.field = field;
    }
}

#[wasm_bindgen_test]
fn _1_rust() {
    let rules = _1_js(Rules::new(1));
    assert_eq!(rules.borrow().field, 2);
}

#[wasm_bindgen_test]
fn _2_rust() {
    let rules = _2_js(Rules::new(2));
    assert_eq!(rules.borrow().field, 4);
}

#[wasm_bindgen_test]
fn _3_rust() {
    let rules = _3_js(Rules::new(3));
    assert_eq!(rules.borrow().field, 6);
}

#[wasm_bindgen_test]
fn _4_rust() {
    let rules = _4_js(Rules::new(4));
    assert_eq!(rules.borrow().field, 8);
}

#[wasm_bindgen_test]
fn _5_rust() {
    let rules = _5_js(Rules::new(5));
    assert_eq!(rules.borrow().field, 10);
}

#[wasm_bindgen_test]
fn _6_rust() {
    let rules = _6_js(Rules::new(6));
    assert_eq!(rules.borrow().field, 12);
}

#[wasm_bindgen_test]
fn _7_rust() {
    let rules = _7_js(Rules::new(7));
    assert_eq!(rules.borrow().field, 14);
}

#[wasm_bindgen]
pub struct GetterCompute;

#[wasm_bindgen]
impl GetterCompute {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<GetterCompute> {
        instantiate! { GetterCompute }
    }

    #[wasm_bindgen(getter)]
    pub fn foo(&self) -> u32 {
        3
    }
}

#[wasm_bindgen_test]
fn getter_compute() {
    test_getter_compute(GetterCompute::new());
}

#[wasm_bindgen]
pub struct SetterCompute(Rc<Cell<u32>>);

#[wasm_bindgen]
impl SetterCompute {
    #[wasm_bindgen(constructor)]
    pub fn new(val: u32) -> WasmType<SetterCompute> {
        instantiate! { SetterCompute(Rc::new(Cell::new(val))) }
    }
}

#[wasm_bindgen]
impl SetterCompute {
    #[wasm_bindgen(setter)]
    pub fn set_foo(&self, x: u32) {
        self.0.set(x + 3);
    }
}

#[wasm_bindgen_test]
fn setter_compute() {
    let setter_compute = SetterCompute::new(3);
    let r = setter_compute.borrow().0.clone();
    test_setter_compute(setter_compute);
    assert_eq!(r.get(), 100);
}
