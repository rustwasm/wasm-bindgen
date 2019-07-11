//! A collection of tests to exercise imports where we don't need to generate a
//! JS shim to convert arguments/returns even when Web IDL bindings is not
//! implemented.

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(inline_js = "
    function assert_eq(a, b) {
        if (a !== b) {
            throw new Error(`assert_eq failed: ${a} != ${b}`);
        }
    }

    module.exports.trivial = function () {};

    module.exports.incoming_i32 = function () { return 0; };
    module.exports.incoming_f32 = function () { return 1.5; };
    module.exports.incoming_f64 = function () { return 13.37; };

    module.exports.outgoing_i32 = function (x) { assert_eq(x, 0); };
    module.exports.outgoing_f32 = function (y) { assert_eq(y, 1.5); };
    module.exports.outgoing_f64 = function (z) { assert_eq(z, 13.37); };

    module.exports.many = function (x, y, z) {
        assert_eq(x, 0);
        assert_eq(y, 1.5);
        assert_eq(z, 13.37);
        return 42;
    };

    module.exports.works_when_anyref_support_is_enabled = function (v) {
        assert_eq(v, 'hello');
        return v;
    };
")]
extern "C" {
    #[wasm_bindgen(assert_no_shim)]
    fn trivial();

    #[wasm_bindgen(assert_no_shim)]
    fn incoming_i32() -> i32;
    #[wasm_bindgen(assert_no_shim)]
    fn incoming_f32() -> f32;
    #[wasm_bindgen(assert_no_shim)]
    fn incoming_f64() -> f64;

    #[wasm_bindgen(assert_no_shim)]
    fn outgoing_i32(x: i32);
    #[wasm_bindgen(assert_no_shim)]
    fn outgoing_f32(y: f32);
    #[wasm_bindgen(assert_no_shim)]
    fn outgoing_f64(z: f64);

    #[wasm_bindgen(assert_no_shim)]
    fn many(x: i32, y: f32, z: f64) -> i32;

    // Note that this should only skip the JS shim if we have anyref support
    // enabled.
    //
    // #[wasm_bindgen(assert_no_shim)]
    fn works_when_anyref_support_is_enabled(v: JsValue) -> JsValue;
}

#[wasm_bindgen_test]
fn no_shims() {
    trivial();

    let x = incoming_i32();
    assert_eq!(x, 0);
    let y = incoming_f32();
    assert_eq!(y, 1.5);
    let z = incoming_f64();
    assert_eq!(z, 13.37);

    outgoing_i32(x);
    outgoing_f32(y);
    outgoing_f64(z);

    let w = many(x, y, z);
    assert_eq!(w, 42);

    let v = JsValue::from("hello");
    let vv = works_when_anyref_support_is_enabled(v.clone());
    assert_eq!(v, vv);
}
