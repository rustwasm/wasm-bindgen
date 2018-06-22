#![allow(non_snake_case)]

use project;

#[test]
fn new() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn new_object() -> js::Object {
                js::Object::new()
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(typeof wasm.new_object(), "object");
            }
        "#)
        .test()
}

#[test]
fn has_own_property() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn has_own_foo_property(obj: &js::Object, property: &JsValue) -> bool {
                obj.has_own_property(&property)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert(wasm.has_own_foo_property({ foo: 42 }, "foo"));
                assert(wasm.has_own_foo_property({ 42: "foo" }, 42));
                assert(!wasm.has_own_foo_property({ foo: 42 }, "bar"));

                const s = Symbol();
                assert(wasm.has_own_foo_property({ [s]: "foo" }, s));
            }
        "#)
        .test()
}

#[test]
fn to_string() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn to_string(obj: &js::Object) -> String {
                obj.to_string()
            }

            #[wasm_bindgen]
            pub fn test() {
                let object = js::Object::new();
                assert_eq!(object.to_string(), "[object Object]");
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.to_string({ foo: 42 }), "[object Object]");
                wasm.test();
            }
        "#)
        .test()
}

#[test]
fn is_prototype_of() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn obj_is_prototype_of_value(obj: &js::Object, value: &JsValue) -> bool {
                obj.is_prototype_of(&value)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            class Foo {}
            class Bar {}

            export function test() {
                const foo = new Foo();
                assert(wasm.obj_is_prototype_of_value(Foo.prototype, foo));
                assert(!wasm.obj_is_prototype_of_value(Bar.prototype, foo));
            }
        "#)
        .test()
}

#[test]
fn property_is_enumerable() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn property_is_enumerable(obj: &js::Object, property: &JsValue) -> bool {
                obj.property_is_enumerable(&property)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert(wasm.property_is_enumerable({ foo: 42 }, "foo"));
                assert(wasm.property_is_enumerable({ 42: "foo" }, 42));
                assert(!wasm.property_is_enumerable({}, 42));

                const obj = {};
                Object.defineProperty(obj, "foo", { enumerable: false });
                assert(!wasm.property_is_enumerable(obj, "foo"));

                const s = Symbol();
                assert.ok(wasm.property_is_enumerable({ [s]: true }, s));
            }
        "#)
        .test()
}

#[test]
fn to_locale_string() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn to_locale_string() -> String {
                let object = js::Object::new();
                object.to_locale_string()
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.to_locale_string(), "[object Object]");
            }
        "#)
        .test()
}
