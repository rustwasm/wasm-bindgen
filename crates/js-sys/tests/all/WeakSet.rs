#![allow(non_snake_case)]

use project;

#[test]
fn new() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn new_weak_set() -> js_sys::WeakSet {
                js_sys::WeakSet::new()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(typeof wasm.new_weak_set(), "object");
            }
        "#,
        )
        .test()
}

#[test]
fn has() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn has_value(this: &js_sys::WeakSet, value: js_sys::Object) -> bool {
                this.has(value)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let set = new WeakSet();
                let value = {some: "value"};
                set.add(value);
                assert.equal(wasm.has_value(set, value), true);

                let nonex = {nonexistent: "value"};
                assert.equal(wasm.has_value(set, nonex), false);
            }
        "#,
        )
        .test()
}

#[test]
fn add() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn add_value(this: &js_sys::WeakSet, value: js_sys::Object) -> js_sys::WeakSet {
                this.add(value)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let set = new WeakSet();
                let value = {some: "value"};
                wasm.add_value(set, value);
                assert.equal(set.has(value), true);

                assert.throws(() => { wasm.add_value(set, 1) }, TypeError);
                assert.throws(() => { wasm.add_value(set, true) }, TypeError);
                assert.throws(() => { wasm.add_value(set, "fail") }, TypeError);
                assert.throws(() => { wasm.add_value(set, null) }, TypeError);
                assert.throws(() => { wasm.add_value(set, undefined) }, TypeError);
            }
        "#,
        )
        .test()
}

#[test]
fn delete() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn delete_value(this: &js_sys::WeakSet, value: js_sys::Object) -> bool {
                this.delete(value)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let set = new WeakSet();
                let value = {some: "value"};
                set.add(value);
                assert.equal(wasm.delete_value(set, value), true);
                assert.equal(set.has(value), false);
                assert.equal(wasm.delete_value(set, value), false);

                assert.equal(wasm.delete_value(set, 1), false);
                assert.equal(wasm.delete_value(set, true), false);
                assert.equal(wasm.delete_value(set, "false"), false);
                assert.equal(wasm.delete_value(set, null), false);
                assert.equal(wasm.delete_value(set, undefined), false);
            }
        "#,
        )
        .test()
}
