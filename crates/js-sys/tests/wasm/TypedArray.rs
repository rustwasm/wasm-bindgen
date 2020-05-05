use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

macro_rules! each {
    ($m:ident) => {
        $m!(Uint8Array);
        $m!(Uint8ClampedArray);
        $m!(Uint16Array);
        $m!(Uint32Array);
        $m!(Int8Array);
        $m!(Int16Array);
        $m!(Int32Array);
        $m!(Float32Array);
        $m!(Float64Array);
    };
}

macro_rules! test_inheritence {
    ($arr:ident) => {{
        let arr = $arr::new(&JsValue::undefined());
        assert!(arr.is_instance_of::<$arr>());
        let _: &Object = arr.as_ref();
        assert!(arr.is_instance_of::<Object>());
    }};
}
#[wasm_bindgen_test]
fn inheritence() {
    each!(test_inheritence);
}

macro_rules! test_undefined {
    ($arr:ident) => {{
        let arr = $arr::new(&JsValue::undefined());
        assert_eq!(arr.length(), 0);
        assert_eq!(arr.byte_length(), 0);
        assert_eq!(arr.byte_offset(), 0);
        assert!(JsValue::from(arr.buffer()).is_object());
    }};
}
#[wasm_bindgen_test]
fn new_undefined() {
    each!(test_undefined);
}

macro_rules! test_length {
    ($arr:ident) => {{
        let arr = $arr::new(&4.into());
        assert_eq!(arr.length(), 4);
        assert!(arr.byte_length() != 0);
        assert_eq!(arr.byte_offset(), 0);
        assert!(JsValue::from(arr.buffer()).is_object());
    }};
}
#[wasm_bindgen_test]
fn new_length() {
    each!(test_length);
}

macro_rules! test_subarray {
    ($arr:ident) => {{
        assert_eq!($arr::new(&4.into()).subarray(0, 1).length(), 1);
    }};
}
#[wasm_bindgen_test]
fn new_subarray() {
    each!(test_subarray);
}

macro_rules! test_fill {
    ($arr:ident) => {{
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
    }};
}
#[wasm_bindgen_test]
fn new_fill() {
    each!(test_fill);
}

macro_rules! test_get_set {
    ($arr:ident) => {{
        let arr = $arr::new(&1.into());
        assert_eq!(arr.get_index(0) as f64, 0 as f64);
        arr.set_index(0, 1 as _);
        assert_eq!(arr.get_index(0) as f64, 1 as f64);
    }};
}
#[wasm_bindgen_test]
fn new_get_set() {
    each!(test_get_set);
}

macro_rules! test_slice {
    ($arr:ident) => {{
        let arr = $arr::new(&4.into());
        assert_eq!(arr.length(), 4);
        assert_eq!(arr.slice(1, 2).length(), 1);
    }};
}
#[wasm_bindgen_test]
fn new_slice() {
    each!(test_slice);
}

#[wasm_bindgen_test]
fn view() {
    let x = [1, 2, 3];
    let array = unsafe { Int32Array::view(&x) };
    assert_eq!(array.length(), 3);
    array.for_each(&mut |x, i, _| {
        assert_eq!(x, (i + 1) as i32);
    });
}

#[wasm_bindgen_test]
fn from() {
    let x: Vec<i32> = vec![1, 2, 3];
    let array = Int32Array::from(x.as_slice());
    assert_eq!(array.length(), 3);
    array.for_each(&mut |x, i, _| {
        assert_eq!(x, (i + 1) as i32);
    });
}

#[wasm_bindgen_test]
fn copy_to() {
    let mut x = [0; 10];
    let array = Int32Array::new(&10.into());
    array.fill(5, 0, 10);
    array.copy_to(&mut x);
    for i in x.iter() {
        assert_eq!(*i, 5);
    }
}

