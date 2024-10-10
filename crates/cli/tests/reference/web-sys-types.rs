// DEPENDENCY: web-sys = { path = '{root}/crates/web-sys', features = ['console', 'Url', 'MediaSourceReadyState'] }

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{Url, MediaSourceReadyState};

#[wasm_bindgen]
pub fn get_url() -> Url {
    assert_eq!(MediaSourceReadyState::Closed, MediaSourceReadyState::Closed);
    Url::new("https://example.com").unwrap()
}
