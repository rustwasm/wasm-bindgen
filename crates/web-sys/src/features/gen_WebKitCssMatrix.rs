use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = DomMatrix , extends = DomMatrixReadOnly , extends = :: js_sys :: Object , js_name = WebKitCSSMatrix , typescript_name = WebKitCSSMatrix ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebKitCssMatrix` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub type WebKitCssMatrix;

    #[wasm_bindgen(catch, constructor, js_class = "WebKitCSSMatrix")]
    ///The `new WebKitCssMatrix(..)` constructor, creating a new instance of `WebKitCssMatrix`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/WebKitCSSMatrix)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn new() -> Result<WebKitCssMatrix, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "WebKitCSSMatrix")]
    ///The `new WebKitCssMatrix(..)` constructor, creating a new instance of `WebKitCssMatrix`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/WebKitCSSMatrix)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn new_with_transform_list(transform_list: &str) -> Result<WebKitCssMatrix, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "WebKitCSSMatrix")]
    ///The `new WebKitCssMatrix(..)` constructor, creating a new instance of `WebKitCssMatrix`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/WebKitCSSMatrix)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn new_with_other(other: &WebKitCssMatrix) -> Result<WebKitCssMatrix, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebKitCSSMatrix" , js_name = inverse ) ]
    ///The `inverse()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/inverse)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn inverse(this: &WebKitCssMatrix) -> Result<WebKitCssMatrix, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = multiply ) ]
    ///The `multiply()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/multiply)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn multiply(this: &WebKitCssMatrix, other: &WebKitCssMatrix) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = rotate ) ]
    ///The `rotate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn rotate(this: &WebKitCssMatrix) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = rotate ) ]
    ///The `rotate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn rotate_with_rot_x(this: &WebKitCssMatrix, rot_x: f64) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = rotate ) ]
    ///The `rotate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn rotate_with_rot_x_and_rot_y(
        this: &WebKitCssMatrix,
        rot_x: f64,
        rot_y: f64,
    ) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = rotate ) ]
    ///The `rotate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn rotate_with_rot_x_and_rot_y_and_rot_z(
        this: &WebKitCssMatrix,
        rot_x: f64,
        rot_y: f64,
        rot_z: f64,
    ) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = rotateAxisAngle ) ]
    ///The `rotateAxisAngle()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn rotate_axis_angle(this: &WebKitCssMatrix) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = rotateAxisAngle ) ]
    ///The `rotateAxisAngle()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn rotate_axis_angle_with_x(this: &WebKitCssMatrix, x: f64) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = rotateAxisAngle ) ]
    ///The `rotateAxisAngle()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn rotate_axis_angle_with_x_and_y(
        this: &WebKitCssMatrix,
        x: f64,
        y: f64,
    ) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = rotateAxisAngle ) ]
    ///The `rotateAxisAngle()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn rotate_axis_angle_with_x_and_y_and_z(
        this: &WebKitCssMatrix,
        x: f64,
        y: f64,
        z: f64,
    ) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = rotateAxisAngle ) ]
    ///The `rotateAxisAngle()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn rotate_axis_angle_with_x_and_y_and_z_and_angle(
        this: &WebKitCssMatrix,
        x: f64,
        y: f64,
        z: f64,
        angle: f64,
    ) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = scale ) ]
    ///The `scale()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn scale(this: &WebKitCssMatrix) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = scale ) ]
    ///The `scale()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn scale_with_scale_x(this: &WebKitCssMatrix, scale_x: f64) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = scale ) ]
    ///The `scale()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn scale_with_scale_x_and_scale_y(
        this: &WebKitCssMatrix,
        scale_x: f64,
        scale_y: f64,
    ) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = scale ) ]
    ///The `scale()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn scale_with_scale_x_and_scale_y_and_scale_z(
        this: &WebKitCssMatrix,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
    ) -> WebKitCssMatrix;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WebKitCSSMatrix" , js_name = setMatrixValue ) ]
    ///The `setMatrixValue()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/setMatrixValue)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn set_matrix_value(
        this: &WebKitCssMatrix,
        transform_list: &str,
    ) -> Result<WebKitCssMatrix, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = skewX ) ]
    ///The `skewX()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewX)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn skew_x(this: &WebKitCssMatrix) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = skewX ) ]
    ///The `skewX()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewX)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn skew_x_with_sx(this: &WebKitCssMatrix, sx: f64) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = skewY ) ]
    ///The `skewY()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewY)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn skew_y(this: &WebKitCssMatrix) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = skewY ) ]
    ///The `skewY()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewY)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn skew_y_with_sy(this: &WebKitCssMatrix, sy: f64) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = translate ) ]
    ///The `translate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn translate(this: &WebKitCssMatrix) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = translate ) ]
    ///The `translate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn translate_with_tx(this: &WebKitCssMatrix, tx: f64) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = translate ) ]
    ///The `translate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn translate_with_tx_and_ty(this: &WebKitCssMatrix, tx: f64, ty: f64) -> WebKitCssMatrix;

    # [ wasm_bindgen ( method , structural , js_class = "WebKitCSSMatrix" , js_name = translate ) ]
    ///The `translate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)
    ///
    ///*This API requires the following crate features to be activated: `WebKitCssMatrix`*
    pub fn translate_with_tx_and_ty_and_tz(
        this: &WebKitCssMatrix,
        tx: f64,
        ty: f64,
        tz: f64,
    ) -> WebKitCssMatrix;

}
