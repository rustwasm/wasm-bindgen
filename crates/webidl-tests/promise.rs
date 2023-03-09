use crate::generated::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn return_promise() {
    let f = TestPromises::new().unwrap();
    let v = JsFuture::from(f.string_promise())
        .await
        .unwrap()
        .as_string()
        .unwrap();
    assert_eq!(v, "abc");
}
