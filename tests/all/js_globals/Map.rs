#![allow(non_snake_case)]

use project;

#[test]
fn clear() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn map_clear(this: &js::Map) {
                this.clear();
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const map = new Map();
                map.set('foo', 'bar');
                map.set('bar', 'baz');
                assert.equal(map.size, 2);
                wasm.map_clear(map);
                assert.equal(map.size, 0);

            }
        "#)
        .test()
}

#[test]
fn delete() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn map_delete(this: &js::Map, key: &str) -> bool {
                this.delete(key)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const map = new Map();
                map.set('foo', 'bar');
                assert.equal(map.size, 1);
                assert.equal(wasm.map_delete(map, 'foo'), true);
                assert.equal(wasm.map_delete(map, 'bar'), false);
                assert.equal(map.size, 0);

            }
        "#)
        .test()
}

#[test]
fn for_each() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_bool_vals(this: &js::Map) -> js::Map {
                let res = js::Map::new();
                this.for_each(&mut |value, key| {
                    if value.as_bool().is_some() {
                        res.set(&key, &value);
                    }
                });
                res
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const map = new Map();
                map.set(1, true);
                map.set(2, false);
                map.set(3, "awoo");
                map.set(4, 100);
                map.set(5, []);
                map.set(6, {});

                const res = wasm.get_bool_vals(map);

                assert.equal(map.size, 6);
                assert.equal(res.size, 2);
                assert.equal(res.get(1), true);
                assert.equal(res.get(2), false);
            }
        "#)
        .test()
}

#[test]
fn get() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn map_get(this: &js::Map, key: &JsValue) -> JsValue {
                this.get(key)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const map = new Map();
                map.set('foo', 'bar');
                map.set(1, 2)
                assert.equal(wasm.map_get(map, 'foo'), 'bar');
                assert.equal(wasm.map_get(map, 1), 2);
                assert.equal(wasm.map_get(map, 2), undefined);
            }
        "#)
        .test()
}

#[test]
fn has() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn has(this: &js::Map, key: &JsValue) -> bool {
                this.has(key)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const map = new Map();
                map.set('foo', 'bar');
                assert.equal(wasm.has(map, 'foo'), true);
                assert.equal(wasm.has(map, 'bar'), false);
            }
        "#)
        .test()
}

#[test]
fn new() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn new_map() -> js::Map {
                js::Map::new()
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const map = wasm.new_map();

                assert.equal(map.size, 0);
            }
        "#)
        .test()
}

#[test]
fn set() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn set(this: &js::Map, key: &JsValue, value: &JsValue) -> js::Map {
                this.set(key, value)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const map = new Map();
                const newMap = wasm.set(map, 'foo', 'bar');
                assert.equal(map.has('foo'), true);
                assert.equal(newMap.has('foo'), true);
            }
        "#)
        .test()
}

#[test]
fn size() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn map_size(this: &js::Map) -> u32 {
                this.size()
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const map = new Map();
                map.set('foo', 'bar');
                map.set('bar', 'baz');
                assert.equal(wasm.map_size(map), 2);
            }
        "#)
        .test()
}
