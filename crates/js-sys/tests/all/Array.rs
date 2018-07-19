#![allow(non_snake_case)]

use project;

#[test]
fn filter() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn keep_numbers(array: &js::Array) -> js::Array {
                array.filter(&mut |x, _, _| x.as_f64().is_some())
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = ["a", "c", "x", "n"];
                let numbers = [1, 2, 3, 4];
                let mixed = ["a", 1, "b", 2];

                assert.deepStrictEqual(wasm.keep_numbers(characters), []);
                assert.deepStrictEqual(wasm.keep_numbers(numbers), numbers);
                assert.deepStrictEqual(wasm.keep_numbers(mixed), [1, 2]);
            }
        "#,
        )
        .test()
}

#[test]
fn index_of() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_index_of(this: &js::Array, value: JsValue, from_index: i32) -> i32 {
                this.index_of(value, from_index)
            }

        "#,
        )
        .file(
            "test.js",
            r#"
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
        "#,
        )
        .test()
}

#[test]
fn is_array() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn is_array(value: &JsValue) -> bool {
                js::Array::is_array(value)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.ok(wasm.is_array([]));
                assert.ok(wasm.is_array([1]));
                assert.ok(wasm.is_array(new Array()));
                assert.ok(wasm.is_array(new Array('a', 'b', 'c', 'd')));
                assert.ok(wasm.is_array(new Array(3)));
                assert.ok(wasm.is_array(Array.prototype));

                assert.ok(!wasm.is_array({}));
                assert.ok(!wasm.is_array(null));
                assert.ok(!wasm.is_array(undefined));
                assert.ok(!wasm.is_array(17));
                assert.ok(!wasm.is_array('Array'));
                assert.ok(!wasm.is_array(true));
                assert.ok(!wasm.is_array(false));
                assert.ok(!wasm.is_array({ __proto__: Array.prototype }));
            }
        "#,
        )
        .test()
}

#[test]
fn sort() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn sort_array(this: &js::Array) -> js::Array {
                this.sort()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let numbers = [3, 1, 6, 2];
                let sorted = wasm.sort_array(numbers);

                assert.deepStrictEqual(sorted, [1, 2, 3, 6])
            }
        "#,
        )
        .test()
}

#[test]
fn some() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn has_elem(array: &js::Array, arg: JsValue) -> bool {
                array.some(&mut |elem| arg == elem)
            }

        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let elements = ["z", 1, "y", 2];

                assert.deepStrictEqual(wasm.has_elem(elements, 2), true);
                assert.deepStrictEqual(wasm.has_elem(elements, "y"), true);
                assert.deepStrictEqual(wasm.has_elem(elements, "not an element"), false);
            }
        "#)
        .test()
}

#[test]
fn last_index_of() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_last_index_of(this: &js::Array, value: JsValue, from_index: i32) -> i32 {
                this.last_index_of(value, from_index)
            }

        "#,
        )
        .file(
            "test.js",
            r#"
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
        "#,
        )
        .test()
}

#[test]
fn join() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn join_array(this: &js::Array, delimiter: &str) -> js::JsString {
                this.join(delimiter)
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = ["a", "c", "x", "n"];
                let stringValue = wasm.join_array(characters, ", ");

                assert.equal("a, c, x, n", stringValue);
                let withForwardSlash = wasm.join_array(characters, "/");
                assert.equal("a/c/x/n", withForwardSlash);
            }
        "#,
        )
        .test()
}

#[test]
fn slice() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn create_slice(this: &js::Array, start: u32, end: u32) -> js::Array {
                this.slice(start, end)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = ["a", "c", "x", "n", 1, "8"];
                let subset = wasm.create_slice(characters, 1, 3);

                assert.equal(subset[0], "c");
                assert.equal(subset[1], "x");
            }
        "#,
        )
        .test()
}

#[test]
fn fill() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn fill_with(this: &js::Array, value: JsValue, start: u32, end: u32) -> js::Array {
                this.fill(value, start, end)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = ["a", "c", "x", "n", 1, "8"];
                let subset = wasm.fill_with(characters, 0, 0, 3);

                assert.equal(subset[0], 0);
                assert.equal(subset[4], 1);
            }
        "#,
        )
        .test()
}

