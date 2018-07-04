#![allow(non_snake_case)]

use project;

#[test]
fn apply() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn apply(target: &js::Function, this_argument: &JsValue, arguments_list: &js::Array) -> JsValue {
                let result = js::Reflect::apply(target, this_argument, arguments_list);
                let result = match result {
                    Ok(val) => val,
                    Err(_err) => "TypeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.apply(''.charAt, 'ponies', [3]), 'i');
                assert.equal(wasm.apply('', 'ponies', [3]), "TypeError");
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
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn construct(target: &js::Function, arguments_list: &js::Array) -> JsValue {
                let result = js::Reflect::construct(target, arguments_list);
                let result = match result {
                    Ok(val) => val,
                    Err(_err) => "TypeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                class Rectangle {
                    public x: number;
                    public y: number;

                    constructor(x: number, y: number){
                        this.x = x,
                        this.y = y
                    }

                    static eq(x: number, y: number) { 
                        return x === y;
                    }

                }

                const args = [10, 10];

                assert.equal(wasm.construct(Rectangle, args).x, 10);
                assert.equal(wasm.construct('', args), "TypeError");
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
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn construct_with_new_target(target: &js::Function, arguments_list: &js::Array, new_target: &js::Function) -> JsValue {
                let result = js::Reflect::construct_with_new_target(target, arguments_list, new_target);
                let result = match result {
                    Ok(val) => val,
                    Err(_err) => "TypeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                class Rectangle {
                    public x: number;
                    public y: number;

                    constructor(x: number, y: number){
                        this.x = x,
                        this.y = y
                    }

                    static eq(x: number, y: number) { 
                        return x === y;
                    }

                }

                class Rectangle2 {
                    public x: number;
                    public y: number;

                    constructor(x: number, y: number){
                        this.x = x,
                        this.y = y
                    }

                    static eq(x: number, y: number) { 
                        return x === y;
                    }

                }

                const args = [10, 10];

                assert.equal(wasm.construct_with_new_target(Rectangle, args, Rectangle2).x, 10);
                assert.equal(wasm.construct_with_new_target(Rectangle, args, ''), "TypeError");
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
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn define_property(target: &js::Object, property_key: &js::JsString, attributes: &js::Object) -> JsValue {
                let result = js::Reflect::define_property(target, property_key, attributes);
                let result = match result {
                    Ok(val) => val,
                    Err(_err) => "TypeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {};

                assert.equal(wasm.define_property(object, "key", { value: 42}), true)
                assert.equal(wasm.define_property("", "key", { value: 42 }), "TypeError");
            }
        "#,
        )
        .test()
}