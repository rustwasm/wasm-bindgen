use super::project;

#[test]
fn unused_imports_not_generated() {
    let mut project = project();

    project
        .debug(false)
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            extern {
                pub fn foo();
            }

            #[wasm_bindgen]
            pub fn run() {
            }
        "#)
        .file("test.js", r#"
            import { run } from "./out";

            export function test() {
                run();
            }
        "#)
        .test();

    let contents = project.read_js();
    assert!(contents.contains("run"), "didn't find `run` in {}", contents);
    assert!(!contents.contains("foo"), "found `foo` in {}", contents);
}

#[test]
fn versions() {
    project()
        .debug(false)
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "webpack", version = "^0.2.0")]
                extern {
                    fn foo();
                }

                #[wasm_bindgen]
                pub fn run() {
                    foo();
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as fs from 'fs';
                import * as assert from 'assert';

                export function test() {
                    const bytes = fs.readFileSync('out_bg.wasm');
                    const m = new WebAssembly.Module(bytes);
                    const name = '__wasm_pack_unstable';
                    const sections = WebAssembly.Module.customSections(m, name);
                    assert.strictEqual(sections.length, 1);
                    const b = new Uint8Array(sections[0]);
                    const buf = new Buffer(b);
                    const map = JSON.parse(buf.toString());
                    assert.deepStrictEqual(map, {
                        version: '0.0.1',
                        modules: [
                            ['webpack', '^0.2.0']
                        ]
                    });
                };
            "#,
        )
        .test();
}