#[test]
fn copy_within() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn copy_values_within_array(this: &js::Array, target: i32, start: i32, end: i32) -> js::Array {
                this.copy_within(target, start, end)
            }

        "#)
        .file("test.js", r#"
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
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn pop_in_it(this: &js::Array) -> JsValue {
                this.pop()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let item = wasm.pop_in_it(characters);
                assert.equal(item, 2);
                assert.equal(characters.length, 5);
            }
        "#,
        )
        .test()
}

#[test]
fn push() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn push_it_along(this: &js::Array, value: JsValue) -> u32 {
                this.push(value)
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let length = wasm.push_it_along(characters, "a");
                assert.equal(length, 7);
                assert.equal(characters[6], "a");
            }
        "#,
        )
        .test()
}

#[test]
fn reverse() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn reverse_array(this: &js::Array) -> js::Array {
                this.reverse()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let reversed = wasm.reverse_array(characters);
                assert.equal(reversed[0], 2);
                assert.equal(reversed[5], 8);
            }
        "#,
        )
        .test()
}

#[test]
fn shift() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn shift_item(this: &js::Array) -> JsValue {
                this.shift()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let shiftedItem = wasm.shift_item(characters);

                assert.equal(shiftedItem, 8);
                assert.equal(characters.length, 5);
            }
        "#,
        )
        .test()
}

#[test]
fn unshift() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn unshift_item(this: &js::Array, value: JsValue) -> u32 {
                this.unshift(value)
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let length = wasm.unshift_item(characters, "abba");

                assert.equal(length, 7);
                assert.equal(characters[0], "abba");
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
            pub fn array_to_string(this: &js::Array) -> js::JsString {
                this.to_string()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let arrayString = wasm.array_to_string(characters);

                assert.equal(arrayString, "8,5,4,3,1,2");
            }
        "#,
        )
        .test()
}

#[test]
fn includes() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn array_includes(this: &js::Array, value: JsValue, from_index: i32) -> bool {
                this.includes(value, from_index)
            }

        "#,
        )
        .file(
            "test.js",
            r#"
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
        "#,
        )
        .test()
}

#[test]
fn concat() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn array_concat(this: &js::Array, arr: &js::Array) -> js::Array {
                this.concat(arr)
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let arr1 = [1, 2, 3];
                let arr2 = [4, 5, 6];

                let new_array = wasm.array_concat(arr1, arr2)
                assert.deepStrictEqual(new_array, [1, 2, 3, 4, 5, 6]);
            }
        "#,
        )
        .test()
}

#[test]
fn length() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn array_length(this: &js::Array) -> u32 {
                this.length()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let characters = [8, 5, 4, 3, 1, 2]
                    let charactersLength = wasm.array_length(characters);
                    assert.equal(charactersLength, 6);

                    var empty = [];
                    let emptyLength = wasm.array_length(empty);
                    assert.equal(emptyLength, 0);
                }
        "#,
        )
        .test()
}

#[test]
fn every() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::js;

                #[wasm_bindgen]
                pub fn array_every_number_is_even(array: &js::Array) -> bool {
                    array.every(&mut |el, _, _| el.as_f64().unwrap() % 2f64 == 0f64)
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    const arrayEven = [2, 4, 6, 8];
                    const arrayOdd = [1, 3, 5, 7];
                    const arrayMixed = [2, 3, 4, 5];

                    assert.ok(wasm.array_every_number_is_even(arrayEven));
                    assert.ok(!wasm.array_every_number_is_even(arrayOdd));
                    assert.ok(!wasm.array_every_number_is_even(arrayMixed));
                }
            "#,
        )
        .test()
}

#[test]
fn find() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::js;

                #[wasm_bindgen]
                pub fn array_find_first_even_number(array: &js::Array) -> JsValue {
                    array.find(&mut |el, _, _| el.as_f64().unwrap() % 2f64 == 0f64)
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    const arrayEven = [2, 4, 6, 8];
                    const arrayOdd = [1, 3, 5, 7];
                    const arrayMixed = [3, 5, 7, 10];

                    assert.equal(wasm.array_find_first_even_number(arrayEven), 2);
                    assert.equal(wasm.array_find_first_even_number(arrayOdd), undefined);
                    assert.equal(wasm.array_find_first_even_number(arrayMixed), 10);
                }
            "#,
        )
        .test()
}

