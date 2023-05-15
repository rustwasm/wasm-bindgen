use crate::generated::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn take_and_return_a_bunch_of_slices() {
    let f = ArrayBufferTest::new().unwrap();
    let x = f.get_buffer();
    f.set_buffer(None);
    f.set_buffer(Some(&x));
    let buf = f.get_data_view();
    assert_eq!(buf.get_int8(0), 42);
}
