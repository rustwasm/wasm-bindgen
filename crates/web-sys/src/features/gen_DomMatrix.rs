use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = DomMatrixReadOnly , extends = :: js_sys :: Object , js_name = DOMMatrix , typescript_type = "DOMMatrix" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DomMatrix` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub type DomMatrix;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = a ) ]
    ///Getter for the `a` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/a)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn a(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = a ) ]
    ///Setter for the `a` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/a)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_a(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = b ) ]
    ///Getter for the `b` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/b)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn b(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = b ) ]
    ///Setter for the `b` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/b)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_b(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = c ) ]
    ///Getter for the `c` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/c)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn c(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = c ) ]
    ///Setter for the `c` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/c)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_c(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = d ) ]
    ///Getter for the `d` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/d)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn d(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = d ) ]
    ///Setter for the `d` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/d)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_d(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = e ) ]
    ///Getter for the `e` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/e)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn e(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = e ) ]
    ///Setter for the `e` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/e)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_e(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = f ) ]
    ///Getter for the `f` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/f)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn f(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = f ) ]
    ///Setter for the `f` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/f)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_f(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = m11 ) ]
    ///Getter for the `m11` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m11)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn m11(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = m11 ) ]
    ///Setter for the `m11` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m11)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_m11(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = m12 ) ]
    ///Getter for the `m12` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m12)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn m12(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = m12 ) ]
    ///Setter for the `m12` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m12)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_m12(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = m13 ) ]
    ///Getter for the `m13` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m13)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn m13(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = m13 ) ]
    ///Setter for the `m13` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m13)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_m13(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = m14 ) ]
    ///Getter for the `m14` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m14)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn m14(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = m14 ) ]
    ///Setter for the `m14` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m14)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_m14(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = m21 ) ]
    ///Getter for the `m21` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m21)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn m21(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = m21 ) ]
    ///Setter for the `m21` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m21)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_m21(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = m22 ) ]
    ///Getter for the `m22` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m22)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn m22(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = m22 ) ]
    ///Setter for the `m22` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m22)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_m22(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = m23 ) ]
    ///Getter for the `m23` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m23)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn m23(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = m23 ) ]
    ///Setter for the `m23` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m23)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_m23(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = m24 ) ]
    ///Getter for the `m24` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m24)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn m24(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = m24 ) ]
    ///Setter for the `m24` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m24)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_m24(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = m31 ) ]
    ///Getter for the `m31` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m31)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn m31(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = m31 ) ]
    ///Setter for the `m31` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m31)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_m31(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = m32 ) ]
    ///Getter for the `m32` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m32)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn m32(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = m32 ) ]
    ///Setter for the `m32` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m32)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_m32(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = m33 ) ]
    ///Getter for the `m33` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m33)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn m33(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = m33 ) ]
    ///Setter for the `m33` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m33)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_m33(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = m34 ) ]
    ///Getter for the `m34` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m34)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn m34(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = m34 ) ]
    ///Setter for the `m34` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m34)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_m34(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = m41 ) ]
    ///Getter for the `m41` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m41)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn m41(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = m41 ) ]
    ///Setter for the `m41` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m41)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_m41(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = m42 ) ]
    ///Getter for the `m42` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m42)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn m42(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = m42 ) ]
    ///Setter for the `m42` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m42)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_m42(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = m43 ) ]
    ///Getter for the `m43` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m43)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn m43(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = m43 ) ]
    ///Setter for the `m43` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m43)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_m43(this: &DomMatrix, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrix" , js_name = m44 ) ]
    ///Getter for the `m44` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m44)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn m44(this: &DomMatrix) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DOMMatrix" , js_name = m44 ) ]
    ///Setter for the `m44` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m44)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_m44(this: &DomMatrix, value: f64);

    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrix")]
    ///The `new DomMatrix(..)` constructor, creating a new instance of `DomMatrix`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn new() -> Result<DomMatrix, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrix")]
    ///The `new DomMatrix(..)` constructor, creating a new instance of `DomMatrix`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn new_with_transform_list(transform_list: &str) -> Result<DomMatrix, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrix")]
    ///The `new DomMatrix(..)` constructor, creating a new instance of `DomMatrix`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn new_with_other(other: &DomMatrixReadOnly) -> Result<DomMatrix, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrix")]
    ///The `new DomMatrix(..)` constructor, creating a new instance of `DomMatrix`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn new_with_array32(array32: &mut [f32]) -> Result<DomMatrix, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrix")]
    ///The `new DomMatrix(..)` constructor, creating a new instance of `DomMatrix`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn new_with_array64(array64: &mut [f64]) -> Result<DomMatrix, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrix")]
    ///The `new DomMatrix(..)` constructor, creating a new instance of `DomMatrix`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/DOMMatrix)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn new_with_number_sequence(
        number_sequence: &::wasm_bindgen::JsValue,
    ) -> Result<DomMatrix, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = invertSelf ) ]
    ///The `invertSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/invertSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn invert_self(this: &DomMatrix) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = multiplySelf ) ]
    ///The `multiplySelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/multiplySelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn multiply_self(this: &DomMatrix, other: &DomMatrix) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = preMultiplySelf ) ]
    ///The `preMultiplySelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/preMultiplySelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn pre_multiply_self(this: &DomMatrix, other: &DomMatrix) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = rotateAxisAngleSelf ) ]
    ///The `rotateAxisAngleSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateAxisAngleSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn rotate_axis_angle_self(
        this: &DomMatrix,
        x: f64,
        y: f64,
        z: f64,
        angle: f64,
    ) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = rotateFromVectorSelf ) ]
    ///The `rotateFromVectorSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateFromVectorSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn rotate_from_vector_self(this: &DomMatrix, x: f64, y: f64) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = rotateSelf ) ]
    ///The `rotateSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn rotate_self(this: &DomMatrix, angle: f64) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = rotateSelf ) ]
    ///The `rotateSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn rotate_self_with_origin_x(this: &DomMatrix, angle: f64, origin_x: f64) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = rotateSelf ) ]
    ///The `rotateSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn rotate_self_with_origin_x_and_origin_y(
        this: &DomMatrix,
        angle: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = scale3dSelf ) ]
    ///The `scale3dSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn scale3d_self(this: &DomMatrix, scale: f64) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = scale3dSelf ) ]
    ///The `scale3dSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn scale3d_self_with_origin_x(this: &DomMatrix, scale: f64, origin_x: f64) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = scale3dSelf ) ]
    ///The `scale3dSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn scale3d_self_with_origin_x_and_origin_y(
        this: &DomMatrix,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = scale3dSelf ) ]
    ///The `scale3dSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn scale3d_self_with_origin_x_and_origin_y_and_origin_z(
        this: &DomMatrix,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
        origin_z: f64,
    ) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = scaleNonUniformSelf ) ]
    ///The `scaleNonUniformSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn scale_non_uniform_self(this: &DomMatrix, scale_x: f64) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = scaleNonUniformSelf ) ]
    ///The `scaleNonUniformSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn scale_non_uniform_self_with_scale_y(
        this: &DomMatrix,
        scale_x: f64,
        scale_y: f64,
    ) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = scaleNonUniformSelf ) ]
    ///The `scaleNonUniformSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn scale_non_uniform_self_with_scale_y_and_scale_z(
        this: &DomMatrix,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
    ) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = scaleNonUniformSelf ) ]
    ///The `scaleNonUniformSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x(
        this: &DomMatrix,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
    ) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = scaleNonUniformSelf ) ]
    ///The `scaleNonUniformSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_and_origin_y(
        this: &DomMatrix,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = scaleNonUniformSelf ) ]
    ///The `scaleNonUniformSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleNonUniformSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn scale_non_uniform_self_with_scale_y_and_scale_z_and_origin_x_and_origin_y_and_origin_z(
        this: &DomMatrix,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
        origin_y: f64,
        origin_z: f64,
    ) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = scaleSelf ) ]
    ///The `scaleSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn scale_self(this: &DomMatrix, scale: f64) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = scaleSelf ) ]
    ///The `scaleSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn scale_self_with_origin_x(this: &DomMatrix, scale: f64, origin_x: f64) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = scaleSelf ) ]
    ///The `scaleSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn scale_self_with_origin_x_and_origin_y(
        this: &DomMatrix,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DOMMatrix" , js_name = setMatrixValue ) ]
    ///The `setMatrixValue()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/setMatrixValue)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn set_matrix_value(this: &DomMatrix, transform_list: &str) -> Result<DomMatrix, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = skewXSelf ) ]
    ///The `skewXSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/skewXSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn skew_x_self(this: &DomMatrix, sx: f64) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = skewYSelf ) ]
    ///The `skewYSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/skewYSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn skew_y_self(this: &DomMatrix, sy: f64) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = translateSelf ) ]
    ///The `translateSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/translateSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn translate_self(this: &DomMatrix, tx: f64, ty: f64) -> DomMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrix" , js_name = translateSelf ) ]
    ///The `translateSelf()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/translateSelf)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`*
    pub fn translate_self_with_tz(this: &DomMatrix, tx: f64, ty: f64, tz: f64) -> DomMatrix;

}
