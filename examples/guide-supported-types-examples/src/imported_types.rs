use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum NumberEnum {
    Foo = 0,
    Bar = 1,
    Qux = 2,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum StringEnum {
    Foo = "foo",
    Bar = "bar",
    Qux = "qux",
}

#[wasm_bindgen]
pub struct Struct {
    pub number: NumberEnum,
    pub string: StringEnum,
}

#[wasm_bindgen]
extern "C" {
    pub type SomeJsType;
}

#[wasm_bindgen]
pub fn imported_type_by_value(x: SomeJsType) {
    /* ... */
}

#[wasm_bindgen]
pub fn imported_type_by_shared_ref(x: &SomeJsType) {
    /* ... */
}

#[wasm_bindgen]
pub fn return_imported_type() -> SomeJsType {
    unimplemented!()
}

#[wasm_bindgen]
pub fn take_option_imported_type(x: Option<SomeJsType>) {
    /* ... */
}

#[wasm_bindgen]
pub fn return_option_imported_type() -> Option<SomeJsType> {
    unimplemented!()
}
