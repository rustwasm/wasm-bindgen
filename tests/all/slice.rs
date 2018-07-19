use super::project;

#[test]
fn export() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                macro_rules! doit {
                    ($($i:ident)*) => ($(
                        #[wasm_bindgen]
                        pub fn $i(a: &[$i]) -> Vec<$i> {
                            assert_eq!(a.len(), 2);
                            assert_eq!(a[0], 1 as $i);
                            assert_eq!(a[1], 2 as $i);
                            a.to_vec()
                        }
                    )*)
                }


                doit! { i8 u8 i16 u16 i32 u32 f32 f64 }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                function assert_arrays_equal(a, b) {
                    console.log(a, b);
                    assert.strictEqual(a.length, b.length);
                    assert.strictEqual(a.byteLength, b.byteLength);
                    for (let i = 0; i < a.length; i++) {
                        assert.strictEqual(a[i], b[i]);
                    }
                }

                export function test() {
                    const i8 = new Int8Array(2);
                    i8[0] = 1;
                    i8[1] = 2;
                    assert_arrays_equal(wasm.i8(i8), i8);
                    const u8 = new Uint8Array(2);
                    u8[0] = 1;
                    u8[1] = 2;
                    assert_arrays_equal(wasm.u8(u8), u8);

                    const i16 = new Int16Array(2);
                    i16[0] = 1;
                    i16[1] = 2;
                    assert_arrays_equal(wasm.i16(i16), i16);
                    const u16 = new Uint16Array(2);
                    u16[0] = 1;
                    u16[1] = 2;
                    assert_arrays_equal(wasm.u16(u16), u16);

                    const i32 = new Int32Array(2);
                    i32[0] = 1;
                    i32[1] = 2;
                    wasm.i32(i32);
                    assert_arrays_equal(wasm.i32(i32), i32);
                    const u32 = new Uint32Array(2);
                    u32[0] = 1;
                    u32[1] = 2;
                    assert_arrays_equal(wasm.u32(u32), u32);

                    const f32 = new Float32Array(2);
                    f32[0] = 1;
                    f32[1] = 2;
                    assert_arrays_equal(wasm.f32(f32), f32);
                    const f64 = new Float64Array(2);
                    f64[0] = 1;
                    f64[1] = 2;
                    assert_arrays_equal(wasm.f64(f64), f64);
                }
            "#,
        )
        .test();
}

