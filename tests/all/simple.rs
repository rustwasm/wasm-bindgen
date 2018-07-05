use super::project;

#[test]
fn add() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(proc_macro, wasm_custom_section)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn add(a: u32, b: u32) -> u32 {
                    a + b
                }

                #[wasm_bindgen]
                pub fn add3(a: u32) -> u32 {
                    a + 3
                }

                #[wasm_bindgen]
                pub fn get2(_b: bool) -> u32 {
                    2
                }

                #[wasm_bindgen]
                pub fn return_and_take_bool(a: bool, b: bool) -> bool {
                    a && b
                }

                #[wasm_bindgen]
                pub fn raw_pointers_work(a: *mut u32, b: *const u8) -> *const u32 {
                    unsafe {
                        (*a) = (*b) as u32;
                        return a
                    }
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    assert.strictEqual(wasm.add(1, 2), 3);
                    assert.strictEqual(wasm.add(2, 3), 5);
                    assert.strictEqual(wasm.add3(2), 5);
                    assert.strictEqual(wasm.get2(true), 2);
                    assert.strictEqual(wasm.return_and_take_bool(true, false), false);
                }
            "#,
        )
        .test();
}

#[test]
fn string_arguments() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(proc_macro, wasm_custom_section)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn assert_foo_and_bar(a: &str, b: &str) {
                    assert_eq!(a, "foo2");
                    assert_eq!(b, "bar");
                }

                #[wasm_bindgen]
                pub fn assert_foo(a: &str) {
                    assert_eq!(a, "foo");
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as wasm from "./out";

                export function test() {
                    wasm.assert_foo("foo");
                    wasm.assert_foo_and_bar("foo2", "bar");
                }
            "#,
        )
        .test();
}

#[test]
fn return_a_string() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(proc_macro, wasm_custom_section)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn clone(a: &str) -> String {
                    a.to_string()
                }

                #[wasm_bindgen]
                pub fn concat(a: &str, b: &str, c: i8) -> String {
                    format!("{} {} {}", a, b, c)
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    assert.strictEqual(wasm.clone("foo"), "foo");
                    assert.strictEqual(wasm.clone("another"), "another");
                    assert.strictEqual(wasm.concat("a", "b", 3), "a b 3");
                    assert.strictEqual(wasm.concat("c", "d", -2), "c d -2");
                }
            "#,
        )
        .test();
}

#[test]
fn exceptions() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(proc_macro, wasm_custom_section)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn foo(_a: u32) {}

                #[wasm_bindgen]
                pub fn bar(_a: &str) {}
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    assert.throws(() => wasm.foo('a'), /expected a number argument/);
                    assert.throws(() => wasm.bar(3), /expected a string argument/);
                }
            "#,
        )
        .test();
}

// #[test]
// fn other_imports() {
//     project()
//         .file("src/lib.rs", r#"
//             #![feature(proc_macro, wasm_custom_section)]
//
//             extern crate wasm_bindgen;
//
//             use wasm_bindgen::prelude::*;
//
//             extern {
//                 fn another_import(a: u32);
//             }
//
//             wasm_bindgen! {
//                 pub fn foo(a: u32) {
//                     unsafe { another_import(a); }
//                 }
//             }
//         "#)
//         .file("test.js", r#"
//             import * as assert from "assert";
//             import * as wasm from "./out";
//
//             let ARG: number | null = null;
//
//             export function test() {
//                 wasm.foo(2);
//                 assert.strictEqual(ARG, 2);
//             }
//         "#)
//         .test();
// }

#[test]
fn other_exports() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #[no_mangle]
                pub extern fn foo(_a: u32) {
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as wasm from "./out_bg";

                export function test() {
                    wasm.foo(2);
                }
            "#,
        )
        .test();
}

#[test]
fn no_std() {
    project()
        .no_std(true)
        .file(
            "src/lib.rs",
            r#"
                #![feature(proc_macro, wasm_custom_section, wasm_import_module)]
                #![no_std]
                #![allow(dead_code)]

                extern crate wasm_bindgen;
                extern crate std as _some_other_name;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "./foo")]
                extern {
                    fn test(a: &str);

                    type Js;
                    #[wasm_bindgen(constructor)]
                    fn new() -> Js;
                    #[wasm_bindgen(method)]
                    fn init(this: &Js);
                }

                #[wasm_bindgen]
                pub fn foo(_a: u32) {}
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as wasm from "./out_bg";

                export function test() {
                    // mostly just testing the project compiles here
                    wasm.foo(1);
                }
            "#,
        )
        .file(
            "foo.js",
            r#"
                export class Js {
                    init() {
                    }
                }
            "#,
        )
        .test();
}

#[test]
fn no_std_class() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(proc_macro, wasm_custom_section, wasm_import_module)]
                #![no_std]
                #![allow(dead_code)]

                extern crate wasm_bindgen;
                extern crate std as _some_other_name;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern {
                    fn test(a: &str);

                    type Js;
                    #[wasm_bindgen(constructor)]
                    fn new() -> Js;
                    #[wasm_bindgen(method, structural)]
                    fn init(this: &Js);
                }

                #[wasm_bindgen]
                pub fn foo(_a: u32) {}

                #[wasm_bindgen]
                pub struct A {}

                #[wasm_bindgen]
                impl A {
                    pub fn foo(&self) {}
                    pub fn bar(&mut self) {}
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as wasm from "./out_bg";

                export function test() {
                    // mostly just testing the project compiles here
                    wasm.foo(1);
                }
            "#,
        )
        .test();
}
