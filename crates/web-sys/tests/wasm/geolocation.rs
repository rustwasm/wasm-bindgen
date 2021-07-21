use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_accessor_works() {
    let window = web_sys::window().unwrap();
    assert!(window.navigator().geolocation().is_ok());
}
