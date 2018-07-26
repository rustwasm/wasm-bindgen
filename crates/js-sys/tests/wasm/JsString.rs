use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use js_sys::*;

#[wasm_bindgen_test]
fn length() {
    fn test(s: &str) {
        assert_eq!(JsString::from(s).length(), s.len() as u32);
    }
    test("Mozilla");
    test("");
}

#[wasm_bindgen_test]
fn char_at() {
    let s = JsString::from("Brave new world");
    assert_eq!(JsValue::from(s.char_at(0)), "B");
    assert_eq!(JsValue::from(s.char_at(999)), "");
}

#[wasm_bindgen_test]
fn char_code_at() {
    let s = "Brave new world";
    let js = JsString::from(s);
    for (i, b) in s.char_indices() {
        assert_eq!(js.char_code_at(i as u32), b as u32 as f64);
    }
    assert!(js.char_code_at(s.len() as u32).is_nan());
}

#[wasm_bindgen_test]
fn code_point_at() {
    assert_eq!(JsString::from("ABC").code_point_at(1), b'B');
    assert!(JsString::from("ABC").code_point_at(42).is_undefined());
}

#[wasm_bindgen_test]
fn concat() {
    // TODO: Implement ability to receive multiple optional arguments
    let s = JsString::from("Hello ").concat(&"World".into());
    assert_eq!(JsValue::from(s), "Hello World");
    let foo = JsString::from("foo");
    assert_eq!(JsValue::from(foo.concat(&Object::new().into())), "foo[object Object]");
    assert_eq!(JsValue::from(foo.concat(&Array::new().into())), "foo");
    assert_eq!(JsValue::from(foo.concat(&JsValue::null())), "foonull");
    assert_eq!(JsValue::from(foo.concat(&true.into())), "footrue");
    assert_eq!(JsValue::from(foo.concat(&1234.into())), "foo1234");
}

#[wasm_bindgen_test]
fn ends_with() {
    let s = "To be, or not to be, that is the question.";
    let js = JsString::from(s);

    // TODO: remove third parameter once we have optional parameters
    assert_eq!(js.ends_with("question.", s.len() as i32), true);
    assert_eq!(js.ends_with("to be", s.len() as i32), false);
    assert_eq!(js.ends_with("to be", 19), true);
}

#[wasm_bindgen_test]
fn includes() {
    let str = JsString::from("Blue Whale");

    // TODO: remove second parameter once we have optional parameters
    assert_eq!(str.includes("Blue", 0), true);
    assert_eq!(str.includes("Blute", 0), false);
    assert_eq!(str.includes("Whale", 0), true);
    assert_eq!(str.includes("Whale", 5), true);
    assert_eq!(str.includes("Whale", 7), false);
    assert_eq!(str.includes("", 0), true);
    assert_eq!(str.includes("", 16), true);
}

#[wasm_bindgen_test]
fn index_of() {
    let str = JsString::from("Blue Whale");

    // TODO: remove second parameter once we have optional parameters
    assert_eq!(str.index_of("Blue", 0), 0);
    // TODO: remove second parameter once we have optional parameters
    assert_eq!(str.index_of("Blute", 0), -1);
    assert_eq!(str.index_of("Whale", 0), 5);
    assert_eq!(str.index_of("Whale", 5), 5);
    assert_eq!(str.index_of("Whale", 7), -1);
    // TODO: remove second parameter once we have optional parameters
    assert_eq!(str.index_of("", 0), 0);
    assert_eq!(str.index_of("", 9), 9);
    assert_eq!(str.index_of("", 10), 10);
    assert_eq!(str.index_of("", 11), 10);
}

#[wasm_bindgen_test]
fn last_index_of() {
    let js = JsString::from("canal");
    let len = js.length() as i32;

    // TODO: remove second parameter once we have optional parameters
    assert_eq!(js.last_index_of("a", len), 3);
    assert_eq!(js.last_index_of("a", 2), 1);
    assert_eq!(js.last_index_of("a", 0), -1);
    // TODO: remove second parameter once we have optional parameters
    assert_eq!(js.last_index_of("x", len), -1);
    assert_eq!(js.last_index_of("c", -5), 0);
    assert_eq!(js.last_index_of("c", 0), 0);
    // TODO: remove second parameter once we have optional parameters
    assert_eq!(js.last_index_of("", len), 5);
    assert_eq!(js.last_index_of("", 2), 2);
}

#[wasm_bindgen_test]
fn normalize() {
    let js = JsString::from("\u{1E9B}\u{0323}");

    // TODO: Handle undefined
    assert_eq!(JsValue::from(js.normalize("NFC")), "\u{1E9B}\u{0323}");
    assert_eq!(JsValue::from(js.normalize("NFD")), "\u{017F}\u{0323}\u{0307}");
    assert_eq!(JsValue::from(js.normalize("NFKC")), "\u{1E69}");
    assert_eq!(JsValue::from(js.normalize("NFKD")), "\u{0073}\u{0323}\u{0307}");
}

