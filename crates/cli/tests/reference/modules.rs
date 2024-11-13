use wasm_bindgen::prelude::*;

mod a {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = parseFloat)]
        pub fn parse_float(text: &JsValue) -> f64;
    }
}

mod b {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = parseFloat)]
        pub fn parse_float(text: &str) -> f64;
    }
}

mod a_again {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = parseFloat)]
        pub fn parse_float(text: &JsValue) -> f64;
    }
}

#[wasm_bindgen]
pub fn exported() {
    let _ = a::parse_float(&JsValue::from_str("3.14"));
    let _ = b::parse_float("3.14");
    let _ = a_again::parse_float(&JsValue::from_str("3.14"));
}
