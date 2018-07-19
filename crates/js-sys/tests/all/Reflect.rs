#![allow(non_snake_case)]

use project;

#[test]
fn apply() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn apply(target: &js_sys::Function, this_argument: &JsValue, arguments_list: &js_sys::Array) -> JsValue {
                js_sys::Reflect::apply(target, this_argument, arguments_list)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.apply("".charAt, "ponies", [3]), "i");
            }
        "#,
        )
        .test()
}

#[test]
fn construct() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn construct(target: &js_sys::Function, arguments_list: &js_sys::Array) -> JsValue {
                js_sys::Reflect::construct(target, arguments_list)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                class Rectangle {
                    constructor(x, y){
                        this.x = x,
                        this.y = y
                    }

                    static eq(x, y) { 
                        return x === y;
                    }

                }

                const args = [10, 10];

                assert.equal(wasm.construct(Rectangle, args).x, 10);
            }
        "#,
        )
        .test()
}

#[test]
fn construct_with_new_target() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn construct_with_new_target(target: &js_sys::Function, arguments_list: &js_sys::Array, new_target: &js_sys::Function) -> JsValue {
                js_sys::Reflect::construct_with_new_target(target, arguments_list, new_target)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                class Rectangle {
                    constructor(x, y){
                        this.x = x,
                        this.y = y
                    }

                    static eq(x, y) { 
                        return x === y;
                    }

                }

                class Rectangle2 {
                    constructor(x, y){
                        this.x = x,
                        this.y = y
                    }

                    static eq(x, y) { 
                        return x === y;
                    }

                }

                const args = [10, 10];

                assert.equal(wasm.construct_with_new_target(Rectangle, args, Rectangle2).x, 10);
            }
        "#,
        )
        .test()
}

#[test]
fn define_property() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn define_property(target: &js_sys::Object, property_key: &JsValue, attributes: &js_sys::Object) -> bool {
                js_sys::Reflect::define_property(target, property_key, attributes)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {};

                assert.equal(wasm.define_property(object, "key", { value: 42}), true)
            }
        "#,
        )
        .test()
}

#[test]
fn delete_property() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn delete_property(target: &js_sys::Object, property_key: &JsValue) -> bool {
                js_sys::Reflect::delete_property(target, property_key)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {
                    property: 42
                };

                wasm.delete_property(object, "property");

                assert.equal(object.property, undefined);

                const array = [1, 2, 3, 4, 5];
                wasm.delete_property(array, 3);

                assert.equal(array[3], undefined);
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
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn get(target: &js_sys::Object, property_key: &JsValue) -> JsValue {
                js_sys::Reflect::get(target, property_key)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {
                    property: 42
                };

                assert.equal(wasm.get(object, "property"), 42);

                const array = [1, 2, 3, 4, 5];

                assert.equal(wasm.get(array, 3), 4);
            }
        "#,
        )
        .test()
}

#[test]
fn get_own_property_descriptor() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn get_own_property_descriptor(target: &js_sys::Object, property_key: &JsValue) -> JsValue {
                js_sys::Reflect::get_own_property_descriptor(target, property_key)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {
                    property: 42
                };

                assert.equal(wasm.get_own_property_descriptor(object, "property").value, 42);
                assert.equal(wasm.get_own_property_descriptor(object, "property1"), undefined);
            }
        "#,
        )
        .test()
}

#[test]
fn get_prototype_of() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn get_prototype_of(target: &js_sys::Object) -> js_sys::Object {
                js_sys::Reflect::get_prototype_of(target)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {
                    property: 42
                };
                const array = [1, 2, 3];

                assert.equal(wasm.get_prototype_of(object), Object.prototype);
                assert.equal(wasm.get_prototype_of(array), Array.prototype);
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
            pub fn has(target: &js_sys::Object, property_key: &JsValue) -> bool {
                js_sys::Reflect::has(target, property_key)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {
                    property: 42
                };
                const array = [1, 2, 3, 4]

                assert.equal(wasm.has(object, "property"), true);
                assert.equal(wasm.has(object, "foo"), false);
                assert.equal(wasm.has(array, 3), true);
                assert.equal(wasm.has(array, 10), false);
            }
        "#,
        )
        .test()
}

#[test]
fn is_extensible() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn is_extensible(target: &js_sys::Object) -> bool {
                js_sys::Reflect::is_extensible(target)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {
                    property: 42
                };

                assert.equal(wasm.is_extensible(object), true);

                Reflect.preventExtensions(object);

                assert.equal(wasm.is_extensible(object), false);

                const object2 = Object.seal({});

                assert.equal(wasm.is_extensible(object2), false);
            }
        "#,
        )
        .test()
}

#[test]
fn own_keys() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn own_keys(target: &js_sys::Object) -> js_sys::Array {
                js_sys::Reflect::own_keys(target)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {
                    property: 42
                };
                const array = [];

                assert.equal(wasm.own_keys(object)[0], "property");
                assert.equal(wasm.own_keys(array)[0], "length");
            }
        "#,
        )
        .test()
}

#[test]
fn prevent_extensions() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn prevent_extensions(target: &js_sys::Object) -> bool {
                js_sys::Reflect::prevent_extensions(target)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object1 = {};

                wasm.prevent_extensions(object1);

                assert.equal(Reflect.isExtensible(object1), false);
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
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn set(target: &js_sys::Object, property_key: &JsValue, value: &JsValue) -> bool {
                js_sys::Reflect::set(target, property_key, value)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {};
                const array = [1, 2, 3, 4];
                assert.equal(wasm.set(object, "key", "value"), true);
                assert.equal(wasm.set(array, 0, 100), true);

                assert.equal(Reflect.get(object, "key"), "value");
                assert.equal(array[0], 100);
            }
        "#,
        )
        .test()
}

#[test]
fn set_with_receiver() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn set_with_receiver(target: &js_sys::Object, property_key: &JsValue, value: &JsValue, receiver: &js_sys::Object) -> bool {
                js_sys::Reflect::set_with_receiver(target, property_key, value, receiver)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {};
                const array = [1, 2, 3, 4];
                assert.equal(wasm.set_with_receiver({}, "key", "value", object), true);
                assert.equal(wasm.set_with_receiver([], 0, 100, array), true);

                assert.equal(Reflect.get(object, "key"), "value");
                assert.equal(array[0], 100);
            }
        "#,
        )
        .test()
}

#[test]
fn set_prototype_of() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn set_prototype_of(target: &js_sys::Object, prototype: &JsValue) -> bool {
                js_sys::Reflect::set_prototype_of(target, prototype)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {};
                assert.equal(wasm.set_prototype_of(object, Object.prototype), true);
                assert.equal(Object.getPrototypeOf(object), Object.prototype);
                assert.equal(wasm.set_prototype_of(object, null), true);
                assert.equal(Object.getPrototypeOf(object), null);
            }
        "#,
        )
        .test()
}