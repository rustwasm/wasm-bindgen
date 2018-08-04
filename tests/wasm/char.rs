use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests/wasm/char.js", version = "*")]
extern {
    fn char_works();
    fn js_parrot(c: char) -> char;
}

#[wasm_bindgen_test]
fn works() {
    char_works();
}

#[wasm_bindgen]
pub fn char_single_char() -> char { 'a' }
#[wasm_bindgen]
pub fn char_wide_char() -> char { '💩' }

#[wasm_bindgen]
pub fn char_parrot(c: char) -> char { c }

#[wasm_bindgen]
pub fn char_short_test(a: char) { assert_eq!(a, 'a'); }
#[wasm_bindgen]
pub fn char_wide_test(p: char) { assert_eq!(p, '💩'); }

#[wasm_bindgen]
pub fn char_round(c: char)-> char { js_parrot(c) }
