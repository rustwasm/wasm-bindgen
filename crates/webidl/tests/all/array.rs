use super::project;

#[test]
fn take_and_return_a_bunch_of_slices() {
    project()
        .file(
            "foo.webidl",
            r#"
                [Constructor()]
                interface Foo {
                    DOMString strings(DOMString arg1);
                    ByteString byteStrings(ByteString arg1);
                    USVString usvStrings(USVString arg1);

                    Float32Array f32(Float32Array a);
                    Float64Array f64(Float64Array a);
                    Int8Array i8(Int8Array a);
                    Int16Array i16(Int16Array a);
                    Int32Array i32(Int32Array a);
                    Uint8Array u8(Uint8Array a);
                    Uint8ClampedArray u8Clamped(Uint8ClampedArray a);
                    Uint16Array u16(Uint16Array a);
                    Uint32Array u32(Uint32Array a);
                };
            "#,
        )
        .file(
            "foo.js",
            r#"
                import { strictEqual } from "assert";

                export class Foo {
                    strings(x) {
                        strictEqual(x, 'y');
                        return 'x';
                    }
                    byteStrings(x) {
                        strictEqual(x, 'yz');
                        return 'xx';
                    }
                    usvStrings(x) {
                        strictEqual(x, 'abc');
                        return 'efg';
                    }
                    f32(x) {
                        strictEqual(x.length, 2);
                        strictEqual(x[0], 1);
                        strictEqual(x[1], 2);
                        return new Float32Array([3, 4, 5]);
                    }
                    f64(x) {
                        strictEqual(x.length, 2);
                        strictEqual(x[0], 1);
                        strictEqual(x[1], 2);
                        return new Float64Array([3, 4, 5]);
                    }
                    i8(x) {
                        strictEqual(x.length, 2);
                        strictEqual(x[0], 1);
                        strictEqual(x[1], 2);
                        return new Int8Array([3, 4, 5]);
                    }
                    i16(x) {
                        strictEqual(x.length, 2);
                        strictEqual(x[0], 1);
                        strictEqual(x[1], 2);
                        return new Int16Array([3, 4, 5]);
                    }
                    i32(x) {
                        strictEqual(x.length, 2);
                        strictEqual(x[0], 1);
                        strictEqual(x[1], 2);
                        return new Int32Array([3, 4, 5]);
                    }
                    u8(x) {
                        strictEqual(x.length, 2);
                        strictEqual(x[0], 1);
                        strictEqual(x[1], 2);
                        return new Uint8Array([3, 4, 5]);
                    }
                    u8Clamped(x) {
                        strictEqual(x.length, 2);
                        strictEqual(x[0], 1);
                        strictEqual(x[1], 2);
                        return new Uint8ClampedArray([3, 4, 5]);
                    }
                    u16(x) {
                        strictEqual(x.length, 2);
                        strictEqual(x[0], 1);
                        strictEqual(x[1], 2);
                        return new Uint16Array([3, 4, 5]);
                    }
                    u32(x) {
                        strictEqual(x.length, 2);
                        strictEqual(x[0], 1);
                        strictEqual(x[1], 2);
                        return new Uint32Array([3, 4, 5]);
                    }
                }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                pub mod foo;

                use foo::Foo;

                #[wasm_bindgen]
                pub fn test() {
                    let f = Foo::new().unwrap();
                    assert_eq!(f.strings("y"), "x");
                    assert_eq!(f.byte_strings("yz"), "xx");
                    assert_eq!(f.usv_strings("abc"), "efg");
                    assert_eq!(f.f32(&[1.0, 2.0]), [3.0, 4.0, 5.0]);
                    assert_eq!(f.f64(&[1.0, 2.0]), [3.0, 4.0, 5.0]);
                    assert_eq!(f.i8(&[1, 2]), [3, 4, 5]);
                    assert_eq!(f.i16(&[1, 2]), [3, 4, 5]);
                    assert_eq!(f.i32(&[1, 2]), [3, 4, 5]);
                    assert_eq!(f.u8(&[1, 2]), [3, 4, 5]);
                    assert_eq!(f.u8_clamped(&[1, 2]), [3, 4, 5]);
                    assert_eq!(f.u16(&[1, 2]), [3, 4, 5]);
                    assert_eq!(f.u32(&[1, 2]), [3, 4, 5]);
                }
            "#,
        )
        .test();
}
