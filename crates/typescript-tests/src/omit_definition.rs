use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TYPE_GET_VALUE: &'static str =
    "export function take_function(func: (x: number) => void): void;";

#[wasm_bindgen(skip_typescript)]
pub fn take_function(_: js_sys::Function) {}

#[wasm_bindgen]
pub struct MyExportedStruct {
    #[wasm_bindgen(skip_typescript)]
    pub field: bool,
}

#[wasm_bindgen]
impl MyExportedStruct {
    #[wasm_bindgen(skip_typescript)]
    pub fn method(&mut self) {
        self.field = !self.field;
    }

    #[wasm_bindgen(skip_typescript)]
    pub fn static_func() {
        panic!("oh no!");
    }
}

#[wasm_bindgen(skip_typescript)]
pub enum MyEnum {
    One,
    Two,
    Three,
}

#[wasm_bindgen(skip_typescript)]
pub struct MyStruct {
    pub field: i32,
}

macro_rules! generate_ts {
    ($lit:literal) => {
        #[wasm_bindgen(typescript_custom_section)]
        const _: &str = $lit;
    };
}

generate_ts!("");
