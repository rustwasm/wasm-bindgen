use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGMatrix , typescript_name = SVGMatrix ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgMatrix` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub type SvgMatrix;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMatrix" , js_name = a ) ]
    ///Getter for the `a` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/a)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn a(this: &SvgMatrix) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGMatrix" , js_name = a ) ]
    ///Setter for the `a` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/a)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn set_a(this: &SvgMatrix, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMatrix" , js_name = b ) ]
    ///Getter for the `b` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/b)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn b(this: &SvgMatrix) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGMatrix" , js_name = b ) ]
    ///Setter for the `b` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/b)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn set_b(this: &SvgMatrix, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMatrix" , js_name = c ) ]
    ///Getter for the `c` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/c)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn c(this: &SvgMatrix) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGMatrix" , js_name = c ) ]
    ///Setter for the `c` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/c)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn set_c(this: &SvgMatrix, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMatrix" , js_name = d ) ]
    ///Getter for the `d` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/d)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn d(this: &SvgMatrix) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGMatrix" , js_name = d ) ]
    ///Setter for the `d` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/d)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn set_d(this: &SvgMatrix, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMatrix" , js_name = e ) ]
    ///Getter for the `e` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/e)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn e(this: &SvgMatrix) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGMatrix" , js_name = e ) ]
    ///Setter for the `e` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/e)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn set_e(this: &SvgMatrix, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGMatrix" , js_name = f ) ]
    ///Getter for the `f` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/f)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn f(this: &SvgMatrix) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGMatrix" , js_name = f ) ]
    ///Setter for the `f` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/f)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn set_f(this: &SvgMatrix, value: f32);

    # [ wasm_bindgen ( method , structural , js_class = "SVGMatrix" , js_name = flipX ) ]
    ///The `flipX()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/flipX)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn flip_x(this: &SvgMatrix) -> SvgMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "SVGMatrix" , js_name = flipY ) ]
    ///The `flipY()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/flipY)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn flip_y(this: &SvgMatrix) -> SvgMatrix;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGMatrix" , js_name = inverse ) ]
    ///The `inverse()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/inverse)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn inverse(this: &SvgMatrix) -> Result<SvgMatrix, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "SVGMatrix" , js_name = multiply ) ]
    ///The `multiply()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/multiply)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn multiply(this: &SvgMatrix, second_matrix: &SvgMatrix) -> SvgMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "SVGMatrix" , js_name = rotate ) ]
    ///The `rotate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/rotate)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn rotate(this: &SvgMatrix, angle: f32) -> SvgMatrix;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGMatrix" , js_name = rotateFromVector ) ]
    ///The `rotateFromVector()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/rotateFromVector)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn rotate_from_vector(this: &SvgMatrix, x: f32, y: f32) -> Result<SvgMatrix, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "SVGMatrix" , js_name = scale ) ]
    ///The `scale()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/scale)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn scale(this: &SvgMatrix, scale_factor: f32) -> SvgMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "SVGMatrix" , js_name = scaleNonUniform ) ]
    ///The `scaleNonUniform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/scaleNonUniform)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn scale_non_uniform(
        this: &SvgMatrix,
        scale_factor_x: f32,
        scale_factor_y: f32,
    ) -> SvgMatrix;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGMatrix" , js_name = skewX ) ]
    ///The `skewX()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/skewX)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn skew_x(this: &SvgMatrix, angle: f32) -> Result<SvgMatrix, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGMatrix" , js_name = skewY ) ]
    ///The `skewY()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/skewY)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn skew_y(this: &SvgMatrix, angle: f32) -> Result<SvgMatrix, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "SVGMatrix" , js_name = translate ) ]
    ///The `translate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/translate)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`*
    pub fn translate(this: &SvgMatrix, x: f32, y: f32) -> SvgMatrix;

}
