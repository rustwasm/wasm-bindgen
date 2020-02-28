use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEOffsetElement , typescript_name = SVGFEOffsetElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeOffsetElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement)\n\n*This API requires the following crate features to be activated: `SvgfeOffsetElement`*"]
    pub type SvgfeOffsetElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = in1 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeOffsetElement`*"]
    pub fn in1(this: &SvgfeOffsetElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_name = dx ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `dx` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/dx)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeOffsetElement`*"]
    pub fn dx(this: &SvgfeOffsetElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = dy ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `dy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/dy)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeOffsetElement`*"]
    pub fn dy(this: &SvgfeOffsetElement) -> SvgAnimatedNumber;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeOffsetElement`*"]
    pub fn x(this: &SvgfeOffsetElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeOffsetElement`*"]
    pub fn y(this: &SvgfeOffsetElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeOffsetElement`*"]
    pub fn width(this: &SvgfeOffsetElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeOffsetElement`*"]
    pub fn height(this: &SvgfeOffsetElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeOffsetElement`*"]
    pub fn result(this: &SvgfeOffsetElement) -> SvgAnimatedString;
}