#[test]
fn import() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                macro_rules! doit {
                    ($(($rust:ident, $js:ident, $i:ident))*) => ($(
                        #[wasm_bindgen(module = "./test")]
                        extern {
                            fn $js(a: &[$i]) -> Vec<$i>;
                        }

                        #[wasm_bindgen]
                        pub fn $rust(a: &[$i]) -> Vec<$i> {
                            assert_eq!(a.len(), 2);
                            assert_eq!(a[0], 1 as $i);
                            assert_eq!(a[1], 2 as $i);
                            $js(a)
                        }
                    )*)
                }


                doit! {
                    (rust_i8, js_i8, i8)
                    (rust_u8, js_u8, u8)
                    (rust_i16, js_i16, i16)
                    (rust_u16, js_u16, u16)
                    (rust_i32, js_i32, i32)
                    (rust_u32, js_u32, u32)
                    (rust_f32, js_f32, f32)
                    (rust_f64, js_f64, f64)
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function js_i8(a) {
                    assert.strictEqual(a.length, 2);
                    assert.strictEqual(a[0], 1);
                    assert.strictEqual(a[1], 2);
                    return a;
                }

                export function js_u8(a) {
                    assert.strictEqual(a.length, 2);
                    assert.strictEqual(a[0], 1);
                    assert.strictEqual(a[1], 2);
                    return a;
                }

                export function js_i16(a) {
                    assert.strictEqual(a.length, 2);
                    assert.strictEqual(a[0], 1);
                    assert.strictEqual(a[1], 2);
                    return a;
                }

                export function js_u16(a) {
                    assert.strictEqual(a.length, 2);
                    assert.strictEqual(a[0], 1);
                    assert.strictEqual(a[1], 2);
                    return a;
                }

                export function js_i32(a) {
                    assert.strictEqual(a.length, 2);
                    assert.strictEqual(a[0], 1);
                    assert.strictEqual(a[1], 2);
                    return a;
                }

                export function js_u32(a) {
                    assert.strictEqual(a.length, 2);
                    assert.strictEqual(a[0], 1);
                    assert.strictEqual(a[1], 2);
                    return a;
                }

                export function js_f32(a) {
                    assert.strictEqual(a.length, 2);
                    assert.strictEqual(a[0], 1);
                    assert.strictEqual(a[1], 2);
                    return a;
                }

                export function js_f64(a) {
                    assert.strictEqual(a.length, 2);
                    assert.strictEqual(a[0], 1);
                    assert.strictEqual(a[1], 2);
                    return a;
                }

                function assert_arrays_equal(a, b) {
                    console.log(a, b);
                    assert.strictEqual(a.length, b.length);
                    assert.strictEqual(a.byteLength, b.byteLength);
                    for (let i = 0; i < a.length; i++) {
                        assert.strictEqual(a[i], b[i]);
                    }
                }

                export function test() {
                    const i8 = new Int8Array(2);
                    i8[0] = 1;
                    i8[1] = 2;
                    assert_arrays_equal(wasm.rust_i8(i8), i8);
                    const u8 = new Uint8Array(2);
                    u8[0] = 1;
                    u8[1] = 2;
                    assert_arrays_equal(wasm.rust_u8(u8), u8);

                    const i16 = new Int16Array(2);
                    i16[0] = 1;
                    i16[1] = 2;
                    assert_arrays_equal(wasm.rust_i16(i16), i16);
                    const u16 = new Uint16Array(2);
                    u16[0] = 1;
                    u16[1] = 2;
                    assert_arrays_equal(wasm.rust_u16(u16), u16);

                    const i32 = new Int32Array(2);
                    i32[0] = 1;
                    i32[1] = 2;
                    assert_arrays_equal(wasm.rust_i32(i32), i32);
                    const u32 = new Uint32Array(2);
                    u32[0] = 1;
                    u32[1] = 2;
                    assert_arrays_equal(wasm.rust_u32(u32), u32);

                    const f32 = new Float32Array(2);
                    f32[0] = 1;
                    f32[1] = 2;
                    assert_arrays_equal(wasm.rust_f32(f32), f32);
                    const f64 = new Float64Array(2);
                    f64[0] = 1;
                    f64[1] = 2;
                    assert_arrays_equal(wasm.rust_f64(f64), f64);
                }
            "#,
        )
        .test();
}

#[test]
fn pass_array_works() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                macro_rules! doit {
                    ($(($rust:ident, $i:ident))*) => ($(
                        #[wasm_bindgen]
                        pub fn $rust(a: &[$i]) {
                            assert_eq!(a.len(), 2);
                            assert_eq!(a[0], 1 as $i);
                            assert_eq!(a[1], 2 as $i);
                        }
                    )*)
                }


                doit! {
                    (rust_i8, i8)
                    (rust_u8, u8)
                    (rust_i16, i16)
                    (rust_u16, u16)
                    (rust_i32, i32)
                    (rust_u32, u32)
                    (rust_f32, f32)
                    (rust_f64, f64)
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as wasm from "./out";

                export function test() {
                    wasm.rust_i8([1, 2]);
                    wasm.rust_u8([1, 2]);
                    wasm.rust_i16([1, 2]);
                    wasm.rust_u16([1, 2]);
                    wasm.rust_i32([1, 2]);
                    wasm.rust_u32([1, 2]);
                    wasm.rust_f32([1, 2]);
                    wasm.rust_f64([1, 2]);
                }
            "#,
        )
        .test();
}

