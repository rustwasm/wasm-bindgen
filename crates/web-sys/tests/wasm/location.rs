use wasm_bindgen_test::*;
use web_sys::Window;

#[wasm_bindgen_test]
fn href() {
    let loc = Window::location();
    loc.href().unwrap();
}

#[wasm_bindgen_test]
fn origin() {
    let loc = Window::location();
    loc.origin().unwrap();
}

#[wasm_bindgen_test]
fn protocol() {
    let loc = Window::location();
    loc.protocol().unwrap();
}

#[wasm_bindgen_test]
fn host() {
    let loc = Window::location();
    loc.host().unwrap();
}

#[wasm_bindgen_test]
fn hostname() {
    let loc = Window::location();
    loc.hostname().unwrap();
}

#[wasm_bindgen_test]
fn port() {
    let loc = Window::location();
    loc.port().unwrap();
}

#[wasm_bindgen_test]
fn pathname() {
    let loc = Window::location();
    loc.pathname().unwrap();
}

#[wasm_bindgen_test]
fn search() {
    let loc = Window::location();
    loc.search().unwrap();
}

#[wasm_bindgen_test]
fn hash() {
    let loc = Window::location();
    loc.hash().unwrap();
}
