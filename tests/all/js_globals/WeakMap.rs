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
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn new_weak_map() -> js::WeakMap {
                js::WeakMap::new()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(typeof wasm.new_weak_map(), "object");
            }
        "#,
        )
        .test()
}

#[test]
fn get() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_value(this: &js::WeakMap, key: js::Object) -> JsValue {
                this.get(key)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let map = new WeakMap();
                let key = {some: "key"};
                map.set(key, "value");
                assert.equal(wasm.get_value(map, key), "value");

                let undef = "unexisting_key";
                assert.equal(typeof wasm.get_value(map, undef), "undefined");
            }
        "#,
        )
        .test()
}

#[test]
fn set() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn set_value(this: &js::WeakMap, key: js::Object, value: JsValue) -> js::WeakMap {
                this.set(key, value)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let map = new WeakMap();
                let key = {some: "key"};
                wasm.set_value(map, key, "value");
                assert.equal(map.get(key), "value");
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
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn has_value(this: &js::WeakMap, key: js::Object) -> bool {
                this.has(key)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let map = new WeakMap();
                let key = {some: "key"};
                map.set(key, "value");
                assert.equal(wasm.has_value(map, key), true);

                let undef = "unexisting_key";
                assert.equal(wasm.has_value(map, undef), false);
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
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn delete_key(this: &js::WeakMap, key: js::Object) -> bool {
                this.delete(key)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let map = new WeakMap();
                let key = {some: "key"};
                map.set(key, "value");
                assert.equal(wasm.delete_key(map, key), true);
                assert.equal(map.has(key), false);
                assert.equal(wasm.delete_key(map, key), false);
            }
        "#,
        )
        .test()
}
