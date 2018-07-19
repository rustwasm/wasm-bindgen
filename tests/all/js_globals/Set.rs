#![allow(non_snake_case)]

use project;

#[test]
fn add() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn add(this: &js::Set, value: &JsValue) -> js::Set {
                this.add(value)
            }

        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let set = new Set([]);

                wasm.add(set, 100);


                assert.equal(set.size, 1);
                assert.equal(Array.from(set)[0], 100);
            }
        "#)
        .test()
}

#[test]
fn clear() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn clear(this: &js::Set) {
                this.clear();
            }

        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let set = new Set([1, 2, 3]);

                wasm.clear(set);

                assert.equal(set.size, 0);
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
            pub fn set_delete(this: &js::Set, value: &JsValue) -> bool {
                this.delete(value)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let set = new Set([1, 2, 3]);

                assert.equal(wasm.set_delete(set, 4), false);
                assert.equal(wasm.set_delete(set, 2), true);
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
                pub fn count_evens(set: &js::Set) -> u32 {
                    let mut res = 0;
                    set.for_each(&mut |value, _, _| {
                        match value.as_f64() {
                            Some(val) if val % 2. == 0. => res += 1,
                            _ => { }
                        }
                    });
                    res
                }
        "#)
        .file("test.js", r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let setEven = new Set([2, 4, 6, 8]);
                    let setEvenExpected = 4;
                    let setEvenActual = wasm.count_evens(setEven);
                    assert.equal(setEvenExpected, setEvenActual);

                    let setOdd = new Set([1, 3, 5, 7]);
                    let setOddExpected = 0;
                    let setOddActual = wasm.count_evens(setOdd);
                    assert.equal(setOddExpected, setOddActual);

                    let setMixed = new Set([3, 5, 7, 10]);
                    let setMixedExpected = 1;
                    let setMixedActual = wasm.count_evens(setMixed);
                    assert.equal(setMixedExpected, setMixedActual);
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
            pub fn has(this: &js::Set, value: &JsValue) -> bool {
                this.has(value)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let set = new Set([1, 2, 3]);

                assert.equal(wasm.has(set, 4), false);
                assert.equal(wasm.has(set, 2), true);
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
            pub fn new_set() -> js::Set {
                js::Set::new()
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let set = wasm.new_set();

                assert.equal(set.size, 0);
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
            pub fn size(this: &js::Set) -> u32 {
                this.size()
            }

        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let set = new Set([8, 5, 4, 3, 1, 2]);

                assert.equal(wasm.size(set), 6);
            }
        "#)
        .test()
}
