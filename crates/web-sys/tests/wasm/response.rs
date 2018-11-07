extern crate futures;
extern crate js_sys;
extern crate wasm_bindgen_futures;

use futures::Future;
use js_sys::{ArrayBuffer, DataView};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::Response;

#[wasm_bindgen(module = "./tests/wasm/response.js")]
extern "C" {
    fn new_response() -> Response;
}

#[wasm_bindgen_test]
fn test_response_from_js() {
    let response = new_response();
    assert!(!response.ok());
    assert!(!response.redirected());
    assert_eq!(response.status(), 501);
}

#[wasm_bindgen_test(async)]
fn test_response_from_bytes() -> impl Future<Item = (), Error = JsValue> {
    let mut bytes: [u8; 3] = [1, 3, 5];
    let response = Response::new_with_opt_u8_array(Some(&mut bytes)).unwrap();
    assert!(response.ok());
    assert_eq!(response.status(), 200);

    let buf_promise = response.array_buffer().unwrap();
    JsFuture::from(buf_promise).map(move |buf_val| {
        assert!(buf_val.is_instance_of::<ArrayBuffer>());
        let array_buf: ArrayBuffer = buf_val.dyn_into().unwrap();
        let data_view = DataView::new(&array_buf, 0, bytes.len());
        for (i, byte) in bytes.iter().enumerate() {
            assert_eq!(&data_view.get_uint8(i), byte);
        }
    })
}
