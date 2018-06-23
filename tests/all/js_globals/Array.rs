#![allow(non_snake_case)]

use project;

#[test]
fn index_of() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_index_of(this: &js::Array, value: JsValue, from_index: i32) -> i32 {
                this.index_of(value, from_index)
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = ["a", "c", "x", "n"];
                let index = wasm.get_index_of(characters, "x", 0);
                let notFoundIndex = wasm.get_index_of(characters, "z", 0);

                assert.equal(index, 2);
                assert.equal(notFoundIndex, -1);

                let withFromIndex = wasm.get_index_of(characters, "x", -3);
                let withFromIndexNotFound = wasm.get_index_of(characters, "a", -2);

                assert.equal(withFromIndex, 2);
                assert.equal(withFromIndexNotFound, -1);
            }
        "#)
        .test()
}

#[test]
fn sort() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn sort_array(this: &js::Array) -> js::Array {
                this.sort()
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let numbers = [3, 1, 6, 2];
                let sorted = wasm.sort_array(numbers);

                assert.deepStrictEqual(sorted, [1, 2, 3, 6])
            }
        "#)
        .test()
}

#[test]
fn last_index_of() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_last_index_of(this: &js::Array, value: JsValue, from_index: i32) -> i32 {
                this.last_index_of(value, from_index)
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = ["a", "x", "c", "x", "n"];
                let index = wasm.get_last_index_of(characters, "x", 5);
                let notFoundIndex = wasm.get_last_index_of(characters, "z", 5);

                assert.equal(index, 3);
                assert.equal(notFoundIndex, -1);

                let withFromIndex = wasm.get_last_index_of(characters, "x", 2);
                let withFromIndexNotFound = wasm.get_last_index_of(characters, "x", 0);

                assert.equal(withFromIndex, 1);
                assert.equal(withFromIndexNotFound, -1);
            }
        "#)
        .test()
}

#[test]
fn join() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn join_array(this: &js::Array, delimiter: &str) -> String {
                this.join(delimiter)
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = ["a", "c", "x", "n"];
                let stringValue = wasm.join_array(characters, ", ");

                assert.equal("a, c, x, n", stringValue);
                let withForwardSlash = wasm.join_array(characters, "/");
                assert.equal("a/c/x/n", withForwardSlash);
            }
        "#)
        .test()
}

#[test]
fn slice() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn create_slice(this: &js::Array, start: u32, end: u32) -> js::Array {
                this.slice(start, end)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = ["a", "c", "x", "n", 1, "8"];
                let subset = wasm.create_slice(characters, 1, 3);

                assert.equal(subset[0], "c");
                assert.equal(subset[1], "x");
            }
        "#)
        .test()
}

#[test]
fn fill() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn fill_with(this: &js::Array, value: JsValue, start: u32, end: u32) -> js::Array {
                this.fill(value, start, end)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = ["a", "c", "x", "n", 1, "8"];
                let subset = wasm.fill_with(characters, 0, 0, 3);

                assert.equal(subset[0], 0);
                assert.equal(subset[4], 1);
            }
        "#)
        .test()
}

#[test]
fn copy_within() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn copy_values_within_array(this: &js::Array, target: i32, start: i32, end: i32) -> js::Array {
                this.copy_within(target, start, end)
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                wasm.copy_values_within_array(characters, 1, 4, 5);

                assert.equal(characters[1], 1);

                // if negatives were used
                wasm.copy_values_within_array(characters, -1, -3, -2);
                assert.equal(characters[5], 3);
            }
        "#)
        .test()
}

#[test]
fn pop() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn pop_in_it(this: &js::Array) -> JsValue {
                this.pop()
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let item = wasm.pop_in_it(characters);
                assert.equal(item, 2);
                assert.equal(characters.length, 5);
            }
        "#)
        .test()
}


