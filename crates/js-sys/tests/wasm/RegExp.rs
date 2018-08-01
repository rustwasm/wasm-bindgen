use wasm_bindgen_test::*;
use js_sys::*;

#[wasm_bindgen_test]
fn exec() {

    let re = RegExp::new("quick\\s(brown).+?(jumps)", "ig");
    let result = re.exec("The Quick Brown Fox Jumps Over The Lazy Dog");

    let mut v = vec![];
    result.unwrap().for_each(&mut |x, _, _| v.push(x));

    assert_eq!(v[0], "Quick Brown Fox Jumps");
    assert_eq!(v[1], "Brown");
    assert_eq!(v[2], "Jumps");

    let result = re.exec("foo");
    assert!(result.is_none());
}

#[wasm_bindgen_test]
fn flags() {
    let re = RegExp::new("foo", "ig");
    assert_eq!(re.flags(), "gi");
}

#[wasm_bindgen_test]
fn global() {
    let re = RegExp::new("foo", "g");
    assert!(re.global());

    let re = RegExp::new("bar", "i");
    assert!(!re.global());
}

#[wasm_bindgen_test]
fn ignore_case() {
    let re = RegExp::new("foo", "");
    assert!(!re.ignore_case());

    let re = RegExp::new("foo", "i");
    assert!(re.ignore_case());
}

#[wasm_bindgen_test]
fn input() {
    let re = RegExp::new("hi", "g");
    re.test("hi there!");
    assert_eq!(RegExp::input(), "hi there!");
}

#[wasm_bindgen_test]
fn last_match() {
    let re = RegExp::new("hi", "g");
    re.test("hi there!");
    assert_eq!(RegExp::last_match(), "hi");
}

#[wasm_bindgen_test]
fn last_paren() {
    let re = RegExp::new("(hi)", "g");
    re.test("hi there!");
    assert_eq!(RegExp::last_paren(), "hi");
}

#[wasm_bindgen_test]
fn left_context() {
    let re = RegExp::new("world", "g");
    re.test("hello world!");
    assert_eq!(RegExp::left_context(), "hello ");
}

#[wasm_bindgen_test]
fn multiline() {
    let re = RegExp::new("foo", "m");
    assert!(re.multiline());
}

#[wasm_bindgen_test]
fn new() {
    let re = RegExp::new("foo", "");
    let re = RegExp::new_regexp(&re, "g");
    assert_eq!(re.to_string(), "/foo/g");
}

#[wasm_bindgen_test]
fn right_context() {
    let re = RegExp::new("hello", "g");
    re.test("hello world!");
    assert_eq!(RegExp::right_context(), " world!");
}

#[wasm_bindgen_test]
fn source() {
    let re = RegExp::new("fooBar", "ig");
    assert_eq!(re.source(), "fooBar");

    let re = RegExp::new("", "ig");
    assert_eq!(re.source(), "(?:)");
}

#[wasm_bindgen_test]
fn sticky() {
    let re = RegExp::new("foo", "y");
    assert!(re.sticky());
}

#[wasm_bindgen_test]
fn test() {
    let re = RegExp::new("foo", "");
    assert!(re.test("football"));
    assert!(!re.test("bar"));
}

#[wasm_bindgen_test]
fn to_string() {
    let re = RegExp::new("a+b+c", "g");
    assert_eq!(re.to_string(), "/a+b+c/g");
}

#[wasm_bindgen_test]
fn unicode() {
    let re = RegExp::new("\u{61}", "u");
    assert!(re.unicode());
}
