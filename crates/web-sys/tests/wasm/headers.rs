use std::collections::HashMap;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_test::*;
use web_sys::Headers;

#[wasm_bindgen(module = "/tests/wasm/headers.js")]
extern "C" {
    fn new_headers() -> Headers;
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
    let headers_map: HashMap<String, String> = collect_headers(headers.iter());
    assert_eq!(headers_map.len(), 2);
    assert_eq!(
        headers_map.get("content-type"),
        Some(&"text/plain".to_string())
    );
    assert_eq!(headers_map.get("a"), Some(&"y, z".to_string()));
    // try creating the headers object in rust
    let headers = Headers::new().unwrap();
    assert!(headers.set("Content-Type", "text/plain").is_ok());
    for (key, value) in headers.iter() {
        assert_eq!(key.as_string().unwrap(), "content-type");
        assert_eq!(value.as_string().unwrap(), "text/plain");
    }
}

fn collect_headers(iter: impl Iterator<Item = (JsValue, JsValue)>) -> HashMap<String, String> {
    iter.map(|(key, val)| (key.as_string().unwrap(), val.as_string().unwrap()))
        .collect()
}
