use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;
use js_sys::*;

macro_rules! each {
    ($m:ident) => (
        $m!(Uint8Array);
        $m!(Uint8ClampedArray);
        $m!(Uint16Array);
        $m!(Uint32Array);
        $m!(Int8Array);
        $m!(Int16Array);
    )
}

macro_rules! test_inheritence {
    ($arr:ident) => ({
        let arr = $arr::new(&JsValue::undefined());
        assert!(arr.is_instance_of::<$arr>());
        assert!(arr.is_instance_of::<Object>());        
    })
}
#[wasm_bindgen_test]
fn inheritence() {
    each!(test_inheritence);
}

macro_rules! each {
    ($m:ident) => (
        $m!(Uint8Array);
        $m!(Uint8ClampedArray);
        $m!(Uint16Array);
        $m!(Uint32Array);
        $m!(Int8Array);
        $m!(Int16Array);
        $m!(Int32Array);
        $m!(Float32Array);
        $m!(Float64Array);
    )
}

macro_rules! test_undefined {
    ($arr:ident) => ({
        let arr = $arr::new(&JsValue::undefined());
        assert_eq!(arr.length(), 0);
        assert_eq!(arr.byte_length(), 0);
        assert_eq!(arr.byte_offset(), 0);
        assert!(JsValue::from(arr.buffer()).is_object());
    })
}
#[wasm_bindgen_test]
fn new_undefined() {
    each!(test_undefined);
}

macro_rules! test_length {
    ($arr:ident) => ({
        let arr = $arr::new(&4.into());
        assert_eq!(arr.length(), 4);
        assert!(arr.byte_length() != 0);
        assert_eq!(arr.byte_offset(), 0);
        assert!(JsValue::from(arr.buffer()).is_object());
    })
}
#[wasm_bindgen_test]
fn new_length() {
    each!(test_length);
}

macro_rules! test_subarray {
    ($arr:ident) => ({
        assert_eq!($arr::new(&4.into()).subarray(0, 1).length(), 1);
    })
}
#[wasm_bindgen_test]
fn new_subarray() {
    each!(test_subarray);
}

macro_rules! test_fill {
    ($arr:ident) => ({
        let arr = $arr::new(&4.into());
        arr.for_each(&mut |x, _, _| {
            assert_eq!(x as f64, 0.0);
        });
        arr.fill(2 as _, 0, 2);
        arr.for_each(&mut |x, i, _| {
            if i < 2 {
                assert_eq!(x as f64, 2.0);
            } else {
                assert_eq!(x as f64, 0.0);
            }
        });
    })
}
#[wasm_bindgen_test]
fn new_fill() {
    each!(test_fill);
}
