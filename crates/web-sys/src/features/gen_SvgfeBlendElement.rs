use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEBlendElement , typescript_name = SVGFEBlendElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeBlendElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement)\n\n*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub type SvgfeBlendElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEBlendElement" , js_name = in1 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeBlendElement`*"]
    pub fn in1(this: &SvgfeBlendElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEBlendElement" , js_name = in2 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/in2)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeBlendElement`*"]
    pub fn in2(this: &SvgfeBlendElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEBlendElement" , js_name = mode ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `mode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/mode)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeBlendElement`*"]
    pub fn mode(this: &SvgfeBlendElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEBlendElement" , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeBlendElement`*"]
    pub fn x(this: &SvgfeBlendElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEBlendElement" , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeBlendElement`*"]
    pub fn y(this: &SvgfeBlendElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEBlendElement" , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeBlendElement`*"]
    pub fn width(this: &SvgfeBlendElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEBlendElement" , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeBlendElement`*"]
    pub fn height(this: &SvgfeBlendElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFEBlendElement" , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeBlendElement`*"]
    pub fn result(this: &SvgfeBlendElement) -> SvgAnimatedString;
}
impl SvgfeBlendElement {
    pub const SVG_FEBLEND_MODE_UNKNOWN: u16 = 0i64 as u16;
    pub const SVG_FEBLEND_MODE_NORMAL: u16 = 1u64 as u16;
    pub const SVG_FEBLEND_MODE_MULTIPLY: u16 = 2u64 as u16;
    pub const SVG_FEBLEND_MODE_SCREEN: u16 = 3u64 as u16;
    pub const SVG_FEBLEND_MODE_DARKEN: u16 = 4u64 as u16;
    pub const SVG_FEBLEND_MODE_LIGHTEN: u16 = 5u64 as u16;
    pub const SVG_FEBLEND_MODE_OVERLAY: u16 = 6u64 as u16;
    pub const SVG_FEBLEND_MODE_COLOR_DODGE: u16 = 7u64 as u16;
    pub const SVG_FEBLEND_MODE_COLOR_BURN: u16 = 8u64 as u16;
    pub const SVG_FEBLEND_MODE_HARD_LIGHT: u16 = 9u64 as u16;
    pub const SVG_FEBLEND_MODE_SOFT_LIGHT: u16 = 10u64 as u16;
    pub const SVG_FEBLEND_MODE_DIFFERENCE: u16 = 11u64 as u16;
    pub const SVG_FEBLEND_MODE_EXCLUSION: u16 = 12u64 as u16;
    pub const SVG_FEBLEND_MODE_HUE: u16 = 13u64 as u16;
    pub const SVG_FEBLEND_MODE_SATURATION: u16 = 14u64 as u16;
    pub const SVG_FEBLEND_MODE_COLOR: u16 = 15u64 as u16;
    pub const SVG_FEBLEND_MODE_LUMINOSITY: u16 = 16u64 as u16;
}
