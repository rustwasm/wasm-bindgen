use super::project;

#[test]
fn add() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

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
fn add_headless() {
    project()
        .headless(true)
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn add(a: u32, b: u32) -> u32 {
                    a + b
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    console.log("start `add_headless` test");
                    assert.strictEqual(wasm.add(1, 2), 3);
                    console.log("end `add_headless` test");
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
                #![feature(use_extern_macros)]

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
                #![feature(use_extern_macros)]

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
                #![feature(use_extern_macros)]

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
//             #![feature(use_extern_macros)]
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
                #![feature(use_extern_macros, wasm_import_module)]
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
                #![feature(use_extern_macros, wasm_import_module)]
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

#[test]
fn jsvalue_typeof() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn is_object(val: &JsValue) -> bool {
                    val.is_object()
                }

                #[wasm_bindgen]
                pub fn is_function(val: &JsValue) -> bool {
                    val.is_function()
                }

                #[wasm_bindgen]
                pub fn is_string(val: &JsValue) -> bool {
                    val.is_string()
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    assert.ok(wasm.is_object({}));
                    assert.ok(!wasm.is_object(42));
                    assert.ok(wasm.is_function(function() {}));
                    assert.ok(!wasm.is_function(42));
                    assert.ok(wasm.is_string("2b or !2b"));
                    assert.ok(!wasm.is_string(42));
                }
            "#,
        )
        .test();
}

#[test]
fn binding_to_unimplemented_apis_doesnt_break_everything() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern {
                    #[derive(Clone)]
                    type Array;

                    #[wasm_bindgen(constructor)]
                    fn new() -> Array;

                    #[wasm_bindgen(method, catch)]
                    fn standardized_method_this_js_runtime_doesnt_implement_yet(this: &Array)
                         -> Result<(), JsValue>;
                }

                #[wasm_bindgen]
                pub fn test() {
                    let array = Array::new();
                    let res = array.standardized_method_this_js_runtime_doesnt_implement_yet();
                    assert!(res.is_err());
                }
            "#,
        )
        .test();
}

#[test]
fn optional_slices() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_custom_section, wasm_import_module)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "./foo")]
                extern {
                    fn optional_str_none(a: Option<&str>);
                    fn optional_str_some(a: Option<&str>);
                    fn optional_slice_none(a: Option<&[u8]>);
                    fn optional_slice_some(a: Option<&[u8]>);

                    fn optional_string_none(a: Option<String>);
                    fn optional_string_some(a: Option<String>);
                    fn optional_string_some_empty(a: Option<String>);

                    fn return_string_none() -> Option<String>;
                    fn return_string_some() -> Option<String>;

                    fn run_rust_tests();
                }

                #[wasm_bindgen]
                pub fn test() {
                    optional_str_none(None);
                    optional_str_some(Some("x"));
                    optional_slice_none(None);
                    optional_slice_some(Some(&[1, 2, 3]));
                    optional_string_none(None);
                    optional_string_some_empty(Some(String::new()));
                    optional_string_some(Some("abcd".to_string()));

                    assert_eq!(return_string_none(), None);
                    assert_eq!(return_string_some(), Some("foo".to_string()));
                    run_rust_tests();
                }

                #[wasm_bindgen]
                pub fn take_optional_str_none(x: Option<String>) {
                    assert!(x.is_none())
                }
                #[wasm_bindgen]
                pub fn take_optional_str_some(x: Option<String>) {
                    assert_eq!(x, Some(String::from("hello")));
                }
                #[wasm_bindgen]
                pub fn return_optional_str_none() -> Option<String> {
                    None
                }
                #[wasm_bindgen]
                pub fn return_optional_str_some() -> Option<String> {
                    Some("world".to_string())
                }
            "#,
        )
        .file(
            "foo.js",
            r#"
                import { strictEqual } from "assert";
                import * as wasm from "./out";

                export function optional_str_none(x) {
                    strictEqual(x, undefined);
                }

                export function optional_str_some(x) {
                    strictEqual(x, 'x');
                }

                export function optional_slice_none(x) {
                    strictEqual(x, undefined);
                }

                export function optional_slice_some(x) {
                    strictEqual(x.length, 3);
                    strictEqual(x[0], 1);
                    strictEqual(x[1], 2);
                    strictEqual(x[2], 3);
                }

                export function optional_string_none(x) {
                    strictEqual(x, undefined);
                }

                export function optional_string_some(x) {
                    strictEqual(x, 'abcd');
                }

                export function optional_string_some_empty(x) {
                    strictEqual(x, '');
                }

                export function return_string_none() {}
                export function return_string_some() {
                    return 'foo';
                }

                export function run_rust_tests() {
                    wasm.take_optional_str_none();
                    wasm.take_optional_str_none(null);
                    wasm.take_optional_str_none(undefined);
                    wasm.take_optional_str_some('hello');
                    strictEqual(wasm.return_optional_str_none(), undefined);
                    strictEqual(wasm.return_optional_str_some(), 'world');
                }
            "#
        )
        .test();
}
