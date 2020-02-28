use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEDisplacementMapElement , typescript_name = SVGFEDisplacementMapElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeDisplacementMapElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement)\n\n*This API requires the following crate features to be activated: `SvgfeDisplacementMapElement`*"]
    pub type SvgfeDisplacementMapElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = in1 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDisplacementMapElement`*"]
    pub fn in1(this: &SvgfeDisplacementMapElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_name = in2 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/in2)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDisplacementMapElement`*"]
    pub fn in2(this: &SvgfeDisplacementMapElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_name = scale ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `scale` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/scale)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDisplacementMapElement`*"]
    pub fn scale(this: &SvgfeDisplacementMapElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = xChannelSelector ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `xChannelSelector` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/xChannelSelector)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeDisplacementMapElement`*"]
    pub fn x_channel_selector(this: &SvgfeDisplacementMapElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_name = yChannelSelector ) ]
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `yChannelSelector` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/yChannelSelector)\n\n*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeDisplacementMapElement`*"]
    pub fn y_channel_selector(this: &SvgfeDisplacementMapElement) -> SvgAnimatedEnumeration;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDisplacementMapElement`*"]
    pub fn x(this: &SvgfeDisplacementMapElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDisplacementMapElement`*"]
    pub fn y(this: &SvgfeDisplacementMapElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDisplacementMapElement`*"]
    pub fn width(this: &SvgfeDisplacementMapElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDisplacementMapElement`*"]
    pub fn height(this: &SvgfeDisplacementMapElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDisplacementMapElement`*"]
    pub fn result(this: &SvgfeDisplacementMapElement) -> SvgAnimatedString;
}
impl SvgfeDisplacementMapElement {
    pub const SVG_CHANNEL_UNKNOWN: u16 = 0i64 as u16;
    pub const SVG_CHANNEL_R: u16 = 1u64 as u16;
    pub const SVG_CHANNEL_G: u16 = 2u64 as u16;
    pub const SVG_CHANNEL_B: u16 = 3u64 as u16;
    pub const SVG_CHANNEL_A: u16 = 4u64 as u16;
}
