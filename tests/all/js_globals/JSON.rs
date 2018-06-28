#![allow(non_snake_case)]

use project;

#[test]
fn stringify() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn stringify_value(value: JsValue) -> js::JsString {
                js::JSON::stringify(value)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.stringify_value(true), "true");
                assert.equal(wasm.stringify_value(1), "1");
                assert.equal(wasm.stringify_value("string"), '"string"');
                assert.equal(wasm.stringify_value(null), "null");
                assert.equal(wasm.stringify_value(undefined), undefined);
                assert.equal(wasm.stringify_value([1, 'false', false]), '[1,"false",false]');
                assert.equal(
                    wasm.stringify_value({some: {deep: {stringify: {test: "object"}}}}), 
                    '{"some":{"deep":{"stringify":{"test":"object"}}}}'
                );

                interface Circular {
                  circular: string | Circular;
                }
                let circular: Circular = {circular: "circular"};
                circular.circular = circular;
                assert.throws(() => wasm.stringify_value(circular), TypeError);
            }
        "#)
        .test()
}

#[test]
fn parse() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn parse_value(value: js::JsString) -> JsValue {
                js::JSON::parse(value)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.parse_value("true"), true);
                assert.equal(wasm.parse_value("1"), 1);
                assert.equal(wasm.parse_value('"some string"'), "some string");
                assert.equal(wasm.parse_value("null"), null);
                assert.deepEqual(wasm.parse_value('[1,"false",false]'), [1, 'false', false]);
                assert.deepEqual(
                    wasm.parse_value('{"some":{"deep":{"parse":{"test":"object"}}}}'), 
                    {some: {deep: {parse: {test: "object"}}}}
                );

                assert.equal(wasm.parse_value(true), true);
                assert.equal(wasm.parse_value(1), 1);
                assert.equal(wasm.parse_value(null), null);

                assert.throws(() => wasm.parse_value('{ some cracked: "json"}'), SyntaxError);
                assert.throws(() => wasm.parse_value({ object: "json"}), SyntaxError);
                assert.throws(() => wasm.parse_value(undefined), SyntaxError);
            }
        "#)
        .test()
}