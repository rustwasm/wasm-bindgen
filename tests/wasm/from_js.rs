use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests/wasm/from_js.js")]
extern "C" {
    fn get_f32_slice() -> JsValue;
}

#[wasm_bindgen_test]
fn numbers() {
    assert_eq!(JsValue::from(2).into_wasm_abi::<i8>().unwrap(), 2);
    assert_eq!(JsValue::from(3).into_wasm_abi::<u8>().unwrap(), 3);
    assert_eq!(JsValue::from(4).into_wasm_abi::<i16>().unwrap(), 4);
    assert_eq!(JsValue::from(5).into_wasm_abi::<u16>().unwrap(), 5);
    assert_eq!(JsValue::from(6).into_wasm_abi::<i32>().unwrap(), 6);
    assert_eq!(JsValue::from(7).into_wasm_abi::<u32>().unwrap(), 7);
    assert_eq!(JsValue::from(8).into_wasm_abi::<isize>().unwrap(), 8);
    assert_eq!(JsValue::from(9).into_wasm_abi::<usize>().unwrap(), 9);
    assert_eq!(JsValue::from(10).into_wasm_abi::<f32>().unwrap(), 10.0);
    assert_eq!(JsValue::from(11).into_wasm_abi::<f64>().unwrap(), 11.0);

    assert_eq!(JsValue::undefined().into_wasm_abi::<Option<i8>>().unwrap(), None);
    assert_eq!(JsValue::from(12).into_wasm_abi::<Option<i8>>().unwrap(), Some(12));
    assert_eq!(JsValue::undefined().into_wasm_abi::<Option<u8>>().unwrap(), None);
    assert_eq!(JsValue::from(13).into_wasm_abi::<Option<u8>>().unwrap(), Some(13));
    assert_eq!(JsValue::undefined().into_wasm_abi::<Option<i16>>().unwrap(), None);
    assert_eq!(JsValue::from(14).into_wasm_abi::<Option<i16>>().unwrap(), Some(14));
    assert_eq!(JsValue::undefined().into_wasm_abi::<Option<u16>>().unwrap(), None);
    assert_eq!(JsValue::from(15).into_wasm_abi::<Option<u16>>().unwrap(), Some(15));
    assert_eq!(JsValue::undefined().into_wasm_abi::<Option<i32>>().unwrap(), None);
    assert_eq!(JsValue::from(16).into_wasm_abi::<Option<i32>>().unwrap(), Some(16));
    assert_eq!(JsValue::undefined().into_wasm_abi::<Option<u32>>().unwrap(), None);
    assert_eq!(JsValue::from(17).into_wasm_abi::<Option<u32>>().unwrap(), Some(17));
    assert_eq!(JsValue::undefined().into_wasm_abi::<Option<isize>>().unwrap(), None);
    assert_eq!(JsValue::from(18).into_wasm_abi::<Option<isize>>().unwrap(), Some(18));
    assert_eq!(JsValue::undefined().into_wasm_abi::<Option<usize>>().unwrap(), None);
    assert_eq!(JsValue::from(19).into_wasm_abi::<Option<usize>>().unwrap(), Some(19));
    assert_eq!(JsValue::undefined().into_wasm_abi::<Option<f32>>().unwrap(), None);
    assert_eq!(JsValue::from(20).into_wasm_abi::<Option<f32>>().unwrap(), Some(20.0));
    assert_eq!(JsValue::undefined().into_wasm_abi::<Option<f64>>().unwrap(), None);
    assert_eq!(JsValue::from(21).into_wasm_abi::<Option<f64>>().unwrap(), Some(21.0));
}

#[wasm_bindgen_test]
fn strings() {
    assert_eq!(JsValue::from("foo").into_wasm_abi::<String>().unwrap(), "foo");
}

#[wasm_bindgen_test]
fn slices() {
    assert_eq!(get_f32_slice().into_wasm_abi::<Vec<f32>>().unwrap(), [1.0, 2.0, 3.0]);
}

#[wasm_bindgen_test]
fn numbers_bad() {
    assert!(JsValue::from(2).into_wasm_abi::<Vec<f32>>().is_err());
}