#[wasm_bindgen_test]
fn pad_end() {
    let js = JsString::from("abc");

    // TODO: remove second parameter once we have optional parameters
    assert_eq!(JsValue::from(js.pad_end(10, " ")), "abc       ");
    // TODO: remove second parameter once we have optional parameters
    assert_eq!(JsValue::from(js.pad_end(10, " ")), "abc       ");
    assert_eq!(JsValue::from(js.pad_end(10, "foo")), "abcfoofoof");
    assert_eq!(JsValue::from(js.pad_end(6, "123456")), "abc123");
    // TODO: remove second parameter once we have optional parameters
    assert_eq!(JsValue::from(js.pad_end(1, " ")), "abc");
}

#[wasm_bindgen_test]
fn pad_start() {
    let js = JsString::from("abc");

    // TODO: remove second parameter once we have optional parameters
    assert_eq!(js.pad_start(10, " "), "       abc");
    assert_eq!(js.pad_start(10, "foo"), "foofoofabc");
    assert_eq!(js.pad_start(6, "123465"), "123abc");
    assert_eq!(js.pad_start(8, "0"), "00000abc");
    // TODO: remove second parameter once we have optional parameters
    assert_eq!(js.pad_start(1, " "), "abc");
}

#[wasm_bindgen_test]
fn repeat() {
    assert_eq!(JsString::from("test").repeat(3), "testtesttest");
}

#[wasm_bindgen_test]
fn slice() {
    let characters = JsString::from("acxn18");
    assert_eq!(characters.slice(1, 3), "cx");
}

#[wasm_bindgen_test]
fn starts_with() {
    let js = JsString::from("To be, or not to be, that is the question.");

    // TODO: remove second parameter for both assertions once we have optional parameters
    assert!(js.starts_with("To be", 0));
    assert!(!js.starts_with("not to be", 0));
    assert!(js.starts_with("not to be", 10));
}

#[wasm_bindgen_test]
fn substring() {
    let js = JsString::from("Mozilla");

    assert_eq!(js.substring(0, 1), "M");
    assert_eq!(js.substring(1, 0), "M");

    assert_eq!(js.substring(0, 6), "Mozill");

    // TODO: Add test once we have optional parameters
    // assert_eq!(js.substring(4), "lla");
    assert_eq!(js.substring(4, 7), "lla");
    assert_eq!(js.substring(7, 4), "lla");

    assert_eq!(js.substring(0, 7), "Mozilla");
    assert_eq!(js.substring(0, 10), "Mozilla");
}

#[wasm_bindgen_test]
fn substr() {
    let js = JsString::from("Mozilla");

    assert_eq!(js.substr(0, 1), "M");
    assert_eq!(js.substr(1, 0), "");
    assert_eq!(js.substr(-1, 1), "a");
    assert_eq!(js.substr(1, -1), "");
    // TODO: Uncomment and test these assertions, once we have support for optional parameters
    // assert_eq!(js.substr(-3), "lla");
    // assert_eq!(js.substr(1), "ozilla");
    assert_eq!(js.substr(-20, 2), "Mo");
    assert_eq!(js.substr(20, 2), "");
}

#[wasm_bindgen_test]
fn to_locale_lower_case() {
    let js = JsString::from("Mozilla");
    assert_eq!(js.to_locale_lower_case(None), "mozilla");
    let s = JsString::from("\u{0130}");
    assert_eq!(s.to_locale_lower_case(Some("tr".into())), "i");
    assert_ne!(s.to_locale_lower_case(Some("en-US".into())), "i");
}

#[wasm_bindgen_test]
fn to_locale_upper_case() {
    let js = JsString::from("mozilla");
    assert_eq!(js.to_locale_upper_case(None), "MOZILLA");
    let s = JsString::from("i\u{0307}");
    assert_eq!(s.to_locale_upper_case(Some("lt".into())), "I");
    assert_ne!(s.to_locale_upper_case(Some("en-US".into())), "I");
}

#[wasm_bindgen_test]
fn to_lower_case() {
    assert_eq!(JsString::from("Mozilla").to_lower_case(), "mozilla");
}

#[wasm_bindgen_test]
fn to_string() {
    assert_eq!(JsString::from("foo").to_string(), "foo");
}

#[wasm_bindgen_test]
fn to_upper_case() {
    assert_eq!(JsString::from("Mozilla").to_upper_case(), "MOZILLA");
}

#[wasm_bindgen_test]
fn trim() {
    assert_eq!(JsString::from("   foo  ").trim(), "foo");
    // Another example of .trim() removing whitespace from just one side.
    assert_eq!(JsString::from("foo   ").trim(), "foo");
}

#[wasm_bindgen_test]
fn trim_end_and_trim_right() {
    let greeting = JsString::from("   Hello world!   ");
    let trimmed = "   Hello world!";
    assert_eq!(greeting.trim_end(), trimmed);
    assert_eq!(greeting.trim_right(), trimmed);
}

#[wasm_bindgen_test]
fn trim_start_and_trim_left() {
    let greeting = JsString::from("   Hello world!   ");
    let trimmed = "Hello world!   ";
    assert_eq!(greeting.trim_start(), trimmed);
    assert_eq!(greeting.trim_left(), trimmed);
}

#[wasm_bindgen_test]
fn value_of() {
    let greeting = JsString::from("Hello world!");
    assert_eq!(greeting.value_of(), "Hello world!");
}
