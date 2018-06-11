use super::project;

#[test]
fn webidl() {
    project()
        .file(
            "foo.webidl",
            r#"
            [Constructor(float value)]
            interface Foo {
                [Pure]
                boolean my_cmp(Foo bar);
            };
        "#,
        )
        .file(
            "foo.ts",
            r#"
            export class Foo {
                value: number;
                constructor(value: number) {
                    this.value = value;
                }
                my_cmp(other: Foo): boolean {
                    return this.value === other.value;
                }
            }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            pub mod foo;

            #[wasm_bindgen]
            pub fn call_my_cmp(first: &foo::Foo, second: foo::Foo) -> bool {
                first.my_cmp(second)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from 'assert';
            import * as wasm from './out';
            import {Foo} from './foo';

            export function test() {
                const pi = new Foo(3.14159);
                const e = new Foo(2.71828);
                assert.strictEqual(wasm.call_my_cmp(pi, pi), true);
                assert.strictEqual(wasm.call_my_cmp(pi, e), false);
                assert.strictEqual(wasm.call_my_cmp(e, pi), false);
                assert.strictEqual(wasm.call_my_cmp(e, e), true);
            }
        "#,
        )
        .test();
}
