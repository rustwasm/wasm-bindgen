#![allow(non_snake_case)]

use super::project;

#[test]
fn test() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::{ArrayBuffer, DataView};

            #[wasm_bindgen]
            pub fn test_data_view(buffer: &ArrayBuffer, offset: usize, len: usize) {
                let v = DataView::new(buffer, offset, len);
                assert_eq!(v.byte_offset(), offset);
                assert_eq!(v.byte_length(), len);
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
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const bytes = new Int8Array(10);
                bytes[2] = 2;
                wasm.test_data_view(bytes.buffer, 2, 8);
                assert.equal(bytes[2], 42);
            }
        "#)
        .test()
}
