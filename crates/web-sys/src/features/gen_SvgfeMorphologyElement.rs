use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEMorphologyElement , typescript_name = SVGFEMorphologyElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeMorphologyElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement)\n\n*This API requires the following crate features to be activated: `SvgfeMorphologyElement`*"]
    pub type SvgfeMorphologyElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = in1 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeMorphologyElement`*"]
    pub fn in1(this: &SvgfeMorphologyElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_name = operator ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `operator` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/operator)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeMorphologyElement`*"]
    pub fn operator(this: &SvgfeMorphologyElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_name = radiusX ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `radiusX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/radiusX)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeMorphologyElement`*"]
    pub fn radius_x(this: &SvgfeMorphologyElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = radiusY ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `radiusY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/radiusY)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeMorphologyElement`*"]
    pub fn radius_y(this: &SvgfeMorphologyElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMorphologyElement`*"]
    pub fn x(this: &SvgfeMorphologyElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMorphologyElement`*"]
    pub fn y(this: &SvgfeMorphologyElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMorphologyElement`*"]
    pub fn width(this: &SvgfeMorphologyElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMorphologyElement`*"]
    pub fn height(this: &SvgfeMorphologyElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeMorphologyElement`*"]
    pub fn result(this: &SvgfeMorphologyElement) -> SvgAnimatedString;
}
impl SvgfeMorphologyElement {
    pub const SVG_MORPHOLOGY_OPERATOR_UNKNOWN: u16 = 0i64 as u16;
    pub const SVG_MORPHOLOGY_OPERATOR_ERODE: u16 = 1u64 as u16;
    pub const SVG_MORPHOLOGY_OPERATOR_DILATE: u16 = 2u64 as u16;
}