#[test]
fn import_mut() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                macro_rules! doit {
                    ($(($rust:ident, $js:ident, $i:ident))*) => (
                        $(
                            #[wasm_bindgen(module = "./test")]
                            extern {
                                fn $js(a: &mut [$i]);
                            }

                            fn $rust() {
                                let mut buf = [
                                    1 as $i,
                                    2 as $i,
                                    3 as $i,
                                ];
                                $js(&mut buf);
                                assert_eq!(buf[0], 4 as $i);
                                assert_eq!(buf[1], 5 as $i);
                                assert_eq!(buf[2], 3 as $i);
                            }
                        )*

                        #[wasm_bindgen]
                        pub fn run() {
                            $($rust();)*
                        }
                    )
                }


                doit! {
                    (rust_i8, js_i8, i8)
                    (rust_u8, js_u8, u8)
                    (rust_i16, js_i16, i16)
                    (rust_u16, js_u16, u16)
                    (rust_i32, js_i32, i32)
                    (rust_u32, js_u32, u32)
                    (rust_f32, js_f32, f32)
                    (rust_f64, js_f64, f64)
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                function foo(a) {
                    assert.strictEqual(a.length, 3);
                    assert.strictEqual(a[0], 1);
                    assert.strictEqual(a[1], 2);
                    a[0] = 4;
                    a[1] = 5;
                }

                export const js_i8 = foo;
                export const js_u8 = foo;
                export const js_i16 = foo;
                export const js_u16 = foo;
                export const js_i32 = foo;
                export const js_u32 = foo;
                export const js_f32 = foo;
                export const js_f64 = foo;

                export function test() {
                    wasm.run();
                }
            "#,
        )
        .test();
}

#[test]
fn export_mut() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                macro_rules! doit {
                    ($($i:ident)*) => ($(
                        #[wasm_bindgen]
                        pub fn $i(a: &mut [$i])  {
                            assert_eq!(a.len(), 3);
                            assert_eq!(a[0], 1 as $i);
                            assert_eq!(a[1], 2 as $i);
                            assert_eq!(a[2], 3 as $i);
                            a[0] = 4 as $i;
                            a[1] = 5 as $i;
                        }
                    )*)
                }


                doit! { i8 u8 i16 u16 i32 u32 f32 f64 }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                function run(a, rust) {
                    assert.strictEqual(a.length, 3);
                    a[0] = 1;
                    a[1] = 2;
                    a[2] = 3;
                    console.log(a);
                    rust(a);
                    console.log(a);
                    assert.strictEqual(a.length, 3);
                    assert.strictEqual(a[0], 4);
                    assert.strictEqual(a[1], 5);
                    assert.strictEqual(a[2], 3);
                }

                export function test() {
                    run(new Int8Array(3), wasm.i8);
                    run(new Uint8Array(3), wasm.u8);
                    run(new Int16Array(3), wasm.i16);
                    run(new Uint16Array(3), wasm.u16);
                    run(new Int32Array(3), wasm.i32);
                    run(new Uint32Array(3), wasm.u32);
                    run(new Float32Array(3), wasm.f32);
                    run(new Float64Array(3), wasm.f64);
                }
            "#,
        )
        .test();
}

#[test]
fn return_vec_ok() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]
                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn broken_vec() -> Vec<u32> {
                    vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
                }

                #[wasm_bindgen]
                pub fn web_main() -> Application {
                    Application::new()
                }

                #[wasm_bindgen]
                pub struct Application {
                    thing: Vec<u32>,
                }

                #[wasm_bindgen]
                impl Application {
                    pub fn new() -> Application {
                        let mut thing = vec![];
                        thing.push(0);
                        thing.push(0);
                        thing.push(0);
                        thing.push(0);
                        thing.push(0);

                        Application {
                            thing: thing
                        }
                    }
                    pub fn tick(&mut self) {
                        self.thing = self.thing.clone();
                    }
                }

                pub fn main() {
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";


                export function test() {
                    let app = wasm.web_main();

                    for (let i = 0; i < 10; i++) {
                        app.tick();
                        let bad = wasm.broken_vec();
                        console.log("Received from rust:", i, bad);
                        assert.strictEqual(bad[0], 1);
                        assert.strictEqual(bad[1], 2);
                        assert.strictEqual(bad[2], 3);
                        assert.strictEqual(bad[3], 4);
                        assert.strictEqual(bad[4], 5);
                        assert.strictEqual(bad[5], 6);
                        assert.strictEqual(bad[6], 7);
                        assert.strictEqual(bad[7], 8);
                        assert.strictEqual(bad[8], 9);
                    }
                }
            "#,
        )
        .test();
}
