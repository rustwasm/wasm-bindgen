use std::f64::{INFINITY, NAN};

use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use js_sys::*;

#[wasm_bindgen_test]
fn is_finite() {
    assert!(Number::is_finite(&42.into()));
    assert!(Number::is_finite(&42.1.into()));
    assert!(!Number::is_finite(&"42".into()));
    assert!(!Number::is_finite(&NAN.into()));
    assert!(!Number::is_finite(&INFINITY.into()));
}

#[wasm_bindgen_test]
fn is_integer() {
    assert!(Number::is_integer(&42.into()));
    assert!(!Number::is_integer(&42.1.into()));
}

#[wasm_bindgen_test]
fn is_nan() {
    assert!(Number::is_nan(&NAN.into()));

    assert!(!Number::is_nan(&JsValue::TRUE));
    assert!(!Number::is_nan(&JsValue::NULL));
    assert!(!Number::is_nan(&37.into()));
    assert!(!Number::is_nan(&"37".into()));
    assert!(!Number::is_nan(&"37.37".into()));
    assert!(!Number::is_nan(&"".into()));
    assert!(!Number::is_nan(&" ".into()));

    // These would all return true with the global isNaN()
    assert!(!Number::is_nan(&"NaN".into()));
    assert!(!Number::is_nan(&JsValue::UNDEFINED));
    assert!(!Number::is_nan(&"blabla".into()));
}

#[wasm_bindgen_test]
fn is_safe_integer() {
    assert_eq!(Number::is_safe_integer(&42.into()), true);
    assert_eq!(Number::is_safe_integer(&(Math::pow(2., 53.) - 1.).into()), true);
    assert_eq!(Number::is_safe_integer(&Math::pow(2., 53.).into()), false);
    assert_eq!(Number::is_safe_integer(&"42".into()), false);
    assert_eq!(Number::is_safe_integer(&42.1.into()), false);
    assert_eq!(Number::is_safe_integer(&NAN.into()), false);
    assert_eq!(Number::is_safe_integer(&INFINITY.into()), false);
}

#[wasm_bindgen_test]
fn new() {
    let n = Number::new(&JsValue::from(42));
    let v = JsValue::from(n);
    assert!(v.is_object());
    assert_eq!(Number::from(v).value_of(), 42.);
}

#[wasm_bindgen_test]
fn parse_int_float() {
    assert_eq!(Number::parse_int("42", 10), 42.);
    assert_eq!(Number::parse_int("42", 16), 66.); // 0x42 == 66
    assert!(Number::parse_int("invalid int", 10).is_nan());

    assert_eq!(Number::parse_float("123456.789"), 123456.789);
    assert!(Number::parse_float("invalid float").is_nan());
}

#[wasm_bindgen_test]
fn to_locale_string() {
    let number = Number::new(&1234.45.into());
    assert_eq!(number.to_locale_string("en-US"), "1,234.45");
    // TODO: these tests seems to be system dependent, disable for now
    // assert_eq!(wasm.to_locale_string(number, "de-DE"), "1,234.45");
    // assert_eq!(wasm.to_locale_string(number, "zh-Hans-CN-u-nu-hanidec"), "1,234.45");
}

#[wasm_bindgen_test]
fn to_precision() {
    assert_eq!(Number::new(&0.1.into()).to_precision(3).unwrap(), "0.100");
    assert!(Number::new(&10.into()).to_precision(101).is_err());
}

#[wasm_bindgen_test]
fn to_string() {
    assert_eq!(Number::new(&42.into()).to_string(10).unwrap(), "42");
    assert_eq!(Number::new(&233.into()).to_string(16).unwrap(), "e9");
    assert!(Number::new(&100.into()).to_string(100).is_err());
}

#[wasm_bindgen_test]
fn value_of() {
    assert_eq!(Number::new(&42.into()).value_of(), 42.);
}

#[wasm_bindgen_test]
fn to_fixed() {
    assert_eq!(Number::new(&123.456.into()).to_fixed(2).unwrap(), "123.46");
    assert!(Number::new(&10.into()).to_fixed(101).is_err());
}

#[wasm_bindgen_test]
fn to_exponential() {
    assert_eq!(Number::new(&123456.into()).to_exponential(2).unwrap(), "1.23e+5");
    assert!(Number::new(&10.into()).to_exponential(101).is_err());
}