#[test]
fn map() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::js;
                use JsValue;

                #[wasm_bindgen]
                pub fn array_map(array: &js::Array) -> js::Array {
                    array.map(&mut |el, _, _| JsValue::from_f64(el.as_f64().unwrap().sqrt()))
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    const numbers = [1, 4, 9];
                    assert.deepStrictEqual(wasm.array_map(numbers), [1, 2, 3]);
                }
            "#,
        )
        .test()
}

#[test]
fn reduce() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::js;
                use JsValue;

                #[wasm_bindgen]
                pub fn array_reduce(array: &js::Array) -> JsValue {
                    array.reduce(&mut |ac, cr, _, _| JsValue::from_str(&format!("{}{}", &ac.as_string().unwrap(), &cr.as_string().unwrap().as_str())), JsValue::from_str(""))
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    assert.equal(wasm.array_reduce(['0', '1', '2', '3', '4']), '01234');
                }
            "#,
        )
        .test()
}

#[test]
fn reduce_right() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::js;
                use JsValue;

                #[wasm_bindgen]
                pub fn array_reduce_right(array: &js::Array) -> JsValue {
                    array.reduce_right(&mut |ac, cr, _, _| JsValue::from_str(&format!("{}{}", &ac.as_string().unwrap(), &cr.as_string().unwrap().as_str())), JsValue::from_str(""))
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    assert.equal(wasm.array_reduce_right(['0', '1', '2', '3', '4']), '43210');
                }
            "#,
        )
        .test()
}

#[test]
fn find_index() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::js;

                #[wasm_bindgen]
                pub fn array_find_first_even_number_index(array: &js::Array) -> u32 {
                    array.find_index(&mut |el, _, _| el.as_f64().unwrap() % 2. == 0.)
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    const arrayEven = [2, 4, 6, 8];
                    const arrayOdd = [1, 3, 5, 7];
                    const arrayMixed = [3, 5, 7, 10];

                    assert.equal(wasm.array_find_first_even_number_index(arrayEven), 0);
                    assert.equal(wasm.array_find_first_even_number_index(arrayOdd), -1);
                    assert.equal(wasm.array_find_first_even_number_index(arrayMixed), 3);
                }
            "#,
        )
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
                use JsValue;

                #[wasm_bindgen]
                pub fn array_to_locale_string(array: &js::Array, locale: &JsValue, options: &JsValue) -> js::JsString {
                    array.to_locale_string(locale, options)
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    const output = wasm.array_to_locale_string([1, 'a', new Date('21 Dec 1997 14:12:00 UTC')], 'en', {timeZone: 'UTC'});
                    assert.equal(typeof output, 'string');
                    assert.ok(output.length > 0);
                }
            "#,
        )
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
                pub fn sum_indices_of_evens(array: &js::Array) -> u32 {
                    let mut res = 0;
                    array.for_each(&mut |elem: JsValue, i, _| {
                        match elem.as_f64() {
                            Some(val) if val % 2. == 0. => res += i,
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
                    const arrayEven = [2, 4, 6, 8];
                    const arrayEvenExpected = 0 + 1 + 2 + 3;
                    const arrayEvenActual = wasm.sum_indices_of_evens(arrayEven);
                    assert.equal(arrayEvenActual, arrayEvenExpected);

                    const arrayOdd = [1, 3, 5, 7];
                    const arrayOddExpected = 0;
                    const arrayOddActual = wasm.sum_indices_of_evens(arrayOdd);
                    assert.equal(arrayOddActual, arrayOddExpected);

                    const arrayMixed = [3, 5, 7, 10];
                    const arrayMixedExpected = 3;
                    const arrayMixedActual = wasm.sum_indices_of_evens(arrayMixed);
                    assert.equal(arrayMixedActual, arrayMixedExpected);
                }
        "#)
        .test()
}
