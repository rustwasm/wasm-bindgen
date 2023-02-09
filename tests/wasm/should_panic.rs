#[wasm_bindgen_test]
#[should_panic]
fn should_panic() {
    panic!()
}

#[wasm_bindgen_test]
#[should_panic = "error message"]
fn should_panic_expected() {
    panic!("error message")
}
