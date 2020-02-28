use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGMatrix , typescript_name = SVGMatrix ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgMatrix` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub type SvgMatrix;
    # [ wasm_bindgen ( structural , method , getter , js_name = a ) ]
    #[doc = "Getter for the `a` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/a)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn a(this: &SvgMatrix) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = a ) ]
    #[doc = "Setter for the `a` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/a)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn set_a(this: &SvgMatrix, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = b ) ]
    #[doc = "Getter for the `b` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/b)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn b(this: &SvgMatrix) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = b ) ]
    #[doc = "Setter for the `b` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/b)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn set_b(this: &SvgMatrix, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = c ) ]
    #[doc = "Getter for the `c` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/c)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn c(this: &SvgMatrix) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = c ) ]
    #[doc = "Setter for the `c` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/c)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn set_c(this: &SvgMatrix, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = d ) ]
    #[doc = "Getter for the `d` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/d)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn d(this: &SvgMatrix) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = d ) ]
    #[doc = "Setter for the `d` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/d)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn set_d(this: &SvgMatrix, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = e ) ]
    #[doc = "Getter for the `e` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/e)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn e(this: &SvgMatrix) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = e ) ]
    #[doc = "Setter for the `e` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/e)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn set_e(this: &SvgMatrix, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = f ) ]
    #[doc = "Getter for the `f` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/f)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn f(this: &SvgMatrix) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = f ) ]
    #[doc = "Setter for the `f` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/f)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn set_f(this: &SvgMatrix, value: f32);
    # [ wasm_bindgen ( method , structural , js_name = flipX ) ]
    #[doc = "The `flipX()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/flipX)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn flip_x(this: &SvgMatrix) -> SvgMatrix;
    # [ wasm_bindgen ( method , structural , js_name = flipY ) ]
    #[doc = "The `flipY()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/flipY)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn flip_y(this: &SvgMatrix) -> SvgMatrix;
    # [ wasm_bindgen ( catch , method , structural , js_name = inverse ) ]
    #[doc = "The `inverse()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/inverse)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn inverse(this: &SvgMatrix) -> Result<SvgMatrix, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = multiply ) ]
    #[doc = "The `multiply()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/multiply)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn multiply(this: &SvgMatrix, second_matrix: &SvgMatrix) -> SvgMatrix;
    # [ wasm_bindgen ( method , structural , js_name = rotate ) ]
    #[doc = "The `rotate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/rotate)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn rotate(this: &SvgMatrix, angle: f32) -> SvgMatrix;
    # [ wasm_bindgen ( catch , method , structural , js_name = rotateFromVector ) ]
    #[doc = "The `rotateFromVector()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/rotateFromVector)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn rotate_from_vector(this: &SvgMatrix, x: f32, y: f32) -> Result<SvgMatrix, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = scale ) ]
    #[doc = "The `scale()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/scale)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn scale(this: &SvgMatrix, scale_factor: f32) -> SvgMatrix;
    # [ wasm_bindgen ( method , structural , js_name = scaleNonUniform ) ]
    #[doc = "The `scaleNonUniform()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/scaleNonUniform)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn scale_non_uniform(
        this: &SvgMatrix,
        scale_factor_x: f32,
        scale_factor_y: f32,
    ) -> SvgMatrix;
    # [ wasm_bindgen ( catch , method , structural , js_name = skewX ) ]
    #[doc = "The `skewX()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/skewX)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn skew_x(this: &SvgMatrix, angle: f32) -> Result<SvgMatrix, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = skewY ) ]
    #[doc = "The `skewY()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/skewY)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn skew_y(this: &SvgMatrix, angle: f32) -> Result<SvgMatrix, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = translate ) ]
    #[doc = "The `translate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMatrix/translate)\n\n*This API requires the following crate features to be activated: `SvgMatrix`*"]
    pub fn translate(this: &SvgMatrix, x: f32, y: f32) -> SvgMatrix;
}
