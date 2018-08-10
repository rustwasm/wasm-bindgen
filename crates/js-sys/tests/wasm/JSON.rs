use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;
use js_sys::*;

#[wasm_bindgen_test]
fn parse_array() {

    let js_array = JSON::parse("[1, 2, 3]").unwrap();;
    assert!(Array::is_array(&js_array));

    let array = Array::from(&js_array);
    assert_eq!(array.length(), 3);
    assert_eq!(array.pop(), 3);
    assert_eq!(array.pop(), 2);
    assert_eq!(array.pop(), 1);

}

#[wasm_bindgen_test]
fn parse_object() {

    let js_object = JSON::parse("{\"x\": 5, \"y\": true, \"z\": [\"foo\", \"bar\"]}").unwrap();
    assert!(js_object.is_object());

    let obj = Object::from(js_object);
    let keys = Object::keys(&obj);
    assert_eq!(keys.length(), 3);
    assert_eq!(keys.pop().as_string().unwrap(), "z");
    assert_eq!(keys.pop().as_string().unwrap(), "y");
    assert_eq!(keys.pop().as_string().unwrap(), "x");

    let values = Object::values(&obj);
    assert_eq!(values.length(), 3);

    let z = values.pop();
    assert!(Array::is_array(&z));
    let z_array = Array::from(&z);
    assert_eq!(z_array.length(), 2);

    let y = values.pop();
    assert_eq!(y.as_bool(), Some(true));

    let x = values.pop();
    assert!(Number::is_integer(&x));
    let x_num = Number::new(&x);
    assert_eq!(x_num.value_of(), 5.0);

}

#[wasm_bindgen_test]
fn parse_error() {
    let js_object = JSON::parse("invalid json");
    assert!(js_object.is_err());
    let err = js_object.unwrap_err();
    assert!(err.is_instance_of::<Error>());
}

#[wasm_bindgen_test]
fn stringify() {
    let arr = Array::new();
    arr.push(&JsValue::from(1));
    arr.push(&JsValue::from(true));
    arr.push(&JsValue::from("hello"));

    let str = JSON::stringify(&JsValue::from(arr));
    let rust_str: String = From::from(str);
    assert_eq!(rust_str, "[1,true,\"hello\"]");
}