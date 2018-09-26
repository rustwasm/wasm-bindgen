use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use web_sys::Response;

#[wasm_bindgen(module = "./tests/wasm/response.js")]
extern "C" {
    fn new_response() -> Response;
}

#[wasm_bindgen_test]
fn test_response() {
    let response = new_response();
    assert!(!response.ok());
    assert!(!response.redirected());
    assert_eq!(response.status(), 501);
}
