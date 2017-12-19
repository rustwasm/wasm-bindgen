extern crate test_support;

#[test]
fn add() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                pub fn add(a: u32, b: u32) -> u32 {
                    a + b
                }

                pub fn add3(a: u32) -> u32 {
                    a + 3
                }

                pub fn get2() -> u32 {
                    2
                }
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";

            export function test(wasm) {
                assert.strictEqual(wasm.add(1, 2), 3);
                assert.strictEqual(wasm.add(2, 3), 5);
                assert.strictEqual(wasm.add3(2), 5);
                assert.strictEqual(wasm.get2(), 2);
            }
        "#)
        .test();
}

#[test]
fn string_arguments() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                pub fn assert_foo_and_bar(a: &str, b: &str) {
                    assert_eq!(a, "foo2");
                    assert_eq!(b, "bar");
                }

                pub fn assert_foo(a: &str) {
                    assert_eq!(a, "foo");
                }
            }
        "#)
        .file("test.js", r#"
            export function test(wasm) {
                wasm.assert_foo("foo");
                wasm.assert_foo_and_bar("foo2", "bar");
            }
        "#)
        .test();
}

#[test]
fn return_a_string() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                pub fn clone(a: &str) -> String {
                    a.to_string()
                }
            }

            wasm_bindgen! {
                pub fn concat(a: &str, b: &str, c: i8) -> String {
                    format!("{} {} {}", a, b, c)
                }
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";

            export function test(wasm) {
                assert.strictEqual(wasm.clone("foo"), "foo");
                assert.strictEqual(wasm.clone("another"), "another");
                assert.strictEqual(wasm.concat("a", "b", 3), "a b 3");
                assert.strictEqual(wasm.concat("c", "d", -2), "c d -2");
            }
        "#)
        .test();
}

#[test]
fn exceptions() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                pub fn foo(_a: u32) {}
                pub fn bar(_a: &str) {}
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";

            export function test(wasm) {
                assert.throws(() => wasm.foo('a'), /expected a number argument/);
                assert.throws(() => wasm.bar(3), /expected a string argument/);
            }
        "#)
        .test();
}
