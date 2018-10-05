use wasm_bindgen_test::*;
use wasm_bindgen::{JsValue, JsCast, Clamped};
use js_sys::{Uint8Array, Uint8ClampedArray};

#[wasm_bindgen_test]
fn numbers() {
    // integers
    assert_eq!(JsValue::from_wasm_abi(2i8), 2);
    assert_eq!(JsValue::from_wasm_abi(3u8), 3);
    assert_eq!(JsValue::from_wasm_abi(4i16), 4);
    assert_eq!(JsValue::from_wasm_abi(5u16), 5);
    assert_eq!(JsValue::from_wasm_abi(6i32), 6);
    assert_eq!(JsValue::from_wasm_abi(6i32), 6);
    assert_eq!(JsValue::from_wasm_abi(8isize), 8);
    assert_eq!(JsValue::from_wasm_abi(9usize), 9);

    // floats
    assert_eq!(JsValue::from_wasm_abi(10.0f32), 10);
    assert_eq!(JsValue::from_wasm_abi(11.0f64), 11);

    // optional integers
    assert_eq!(JsValue::from_wasm_abi(None::<i8>), JsValue::undefined());
    assert_eq!(JsValue::from_wasm_abi(Some(12i8)), 12);
    assert_eq!(JsValue::from_wasm_abi(None::<u8>), JsValue::undefined());
    assert_eq!(JsValue::from_wasm_abi(Some(13u8)), 13);
    assert_eq!(JsValue::from_wasm_abi(None::<i16>), JsValue::undefined());
    assert_eq!(JsValue::from_wasm_abi(Some(14i16)), 14);
    assert_eq!(JsValue::from_wasm_abi(None::<u16>), JsValue::undefined());
    assert_eq!(JsValue::from_wasm_abi(Some(15u16)), 15);
    assert_eq!(JsValue::from_wasm_abi(None::<i32>), JsValue::undefined());
    assert_eq!(JsValue::from_wasm_abi(Some(16i32)), 16);
    assert_eq!(JsValue::from_wasm_abi(None::<u32>), JsValue::undefined());
    assert_eq!(JsValue::from_wasm_abi(Some(17u32)), 17);
    assert_eq!(JsValue::from_wasm_abi(None::<f32>), JsValue::undefined());
    assert_eq!(JsValue::from_wasm_abi(Some(18.0f32)), 18);
    assert_eq!(JsValue::from_wasm_abi(None::<f64>), JsValue::undefined());
    assert_eq!(JsValue::from_wasm_abi(Some(19.0f64)), 19);
}

#[wasm_bindgen_test]
fn strings() {
    assert_eq!(JsValue::from_wasm_abi("foo"), "foo");
    assert_eq!(JsValue::from_wasm_abi(String::from("bar")), "bar");
}

#[wasm_bindgen_test]
fn slices() {
    let x: &[u8] = &[1];
    let array = JsValue::from_wasm_abi(x)
        .dyn_into::<Uint8Array>()
        .unwrap();
    assert_eq!(array.length(), 1);
    JsValue::from_wasm_abi(Clamped(x))
        .dyn_into::<Uint8ClampedArray>()
        .unwrap();
}
