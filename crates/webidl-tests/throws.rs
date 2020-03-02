use crate::generated::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn throws() {
    assert!(Thang::new(0).is_err());
    let thang = Thang::new(1).unwrap();

    assert!(thang.ok_attr().is_ok());
    assert!(thang.set_ok_attr(0).is_ok());

    assert!(thang.err_attr().is_err());
    assert!(thang.set_err_attr(0).is_err());

    assert!(thang.ok_method().is_ok());
    assert!(thang.err_method().is_err());

    assert!(Thang::ok_static_method().is_ok());
    assert!(Thang::err_static_method().is_err());

    assert!(Thang::ok_static_attr().is_ok());
    assert!(Thang::set_ok_static_attr(0).is_ok());

    assert!(Thang::err_static_attr().is_err());
    assert!(Thang::set_err_static_attr(0).is_err());
}
