use super::project;

#[test]
fn works() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn js_parrot(c: char)  -> char;
            }

            #[wasm_bindgen]
            pub fn single_char() -> char { 'a' }
            #[wasm_bindgen]
            pub fn wide_char() -> char { '💩' }

            #[wasm_bindgen]
            pub fn parrot(c: char) -> char { c }

            #[wasm_bindgen]
            pub fn short_test(a: char) { assert_eq!(a, 'a'); }
            #[wasm_bindgen]
            pub fn wide_test(p: char) { assert_eq!(p, '💩'); }

            #[wasm_bindgen]
            pub fn char_round(c: char)-> char { js_parrot(c) }
        "#)
        .file("test.js", r#"
            import * as wasm from './out';

            function assertEq(a, b) {
              console.log(a, '?=', b);
              if (a === b)
                return;
              throw new Error('not equal');
            }

            export function test() {
              assertEq(wasm.single_char(), 'a');
              assertEq(wasm.wide_char(), '💩');
              assertEq(wasm.parrot('Ղ'), 'Ղ');
              assertEq(wasm.parrot('ҝ'), 'ҝ');
              assertEq(wasm.parrot('Δ'), 'Δ');
              assertEq(wasm.parrot('䉨'), '䉨');
              assertEq(wasm.char_round('a'), 'a');
              assertEq(wasm.char_round('㊻'), '㊻');
              wasm.short_test('a');
              wasm.wide_test('💩');
            }

            export function js_parrot(a) { return a; }
        "#)
        .test();
}
