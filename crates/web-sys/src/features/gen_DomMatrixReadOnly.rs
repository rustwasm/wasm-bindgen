use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMMatrixReadOnly , typescript_type = "DOMMatrixReadOnly" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DomMatrixReadOnly` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub type DomMatrixReadOnly;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = a ) ]
    ///Getter for the `a` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/a)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn a(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = b ) ]
    ///Getter for the `b` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/b)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn b(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = c ) ]
    ///Getter for the `c` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/c)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn c(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = d ) ]
    ///Getter for the `d` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/d)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn d(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = e ) ]
    ///Getter for the `e` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/e)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn e(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = f ) ]
    ///Getter for the `f` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/f)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn f(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = m11 ) ]
    ///Getter for the `m11` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m11)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn m11(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = m12 ) ]
    ///Getter for the `m12` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m12)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn m12(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = m13 ) ]
    ///Getter for the `m13` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m13)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn m13(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = m14 ) ]
    ///Getter for the `m14` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m14)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn m14(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = m21 ) ]
    ///Getter for the `m21` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m21)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn m21(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = m22 ) ]
    ///Getter for the `m22` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m22)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn m22(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = m23 ) ]
    ///Getter for the `m23` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m23)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn m23(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = m24 ) ]
    ///Getter for the `m24` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m24)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn m24(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = m31 ) ]
    ///Getter for the `m31` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m31)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn m31(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = m32 ) ]
    ///Getter for the `m32` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m32)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn m32(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = m33 ) ]
    ///Getter for the `m33` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m33)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn m33(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = m34 ) ]
    ///Getter for the `m34` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m34)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn m34(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = m41 ) ]
    ///Getter for the `m41` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m41)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn m41(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = m42 ) ]
    ///Getter for the `m42` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m42)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn m42(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = m43 ) ]
    ///Getter for the `m43` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m43)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn m43(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = m44 ) ]
    ///Getter for the `m44` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m44)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn m44(this: &DomMatrixReadOnly) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = is2D ) ]
    ///Getter for the `is2D` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/is2D)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn is_2d(this: &DomMatrixReadOnly) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMMatrixReadOnly" , js_name = isIdentity ) ]
    ///Getter for the `isIdentity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/isIdentity)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn is_identity(this: &DomMatrixReadOnly) -> bool;

    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrixReadOnly")]
    ///The `new DomMatrixReadOnly(..)` constructor, creating a new instance of `DomMatrixReadOnly`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/DOMMatrixReadOnly)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn new() -> Result<DomMatrixReadOnly, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrixReadOnly")]
    ///The `new DomMatrixReadOnly(..)` constructor, creating a new instance of `DomMatrixReadOnly`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/DOMMatrixReadOnly)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn new_with_str(init: &str) -> Result<DomMatrixReadOnly, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMMatrixReadOnly")]
    ///The `new DomMatrixReadOnly(..)` constructor, creating a new instance of `DomMatrixReadOnly`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/DOMMatrixReadOnly)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn new_with_f64_sequence(
        init: &::wasm_bindgen::JsValue,
    ) -> Result<DomMatrixReadOnly, JsValue>;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = flipX ) ]
    ///The `flipX()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/flipX)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn flip_x(this: &DomMatrixReadOnly) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = flipY ) ]
    ///The `flipY()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/flipY)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn flip_y(this: &DomMatrixReadOnly) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = inverse ) ]
    ///The `inverse()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/inverse)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn inverse(this: &DomMatrixReadOnly) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = multiply ) ]
    ///The `multiply()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/multiply)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn multiply(this: &DomMatrixReadOnly, other: &DomMatrix) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = rotate ) ]
    ///The `rotate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotate)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn rotate(this: &DomMatrixReadOnly, angle: f64) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = rotate ) ]
    ///The `rotate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotate)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn rotate_with_origin_x(this: &DomMatrixReadOnly, angle: f64, origin_x: f64) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = rotate ) ]
    ///The `rotate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotate)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn rotate_with_origin_x_and_origin_y(
        this: &DomMatrixReadOnly,
        angle: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = rotateAxisAngle ) ]
    ///The `rotateAxisAngle()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotateAxisAngle)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn rotate_axis_angle(
        this: &DomMatrixReadOnly,
        x: f64,
        y: f64,
        z: f64,
        angle: f64,
    ) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = rotateFromVector ) ]
    ///The `rotateFromVector()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotateFromVector)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn rotate_from_vector(this: &DomMatrixReadOnly, x: f64, y: f64) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = scale ) ]
    ///The `scale()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn scale(this: &DomMatrixReadOnly, scale: f64) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = scale ) ]
    ///The `scale()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn scale_with_origin_x(this: &DomMatrixReadOnly, scale: f64, origin_x: f64) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = scale ) ]
    ///The `scale()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn scale_with_origin_x_and_origin_y(
        this: &DomMatrixReadOnly,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = scale3d ) ]
    ///The `scale3d()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn scale3d(this: &DomMatrixReadOnly, scale: f64) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = scale3d ) ]
    ///The `scale3d()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn scale3d_with_origin_x(this: &DomMatrixReadOnly, scale: f64, origin_x: f64) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = scale3d ) ]
    ///The `scale3d()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn scale3d_with_origin_x_and_origin_y(
        this: &DomMatrixReadOnly,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = scale3d ) ]
    ///The `scale3d()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn scale3d_with_origin_x_and_origin_y_and_origin_z(
        this: &DomMatrixReadOnly,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
        origin_z: f64,
    ) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = scaleNonUniform ) ]
    ///The `scaleNonUniform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn scale_non_uniform(this: &DomMatrixReadOnly, scale_x: f64) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = scaleNonUniform ) ]
    ///The `scaleNonUniform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn scale_non_uniform_with_scale_y(
        this: &DomMatrixReadOnly,
        scale_x: f64,
        scale_y: f64,
    ) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = scaleNonUniform ) ]
    ///The `scaleNonUniform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn scale_non_uniform_with_scale_y_and_scale_z(
        this: &DomMatrixReadOnly,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
    ) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = scaleNonUniform ) ]
    ///The `scaleNonUniform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn scale_non_uniform_with_scale_y_and_scale_z_and_origin_x(
        this: &DomMatrixReadOnly,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
    ) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = scaleNonUniform ) ]
    ///The `scaleNonUniform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_and_origin_y(
        this: &DomMatrixReadOnly,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = scaleNonUniform ) ]
    ///The `scaleNonUniform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn scale_non_uniform_with_scale_y_and_scale_z_and_origin_x_and_origin_y_and_origin_z(
        this: &DomMatrixReadOnly,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
        origin_y: f64,
        origin_z: f64,
    ) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = skewX ) ]
    ///The `skewX()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/skewX)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn skew_x(this: &DomMatrixReadOnly, sx: f64) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = skewY ) ]
    ///The `skewY()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/skewY)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn skew_y(this: &DomMatrixReadOnly, sy: f64) -> DomMatrix;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DOMMatrixReadOnly" , js_name = toFloat32Array ) ]
    ///The `toFloat32Array()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/toFloat32Array)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn to_float32_array(this: &DomMatrixReadOnly) -> Result<Vec<f32>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DOMMatrixReadOnly" , js_name = toFloat64Array ) ]
    ///The `toFloat64Array()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/toFloat64Array)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn to_float64_array(this: &DomMatrixReadOnly) -> Result<Vec<f64>, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`*
    pub fn to_json(this: &DomMatrixReadOnly) -> ::js_sys::Object;

    #[cfg(feature = "DomPoint")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = transformPoint ) ]
    ///The `transformPoint()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/transformPoint)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`, `DomPoint`*
    pub fn transform_point(this: &DomMatrixReadOnly) -> DomPoint;

    #[cfg(all(feature = "DomPoint", feature = "DomPointInit",))]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = transformPoint ) ]
    ///The `transformPoint()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/transformPoint)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrixReadOnly`, `DomPoint`, `DomPointInit`*
    pub fn transform_point_with_point(this: &DomMatrixReadOnly, point: &DomPointInit) -> DomPoint;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = translate ) ]
    ///The `translate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/translate)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn translate(this: &DomMatrixReadOnly, tx: f64, ty: f64) -> DomMatrix;

    #[cfg(feature = "DomMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMMatrixReadOnly" , js_name = translate ) ]
    ///The `translate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/translate)
    ///
    ///*This API requires the following crate features to be activated: `DomMatrix`, `DomMatrixReadOnly`*
    pub fn translate_with_tz(this: &DomMatrixReadOnly, tx: f64, ty: f64, tz: f64) -> DomMatrix;

}
