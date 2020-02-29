use wasm_bindgen_test::*;
use crate::generated::*;

#[wasm_bindgen_test]
fn take_and_return_a_bunch_of_slices() {
    let f = ArrayBufferTest::new().unwrap();
    let x = f.get_buffer();
    f.set_buffer(None);
    f.set_buffer(Some(&x));
}
