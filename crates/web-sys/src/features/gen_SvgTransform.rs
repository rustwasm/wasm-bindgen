use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SVGTransform , typescript_name = SVGTransform ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgTransform` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform)\n\n*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub type SvgTransform;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/type)\n\n*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub fn type_(this: &SvgTransform) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_name = matrix ) ]
    #[cfg(feature = "SvgMatrix")]
    #[doc = "Getter for the `matrix` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/matrix)\n\n*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`*"]
    pub fn matrix(this: &SvgTransform) -> SvgMatrix;
    # [ wasm_bindgen ( structural , method , getter , js_name = angle ) ]
    #[doc = "Getter for the `angle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/angle)\n\n*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub fn angle(this: &SvgTransform) -> f32;
    #[cfg(feature = "SvgMatrix")]
    # [ wasm_bindgen ( catch , method , structural , js_name = setMatrix ) ]
    #[doc = "The `setMatrix()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setMatrix)\n\n*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`*"]
    pub fn set_matrix(this: &SvgTransform, matrix: &SvgMatrix) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setRotate ) ]
    #[doc = "The `setRotate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setRotate)\n\n*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub fn set_rotate(this: &SvgTransform, angle: f32, cx: f32, cy: f32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setScale ) ]
    #[doc = "The `setScale()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setScale)\n\n*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub fn set_scale(this: &SvgTransform, sx: f32, sy: f32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setSkewX ) ]
    #[doc = "The `setSkewX()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setSkewX)\n\n*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub fn set_skew_x(this: &SvgTransform, angle: f32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setSkewY ) ]
    #[doc = "The `setSkewY()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setSkewY)\n\n*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub fn set_skew_y(this: &SvgTransform, angle: f32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setTranslate ) ]
    #[doc = "The `setTranslate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setTranslate)\n\n*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub fn set_translate(this: &SvgTransform, tx: f32, ty: f32) -> Result<(), JsValue>;
}
impl SvgTransform {
    pub const SVG_TRANSFORM_UNKNOWN: u16 = 0i64 as u16;
    pub const SVG_TRANSFORM_MATRIX: u16 = 1u64 as u16;
    pub const SVG_TRANSFORM_TRANSLATE: u16 = 2u64 as u16;
    pub const SVG_TRANSFORM_SCALE: u16 = 3u64 as u16;
    pub const SVG_TRANSFORM_ROTATE: u16 = 4u64 as u16;
    pub const SVG_TRANSFORM_SKEWX: u16 = 5u64 as u16;
    pub const SVG_TRANSFORM_SKEWY: u16 = 6u64 as u16;
}
