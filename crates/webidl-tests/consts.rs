use crate::generated::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn bool() {
    let falsish: bool = ConstBool::NOT_TRUE;
    assert!(!falsish);
    let trueish: bool = ConstBool::NOT_FALSE;
    assert!(trueish);
}

#[wasm_bindgen_test]
fn ints() {
    assert_eq!(ConstByte::IMIN, i8::min_value());
    assert_eq!(ConstByte::IMAX, i8::max_value());
    assert_eq!(ConstByte::UMIN, u8::min_value());
    assert_eq!(ConstByte::UMAX, u8::max_value());

    assert_eq!(ConstShort::IMIN, i16::min_value());
    assert_eq!(ConstShort::IMAX, i16::max_value());
    assert_eq!(ConstShort::UMIN, u16::min_value());
    assert_eq!(ConstShort::UMAX, u16::max_value());

    assert_eq!(ConstLong::IMIN, i32::min_value());
    assert_eq!(ConstLong::IMAX, i32::max_value());
    assert_eq!(ConstLong::UMIN, u32::min_value());
    assert_eq!(ConstLong::UMAX, u32::max_value());
}

#[wasm_bindgen_test]
fn floats() {
    assert_eq!(ConstFloats::F, 0.0_f32);
    assert!(ConstFloats::NEG_INF.is_infinite());
    assert!(ConstFloats::NEG_INF.is_sign_negative());
    assert!(ConstFloats::INF.is_infinite());
    assert!(ConstFloats::INF.is_sign_positive());
    assert!(ConstFloats::NAN.is_nan());

    assert_eq!(ConstDoubles::D, 0.0_f64);
    assert!(ConstDoubles::NEG_INF.is_infinite());
    assert!(ConstDoubles::NEG_INF.is_sign_negative());
    assert!(ConstDoubles::INF.is_infinite());
    assert!(ConstDoubles::INF.is_sign_positive());
    assert!(ConstDoubles::NAN.is_nan());
    assert_eq!(ConstDoubles::ONE, 1.0);
}
