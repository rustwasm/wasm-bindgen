use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEColorMatrixElement , typescript_name = SVGFEColorMatrixElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeColorMatrixElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement)\n\n*This API requires the following crate features to be activated: `SvgfeColorMatrixElement`*"]
    pub type SvgfeColorMatrixElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = in1 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeColorMatrixElement`*"]
    pub fn in1(this: &SvgfeColorMatrixElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/type)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeColorMatrixElement`*"]
    pub fn type_(this: &SvgfeColorMatrixElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_name = values ) ]
    #[cfg(feature = "SvgAnimatedNumberList")]
    #[doc = "Getter for the `values` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/values)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgfeColorMatrixElement`*"]
    pub fn values(this: &SvgfeColorMatrixElement) -> SvgAnimatedNumberList;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeColorMatrixElement`*"]
    pub fn x(this: &SvgfeColorMatrixElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeColorMatrixElement`*"]
    pub fn y(this: &SvgfeColorMatrixElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeColorMatrixElement`*"]
    pub fn width(this: &SvgfeColorMatrixElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeColorMatrixElement`*"]
    pub fn height(this: &SvgfeColorMatrixElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeColorMatrixElement`*"]
    pub fn result(this: &SvgfeColorMatrixElement) -> SvgAnimatedString;
}
impl SvgfeColorMatrixElement {
    pub const SVG_FECOLORMATRIX_TYPE_UNKNOWN: u16 = 0i64 as u16;
    pub const SVG_FECOLORMATRIX_TYPE_MATRIX: u16 = 1u64 as u16;
    pub const SVG_FECOLORMATRIX_TYPE_SATURATE: u16 = 2u64 as u16;
    pub const SVG_FECOLORMATRIX_TYPE_HUEROTATE: u16 = 3u64 as u16;
    pub const SVG_FECOLORMATRIX_TYPE_LUMINANCETOALPHA: u16 = 4u64 as u16;
}
