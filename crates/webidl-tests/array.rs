use wasm_bindgen::Clamped;
use wasm_bindgen_test::*;

include!(concat!(env!("OUT_DIR"), "/array.rs"));

#[wasm_bindgen_test]
fn take_and_return_a_bunch_of_slices() {
    let f = TestArrays::new().unwrap();
    assert_eq!(f.strings("y"), "x");
    assert_eq!(f.byte_strings("yz"), "xx");
    assert_eq!(f.usv_strings("abc"), "efg");
    assert_eq!(f.f32(&mut [1.0, 2.0]), [3.0, 4.0, 5.0]);
    assert_eq!(f.f64(&mut [1.0, 2.0]), [3.0, 4.0, 5.0]);
    assert_eq!(f.i8(&mut [1, 2]), [3, 4, 5]);
    assert_eq!(f.i16(&mut [1, 2]), [3, 4, 5]);
    assert_eq!(f.i32(&mut [1, 2]), [3, 4, 5]);
    assert_eq!(f.u8(&mut [1, 2]), [3, 4, 5]);
    assert_eq!(f.u16(&mut [1, 2]), [3, 4, 5]);
    assert_eq!(f.u32(&mut [1, 2]), [3, 4, 5]);
    assert_eq!(f.u8_clamped(Clamped(&mut [1, 2])).0, [3, 4, 5]);
}
