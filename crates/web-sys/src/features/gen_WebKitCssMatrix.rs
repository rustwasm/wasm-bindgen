use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = DomMatrix , extends = DomMatrixReadOnly , extends = :: js_sys :: Object , js_name = WebKitCSSMatrix , typescript_name = WebKitCSSMatrix ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebKitCssMatrix` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub type WebKitCssMatrix;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new WebKitCssMatrix(..)` constructor, creating a new instance of `WebKitCssMatrix`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/WebKitCSSMatrix)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn new(this: &WebKitCssMatrix) -> Result<WebKitCssMatrix, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new WebKitCssMatrix(..)` constructor, creating a new instance of `WebKitCssMatrix`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/WebKitCSSMatrix)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn new_with_transform_list(
        this: &WebKitCssMatrix,
        transform_list: &str,
    ) -> Result<WebKitCssMatrix, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new WebKitCssMatrix(..)` constructor, creating a new instance of `WebKitCssMatrix`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/WebKitCSSMatrix)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn new_with_other(
        this: &WebKitCssMatrix,
        other: &WebKitCssMatrix,
    ) -> Result<WebKitCssMatrix, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = inverse ) ]
    #[doc = "The `inverse()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/inverse)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn inverse(this: &WebKitCssMatrix) -> Result<WebKitCssMatrix, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = multiply ) ]
    #[doc = "The `multiply()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/multiply)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn multiply(this: &WebKitCssMatrix, other: &WebKitCssMatrix) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = rotate ) ]
    #[doc = "The `rotate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate(this: &WebKitCssMatrix) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = rotate ) ]
    #[doc = "The `rotate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate_with_rot_x(this: &WebKitCssMatrix, rot_x: f64) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = rotate ) ]
    #[doc = "The `rotate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate_with_rot_x_and_rot_y(
        this: &WebKitCssMatrix,
        rot_x: f64,
        rot_y: f64,
    ) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = rotate ) ]
    #[doc = "The `rotate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate_with_rot_x_and_rot_y_and_rot_z(
        this: &WebKitCssMatrix,
        rot_x: f64,
        rot_y: f64,
        rot_z: f64,
    ) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = rotateAxisAngle ) ]
    #[doc = "The `rotateAxisAngle()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate_axis_angle(this: &WebKitCssMatrix) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = rotateAxisAngle ) ]
    #[doc = "The `rotateAxisAngle()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate_axis_angle_with_x(this: &WebKitCssMatrix, x: f64) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = rotateAxisAngle ) ]
    #[doc = "The `rotateAxisAngle()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate_axis_angle_with_x_and_y(
        this: &WebKitCssMatrix,
        x: f64,
        y: f64,
    ) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = rotateAxisAngle ) ]
    #[doc = "The `rotateAxisAngle()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate_axis_angle_with_x_and_y_and_z(
        this: &WebKitCssMatrix,
        x: f64,
        y: f64,
        z: f64,
    ) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = rotateAxisAngle ) ]
    #[doc = "The `rotateAxisAngle()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn rotate_axis_angle_with_x_and_y_and_z_and_angle(
        this: &WebKitCssMatrix,
        x: f64,
        y: f64,
        z: f64,
        angle: f64,
    ) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = scale ) ]
    #[doc = "The `scale()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn scale(this: &WebKitCssMatrix) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = scale ) ]
    #[doc = "The `scale()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn scale_with_scale_x(this: &WebKitCssMatrix, scale_x: f64) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = scale ) ]
    #[doc = "The `scale()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn scale_with_scale_x_and_scale_y(
        this: &WebKitCssMatrix,
        scale_x: f64,
        scale_y: f64,
    ) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = scale ) ]
    #[doc = "The `scale()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn scale_with_scale_x_and_scale_y_and_scale_z(
        this: &WebKitCssMatrix,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
    ) -> WebKitCssMatrix;
    # [ wasm_bindgen ( catch , method , structural , js_name = setMatrixValue ) ]
    #[doc = "The `setMatrixValue()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/setMatrixValue)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn set_matrix_value(
        this: &WebKitCssMatrix,
        transform_list: &str,
    ) -> Result<WebKitCssMatrix, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = skewX ) ]
    #[doc = "The `skewX()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewX)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn skew_x(this: &WebKitCssMatrix) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = skewX ) ]
    #[doc = "The `skewX()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewX)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn skew_x_with_sx(this: &WebKitCssMatrix, sx: f64) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = skewY ) ]
    #[doc = "The `skewY()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewY)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn skew_y(this: &WebKitCssMatrix) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = skewY ) ]
    #[doc = "The `skewY()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewY)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn skew_y_with_sy(this: &WebKitCssMatrix, sy: f64) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = translate ) ]
    #[doc = "The `translate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn translate(this: &WebKitCssMatrix) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = translate ) ]
    #[doc = "The `translate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn translate_with_tx(this: &WebKitCssMatrix, tx: f64) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = translate ) ]
    #[doc = "The `translate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn translate_with_tx_and_ty(this: &WebKitCssMatrix, tx: f64, ty: f64) -> WebKitCssMatrix;
    # [ wasm_bindgen ( method , structural , js_name = translate ) ]
    #[doc = "The `translate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)\n\n*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
    pub fn translate_with_tx_and_ty_and_tz(
        this: &WebKitCssMatrix,
        tx: f64,
        ty: f64,
        tz: f64,
    ) -> WebKitCssMatrix;
}
