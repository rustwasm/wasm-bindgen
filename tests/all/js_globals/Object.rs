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
            pub fn new_object() -> js::Object {
                js::Object::new()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(typeof wasm.new_object(), "object");
            }
        "#,
        )
        .test()
}

#[test]
fn has_own_property() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn has_own_foo_property(obj: &js::Object, property: &JsValue) -> bool {
                obj.has_own_property(&property)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.ok(wasm.has_own_foo_property({ foo: 42 }, "foo"));
                assert.ok(wasm.has_own_foo_property({ 42: "foo" }, 42));
                assert.ok(!wasm.has_own_foo_property({ foo: 42 }, "bar"));

                const s = Symbol();
                assert.ok(wasm.has_own_foo_property({ [s]: "foo" }, s));
            }
        "#,
        )
        .test()
}

#[test]
fn to_string() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn to_string(obj: &js::Object) -> js::JsString {
                obj.to_string()
            }

            #[wasm_bindgen]
            pub fn test() {
                let object = js::Object::new();
                assert_eq!(String::from(object.to_string()), "[object Object]");
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.to_string({ foo: 42 }), "[object Object]");
                wasm.test();
            }
        "#,
        )
        .test()
}

#[test]
fn is_extensible() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn is_extensible(obj: &js::Object) -> bool {
                js::Object::is_extensible(&obj)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {};
                assert.ok(wasm.is_extensible(object));

                Object.preventExtensions(object);
                assert.ok(!wasm.is_extensible(object));
            }
        "#)
        .test()
}

#[test]
fn is_frozen() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn is_frozen(obj: &js::Object) -> bool {
                js::Object::is_frozen(&obj)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {};
                assert.ok(!wasm.is_frozen(object));

                Object.freeze(object);
                assert.ok(wasm.is_frozen(object));
            }
        "#)
        .test()
}

#[test]
fn is_sealed() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn is_sealed(obj: &js::Object) -> bool {
                js::Object::is_sealed(&obj)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {};
                assert.ok(!wasm.is_sealed(object));

                Object.seal(object);
                assert.ok(wasm.is_sealed(object));
            }
        "#)
        .test()
}

#[test]
fn is_prototype_of() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn obj_is_prototype_of_value(obj: &js::Object, value: &JsValue) -> bool {
                obj.is_prototype_of(&value)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            class Foo {}
            class Bar {}

            export function test() {
                const foo = new Foo();
                assert.ok(wasm.obj_is_prototype_of_value(Foo.prototype, foo));
                assert.ok(!wasm.obj_is_prototype_of_value(Bar.prototype, foo));
            }
        "#,
        )
        .test()
}

#[test]
fn keys() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn keys(obj: &js::Object) -> js::Array {
                js::Object::keys(obj)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const obj = { a: 1, b: 2, c: 3 };
                assert.deepStrictEqual(wasm.keys(obj), ["a", "b", "c"]);
            }
        "#,
        )
        .test()
}

#[test]
fn prevent_extensions() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn prevent_extensions(obj: &js::Object) {
                js::Object::prevent_extensions(obj);
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {};
                wasm.prevent_extensions(object);

                assert.ok(!Object.isExtensible(object));
                assert.throws(() => {
                    'use strict';
                    Object.defineProperty(object, 'foo', { value: 42 });
                }, TypeError);
            }
        "#)
        .test()
}

#[test]
fn property_is_enumerable() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn property_is_enumerable(obj: &js::Object, property: &JsValue) -> bool {
                obj.property_is_enumerable(&property)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.ok(wasm.property_is_enumerable({ foo: 42 }, "foo"));
                assert.ok(wasm.property_is_enumerable({ 42: "foo" }, 42));
                assert.ok(!wasm.property_is_enumerable({}, 42));

                const obj = {};
                Object.defineProperty(obj, "foo", { enumerable: false });
                assert.ok(!wasm.property_is_enumerable(obj, "foo"));

                const s = Symbol();
                assert.ok(wasm.property_is_enumerable({ [s]: true }, s));
            }
        "#,
        )
        .test()
}

#[test]
fn seal() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn seal(value: &JsValue) -> JsValue {
                js::Object::seal(&value)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = { foo: 'bar' };
                const sealedObject = wasm.seal(object);
                assert.strictEqual(object, sealedObject);
                assert.throws(() => {
                    'use strict';
                    sealedObject.bar = 'foo';
                }, TypeError);
                assert.throws(() => {
                    'use strict';
                    delete sealedObject.foo;
                }, TypeError);

                const primitive = 42;
                assert.doesNotThrow(() => {
                    'use strict';
                    // according to ES2015, this should not throw anymore
                    // see https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/seal#Notes
                    wasm.seal(primitive);
                });

                const array = [1, 2, 3];
                const sealedArray = wasm.seal(array);
                assert.throws(() => {
                    'use strict';
                    sealedArray.push(42);
                }, TypeError);
            }
        "#)
        .test()
}

#[test]
fn set_prototype_of() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn set_prototype_of(object: &js::Object, prototype: &js::Object) -> js::Object {
                js::Object::set_prototype_of(&object, &prototype)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = { foo: 42 };
                const newPrototype = { bar: 'baz' };

                const modifiedObject = wasm.set_prototype_of(object, newPrototype);
                assert.ok(newPrototype.isPrototypeOf(modifiedObject));
            }
        "#)
        .test()
}

#[test]
fn to_locale_string() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn to_locale_string() -> js::JsString {
                let object = js::Object::new();
                object.to_locale_string()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.to_locale_string(), "[object Object]");
            }
        "#,
        )
        .test()
}

#[test]
fn value_of() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn value_of(obj: &js::Object) -> js::Object {
                obj.value_of()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const obj = { foo: 42 };
                assert.strictEqual(wasm.value_of(obj), obj);
                assert.notStrictEqual(wasm.value_of(obj), { foo: 42 });
            }
        "#,
        )
        .test()
}

#[test]
fn values() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn values(obj: &js::Object) -> js::Array {
                js::Object::values(&obj)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = { foo: 'bar', baz: 'qux' };
                const values = wasm.values(object);
                assert.equal(values.length, 2);
                assert.deepEqual(values.sort(), ['bar', 'qux']);
            }
        "#)
        .test()
}
