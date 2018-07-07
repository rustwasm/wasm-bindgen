#![allow(non_snake_case)]

use project;

#[test]
fn has_instance() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn symbol_has_instance() -> js::Symbol {
                js::Symbol::has_instance()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";
            class Array1 {
                static [wasm.symbol_has_instance()](instance) {
                    return Array.isArray(instance);
                }
            }

            export function test() {
                assert.equal(typeof wasm.symbol_has_instance(), "symbol");
                assert.ok([] instanceof Array1);
            }
        "#,
        )
        .test()
}

#[test]
fn is_concat_spreadable() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn symbol_is_cancat_spreadable() -> js::Symbol {
                js::Symbol::is_concat_spreadable()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const alpha = ['a', 'b', 'c'];
                const numeric = [1, 2, 3];
                let alphaNumeric = alpha.concat(numeric);

                assert.deepEqual(alphaNumeric, ["a", "b", "c", 1, 2, 3]);

                numeric[wasm.symbol_is_cancat_spreadable()] = false;
                alphaNumeric = alpha.concat(numeric);

                assert.deepEqual(alphaNumeric, ["a", "b", "c", [1, 2, 3]]);
            }
        "#,
        )
        .test();
}

#[test]
fn iterator() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn symbol_iterator() -> js::Symbol {
                js::Symbol::iterator()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const iterable1 = new Object();

                iterable1[wasm.symbol_iterator()] = function* () {
                    yield 1;
                    yield 2;
                    yield 3;
                };

                assert.deepEqual([...iterable1], [1, 2, 3]);
            }
        "#,
        )
        .test();
}

#[test]
fn match_() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn symbol_match() -> js::Symbol {
                js::Symbol::match_()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const regexp1 = /foo/;
                assert.throws(() => '/foo/'.startsWith(regexp1));

                regexp1[wasm.symbol_match()] = false;

                assert.ok('/foo/'.startsWith(regexp1));

                assert.equal('/baz/'.endsWith(regexp1), false);
            }
        "#,
        )
        .test();
}

#[test]
fn replace() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn symbol_replace() -> js::Symbol {
                js::Symbol::replace()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                class Replace1 {
                    constructor(value) {
                        this.value = value;
                    }
                    [wasm.symbol_replace()](string) {
                        return `s/${string}/${this.value}/g`;
                    }
                }

                export function test() {
                    assert.equal('foo'.replace(new Replace1('bar')), 's/foo/bar/g');
                }
            "#,
        )
        .test();
}

#[test]
fn search() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn symbol_search() -> js::Symbol {
                js::Symbol::search()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            class Search1 {
                constructor(value) {
                    this.value = value;
                }

                [wasm.symbol_search()](string) {
                    return string.indexOf(this.value);
                }
            }

            export function test() {
                assert.equal('foobar'.search(new Search1('bar')), 3);
            }
        "#,
        )
        .test();
}

#[test]
fn species() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn symbol_species() -> js::Symbol {
                js::Symbol::species()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            class Array1 extends Array {
                static get [wasm.symbol_species()]() { return Array; }
            }

            export function test() {
                const a = new Array1(1, 2, 3);
                const mapped = a.map(x => x * x);

                assert.equal(mapped instanceof Array1, false);

                assert.ok(mapped instanceof Array);
            }
        "#,
        )
        .test();
}

#[test]
fn split() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn symbol_split() -> js::Symbol {
                js::Symbol::split()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            class Split1 {
              constructor(value) {
                  this.value = value;
              }

              [wasm.symbol_split()](string) {
                  var index = string.indexOf(this.value);
                  return this.value + string.substr(0, index) + "/"
                      + string.substr(index + this.value.length);
              }
            }

            export function test() {
                assert.equal('foobar'.split(new Split1('foo')), 'foo/bar');
            }
        "#,
        )
        .test();
}

#[test]
fn to_primitive() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn symbol_to_primitive() -> js::Symbol {
                js::Symbol::to_primitive()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object1 = {
                    [wasm.symbol_to_primitive()](hint) {
                        if (hint == 'number') {
                            return 42;
                        }
                        return null;
                    }
                };

                assert.equal(+object1, 42);
            }
        "#,
        )
        .test();
}

#[test]
fn to_string_tag() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn symbol_to_string_tag() -> js::Symbol {
                js::Symbol::to_string_tag()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            class ValidatorClass {
                get [wasm.symbol_to_string_tag()]() {
                    return 'Validator';
                }
            }

            export function test() {
                assert.equal(Object.prototype.toString.call(new ValidatorClass()), '[object Validator]');
            }
        "#,
        )
        .test();
}
