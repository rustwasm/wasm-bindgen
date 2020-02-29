use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGTransform , typescript_type = "SVGTransform" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgTransform` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`*
    pub type SvgTransform;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTransform" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/type)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`*
    pub fn type_(this: &SvgTransform) -> u16;

    #[cfg(feature = "SvgMatrix")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTransform" , js_name = matrix ) ]
    ///Getter for the `matrix` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/matrix)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`*
    pub fn matrix(this: &SvgTransform) -> SvgMatrix;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGTransform" , js_name = angle ) ]
    ///Getter for the `angle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/angle)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`*
    pub fn angle(this: &SvgTransform) -> f32;

    #[cfg(feature = "SvgMatrix")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTransform" , js_name = setMatrix ) ]
    ///The `setMatrix()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setMatrix)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`*
    pub fn set_matrix(this: &SvgTransform, matrix: &SvgMatrix) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTransform" , js_name = setRotate ) ]
    ///The `setRotate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setRotate)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`*
    pub fn set_rotate(this: &SvgTransform, angle: f32, cx: f32, cy: f32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTransform" , js_name = setScale ) ]
    ///The `setScale()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setScale)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`*
    pub fn set_scale(this: &SvgTransform, sx: f32, sy: f32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTransform" , js_name = setSkewX ) ]
    ///The `setSkewX()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setSkewX)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`*
    pub fn set_skew_x(this: &SvgTransform, angle: f32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTransform" , js_name = setSkewY ) ]
    ///The `setSkewY()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setSkewY)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`*
    pub fn set_skew_y(this: &SvgTransform, angle: f32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SVGTransform" , js_name = setTranslate ) ]
    ///The `setTranslate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setTranslate)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`*
    pub fn set_translate(this: &SvgTransform, tx: f32, ty: f32) -> Result<(), JsValue>;

}

impl SvgTransform {
    ///The `SVGTransform.SVG_TRANSFORM_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`*

    pub const SVG_TRANSFORM_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGTransform.SVG_TRANSFORM_MATRIX` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`*

    pub const SVG_TRANSFORM_MATRIX: u16 = 1u64 as u16;

    ///The `SVGTransform.SVG_TRANSFORM_TRANSLATE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`*

    pub const SVG_TRANSFORM_TRANSLATE: u16 = 2u64 as u16;

    ///The `SVGTransform.SVG_TRANSFORM_SCALE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`*

    pub const SVG_TRANSFORM_SCALE: u16 = 3u64 as u16;

    ///The `SVGTransform.SVG_TRANSFORM_ROTATE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`*

    pub const SVG_TRANSFORM_ROTATE: u16 = 4u64 as u16;

    ///The `SVGTransform.SVG_TRANSFORM_SKEWX` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`*

    pub const SVG_TRANSFORM_SKEWX: u16 = 5u64 as u16;

    ///The `SVGTransform.SVG_TRANSFORM_SKEWY` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`*

    pub const SVG_TRANSFORM_SKEWY: u16 = 6u64 as u16;
}
