use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use web_sys::Headers;

#[wasm_bindgen(module = "/tests/wasm/headers.js")]
extern "C" {
    fn new_headers() -> Headers;
    fn new_headers_2() -> Headers;
}

#[wasm_bindgen_test]
fn headers() {
    let headers = new_headers();
    assert_eq!(headers.get("foo").unwrap(), None);
    assert_eq!(
        headers.get("content-type").unwrap(),
        Some("text/plain".to_string()),
    );
    assert_eq!(
        headers.get("Content-Type").unwrap(),
        Some("text/plain".to_string()),
    );
    assert!(headers.get("").is_err());
    assert!(headers.set("", "").is_err());
    assert!(headers.set("x", "").is_ok());
    assert_eq!(headers.get("x").unwrap(), Some(String::new()));
    assert!(headers.delete("x").is_ok());
    assert_eq!(headers.get("x").unwrap(), None);
    assert!(headers.append("a", "y").is_ok());
    assert!(headers.append("a", "z").is_ok());
    assert_eq!(headers.get("a").unwrap(), Some("y, z".to_string()));
}

#[wasm_bindgen_test]
fn headers_iter() {
    let headers = new_headers_2()
        .entries()
        .into_iter()
        .map(|x| {
            let array = x.unwrap().dyn_into::<Array>().unwrap();
            assert_eq!(array.length(), 2);
            let k: String = array.at(0).as_string().unwrap();
            let v: String = array.at(1).as_string().unwrap();
            (k, v)
        })
        .collect::<Vec<(String, String)>>();

    assert_eq!(headers.len(), 2);
    assert_eq!(&headers[0].0, "content-type");
    assert_eq!(&headers[0].1, "text/plain");
    assert_eq!(&headers[1].0, "cookie");
    assert_eq!(&headers[1].1, "foobarbaz");
}

#[wasm_bindgen_test]
fn headers_for_each() {
    let mut count = 0;
    let cb = Closure::wrap(Box::new(move |jval: JsValue| {
        let val: String = jval.as_string().unwrap();
        if count == 0 {
            assert_eq!(val, "text/plain");
        } else {
            assert_eq!(val, "foobarbaz");
        }
        count += 1;
    }) as Box<dyn FnMut(JsValue)>);
    let res = new_headers_2().for_each(cb.as_ref().unchecked_ref());
    assert!(res.is_ok());
}
