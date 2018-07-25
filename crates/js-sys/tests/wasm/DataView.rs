use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use js_sys::*;

#[wasm_bindgen_test]
fn test() {
    let bytes = Int8Array::new(&JsValue::from(10));

    // TODO: figure out how to do `bytes[2] = 2`
    bytes.subarray(2, 3).fill(2, 0, 1);

    let v = DataView::new(&bytes.buffer(), 2, 8);
    assert_eq!(v.byte_offset(), 2);
    assert_eq!(v.byte_length(), 8);
    assert_eq!(v.get_int8(0), 2);
    assert_eq!(v.get_uint8(0), 2);

    v.set_int8(0, 42);
    assert_eq!(v.get_int8(0), 42);
    v.set_uint8(0, 255);
    assert_eq!(v.get_uint8(0), 255);
    v.set_int16(0, 32767);
    assert_eq!(v.get_int16(0), 32767);
    v.set_uint16(0, 65535);
    assert_eq!(v.get_uint16(0), 65535);
    v.set_int32(0, 123456789);
    assert_eq!(v.get_int32(0), 123456789);
    v.set_uint32(0, 3_123_456_789);
    assert_eq!(v.get_uint32(0), 3_123_456_789);
    v.set_float32(0, 100.123);
    assert_eq!(v.get_float32(0), 100.123);
    v.set_float64(0, 123456789.123456);
    assert_eq!(v.get_float64(0), 123456789.123456);

    v.set_int8(0, 42);

    // TODO: figure out how to do `bytes[2]`
    bytes.subarray(2, 3).for_each(&mut |x, _, _| assert_eq!(x, 42));
}