#[wasm_bindgen_test]
fn to_vec() {
    let array = Int32Array::new(&10.into());
    array.fill(5, 0, 10);
    assert_eq!(array.to_vec(), vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
}

#[wasm_bindgen(module = "tests/wasm/TypedArray.js")]
extern "C" {
    fn populate_array(arr: JsValue, start: JsValue, len: JsValue) -> JsValue;
}

fn test_array_view_mut_raw<ElemT: std::cmp::PartialEq + std::fmt::Debug, ArrT>(
    sut: unsafe fn(*mut ElemT, usize) -> ArrT,
    u8ToElem: fn(u8) -> ElemT,
    arrToJsValue: fn(ArrT) -> JsValue,
) {
    let start: u8 = 10;
    let len: usize = 32;
    let end: u8 = start + len as u8;
    let mut buffer: Vec<ElemT> = Vec::with_capacity(len);
    unsafe {
        buffer.set_len(len);
        let array: ArrT = sut(buffer.as_mut_ptr(), len);
        populate_array(
            arrToJsValue(array),
            JsValue::from(start),
            JsValue::from(len as u32),
        );
    }
    let expected: Vec<ElemT> = (start..end).map(u8ToElem).collect();
    assert_eq!(buffer, expected)
}

fn u8Toi8_unsafe(x: u8) -> i8 {
    x as i8
}

#[wasm_bindgen_test]
fn Int8Array_view_mut_raw() {
    test_array_view_mut_raw(
        js_sys::Int8Array::view_mut_raw,
        u8Toi8_unsafe,
        JsValue::from,
    );
}

#[wasm_bindgen_test]
fn Int16Array_view_mut_raw() {
    test_array_view_mut_raw(js_sys::Int16Array::view_mut_raw, i16::from, JsValue::from);
}

#[wasm_bindgen_test]
fn Int32Array_view_mut_raw() {
    test_array_view_mut_raw(js_sys::Int32Array::view_mut_raw, i32::from, JsValue::from);
}

#[wasm_bindgen_test]
fn Uint8Array_view_mut_raw() {
    test_array_view_mut_raw(js_sys::Uint8Array::view_mut_raw, u8::from, JsValue::from);
}

#[wasm_bindgen_test]
fn Uint8ClampedArray_view_mut_raw() {
    test_array_view_mut_raw(
        js_sys::Uint8ClampedArray::view_mut_raw,
        u8::from,
        JsValue::from,
    );
}

#[wasm_bindgen_test]
fn Uint16Array_view_mut_raw() {
    test_array_view_mut_raw(js_sys::Uint16Array::view_mut_raw, u16::from, JsValue::from);
}

#[wasm_bindgen_test]
fn Uint32Array_view_mut_raw() {
    test_array_view_mut_raw(js_sys::Uint32Array::view_mut_raw, u32::from, JsValue::from);
}

#[wasm_bindgen_test]
fn Float32Array_view_mut_raw() {
    test_array_view_mut_raw(js_sys::Float32Array::view_mut_raw, f32::from, JsValue::from);
}

#[wasm_bindgen_test]
fn Float64Array_view_mut_raw() {
    test_array_view_mut_raw(js_sys::Float64Array::view_mut_raw, f64::from, JsValue::from);
}

fn test_js_array_copy_to_mem_raw<ElemT: std::cmp::PartialEq + std::fmt::Debug, JsArrT>(
    sut: fn(&JsArrT, *mut ElemT, usize) -> (),
    sliceToJsArray: fn(&[ElemT]) -> JsArrT,
    u8ToElem: fn(u8) -> ElemT,
) {
    let start: u8 = 10;
    let len: usize = 32;
    let end: u8 = start + len as u8;
    let expected: Vec<ElemT> = (start..end).map(u8ToElem).collect();

    // create typed js array with values
    let js_arr: JsArrT = sliceToJsArray(expected.as_slice());

    // allocate uninitialized Vec instance
    let mut vec: Vec<ElemT> = Vec::with_capacity(len);
    unsafe {
        vec.set_len(len);
    }

    // call the function under test
    sut(&js_arr, vec.as_mut_ptr(), len);

    // make sure js_arr values copied into vec
    assert_eq!(vec, expected);
}

#[wasm_bindgen_test]
fn Int8Array_copy_to_mem_raw() {
    test_js_array_copy_to_mem_raw(
        js_sys::Int8Array::copy_to_mem_raw,
        |xs: &[i8]| js_sys::Int8Array::from(xs),
        u8Toi8_unsafe,
    );
}

#[wasm_bindgen_test]
fn Int16Array_copy_to_mem_raw() {
    test_js_array_copy_to_mem_raw(
        js_sys::Int16Array::copy_to_mem_raw,
        |xs: &[i16]| js_sys::Int16Array::from(xs),
        i16::from,
    );
}

#[wasm_bindgen_test]
fn Int32Array_copy_to_mem_raw() {
    test_js_array_copy_to_mem_raw(
        js_sys::Int32Array::copy_to_mem_raw,
        |xs: &[i32]| js_sys::Int32Array::from(xs),
        i32::from,
    );
}

#[wasm_bindgen_test]
fn Uint8Array_copy_to_mem_raw() {
    test_js_array_copy_to_mem_raw(
        js_sys::Uint8Array::copy_to_mem_raw,
        |xs: &[u8]| js_sys::Uint8Array::from(xs),
        u8::from,
    );
}

#[wasm_bindgen_test]
fn Uint8ClampedArray_copy_to_mem_raw() {
    test_js_array_copy_to_mem_raw(
        js_sys::Uint8ClampedArray::copy_to_mem_raw,
        |xs: &[u8]| js_sys::Uint8ClampedArray::from(xs),
        u8::from,
    );
}

#[wasm_bindgen_test]
fn Uint16Array_copy_to_mem_raw() {
    test_js_array_copy_to_mem_raw(
        js_sys::Uint16Array::copy_to_mem_raw,
        |xs: &[u16]| js_sys::Uint16Array::from(xs),
        u16::from,
    );
}

#[wasm_bindgen_test]
fn Uint32Array_copy_to_mem_raw() {
    test_js_array_copy_to_mem_raw(
        js_sys::Uint32Array::copy_to_mem_raw,
        |xs: &[u32]| js_sys::Uint32Array::from(xs),
        u32::from,
    );
}

#[wasm_bindgen_test]
fn Float32Array_copy_to_mem_raw() {
    test_js_array_copy_to_mem_raw(
        js_sys::Float32Array::copy_to_mem_raw,
        |xs: &[f32]| js_sys::Float32Array::from(xs),
        f32::from,
    );
}

#[wasm_bindgen_test]
fn Float64Array_copy_to_mem_raw() {
    test_js_array_copy_to_mem_raw(
        js_sys::Float64Array::copy_to_mem_raw,
        |xs: &[f64]| js_sys::Float64Array::from(xs),
        f64::from,
    );
}
