use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGGradientElement , typescript_name = SVGGradientElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgGradientElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement)\n\n*This API requires the following crate features to be activated: `SvgGradientElement`*"]
    pub type SvgGradientElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGGradientElement" , js_name = gradientUnits ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `gradientUnits` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/gradientUnits)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgGradientElement`*"]
    pub fn gradient_units(this: &SvgGradientElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGGradientElement" , js_name = gradientTransform ) ]
    #[cfg(feature = "SvgAnimatedTransformList")]
    #[doc = "Getter for the `gradientTransform` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/gradientTransform)\n\n*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgGradientElement`*"]
    pub fn gradient_transform(this: &SvgGradientElement) -> SvgAnimatedTransformList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGGradientElement" , js_name = spreadMethod ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `spreadMethod` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/spreadMethod)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgGradientElement`*"]
    pub fn spread_method(this: &SvgGradientElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGGradientElement" , js_name = href ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/href)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgGradientElement`*"]
    pub fn href(this: &SvgGradientElement) -> SvgAnimatedString;
}
impl SvgGradientElement {
    pub const SVG_SPREADMETHOD_UNKNOWN: u16 = 0i64 as u16;
    pub const SVG_SPREADMETHOD_PAD: u16 = 1u64 as u16;
    pub const SVG_SPREADMETHOD_REFLECT: u16 = 2u64 as u16;
    pub const SVG_SPREADMETHOD_REPEAT: u16 = 3u64 as u16;
}