#[test]
fn push() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn push_it_along(this: &js::Array, value: JsValue) -> u32 {
                this.push(value)
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let length = wasm.push_it_along(characters, "a");
                assert.equal(length, 7);
                assert.equal(characters[6], "a");
            }
        "#)
        .test()
}

#[test]
fn reverse() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn reverse_array(this: &js::Array) -> js::Array {
                this.reverse()
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let reversed = wasm.reverse_array(characters);
                assert.equal(reversed[0], 2);
                assert.equal(reversed[5], 8);
            }
        "#)
        .test()
}

#[test]
fn shift() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn shift_item(this: &js::Array) -> JsValue {
                this.shift()
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let shiftedItem = wasm.shift_item(characters);

                assert.equal(shiftedItem, 8);
                assert.equal(characters.length, 5);
            }
        "#)
        .test()
}

#[test]
fn unshift() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn unshift_item(this: &js::Array, value: JsValue) -> u32 {
                this.unshift(value)
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let length = wasm.unshift_item(characters, "abba");

                assert.equal(length, 7);
                assert.equal(characters[0], "abba");
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
            pub fn array_to_string(this: &js::Array) -> String {
                this.to_string()
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let arrayString = wasm.array_to_string(characters);

                assert.equal(arrayString, "8,5,4,3,1,2");
            }
        "#)
        .test()
}


#[test]
fn includes() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn array_includes(this: &js::Array, value: JsValue, from_index: i32) -> bool {
                this.includes(value, from_index)
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let isTwoIncluded = wasm.array_includes(characters, 2, 0);
                let isNineIncluded = wasm.array_includes(characters, 9, 0);

                assert.ok(isTwoIncluded);
                assert.ok(!isNineIncluded);

                let isThreeIncluded = wasm.array_includes(characters, 3, 4);
                assert.ok(!isThreeIncluded);
            }
        "#)
        .test()
}

#[test]
fn concat() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn array_concat(this: &js::Array, arr: &js::Array) -> js::Array {
                this.concat(arr)
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let arr1 = [1, 2, 3];
                let arr2 = [4, 5, 6];

                let new_array = wasm.array_concat(arr1, arr2)
                assert.deepStrictEqual(new_array, [1, 2, 3, 4, 5, 6]);
            }
        "#)
        .test()
}

#[test]
fn length() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn array_length(this: &js::Array) -> u32 {
                this.length()
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let charactersLength = wasm.array_length(characters);
                assert.equal(charactersLength, 6);

                var empty : number[] = [];
                let emptyLength = wasm.array_length(empty);
                assert.equal(emptyLength, 0);
            }
        "#)
        .test()
}

#[test]
fn filter() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn array_filter(this: &js::Array, function: JsValue) -> js::Array {
                this.filter(function)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]

                // filters by element
                let filteredArray1 = wasm.array_filter(characters, function(element: number) {
                    return element > 4;
                });
                assert.equal(filteredArray1.length, 2)
                assert.equal(filteredArray1[1], 5)

                // filters by index and element
                let filteredArray2 = wasm.array_filter(characters, function(element: number, index: number) {
                    return index < 3 && element > 7;
                });
                assert.equal(filteredArray2.length, 1)
                assert.equal(filteredArray2[0], 8)
            }
        "#)
        .test()
}

#[test]
fn find() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn array_find(this: &js::Array, function: JsValue) -> JsValue {
                this.find(function)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]

                // finds item
                let foundElement = wasm.array_find(characters, function(element: number) {
                    return element > 6;
                });
                assert.equal(foundElement, 8);

                // if unfound returns undefined
                let unfoundElement1 = wasm.array_find(characters, function(element: number) {
                    return element > 8;
                });
                assert.equal(unfoundElement1, undefined);

                // if incorrect type returns undefined
                let unfoundElement2 = wasm.array_find(characters, function(element: string) {
                    return element > "a";
                });
                assert.equal(unfoundElement2, undefined);
            }
        "#)
        .test()
}