use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEFloodElement , typescript_name = SVGFEFloodElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeFloodElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement)\n\n*This API requires the following crate features to be activated: `SvgfeFloodElement`*"]
    pub type SvgfeFloodElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeFloodElement`*"]
    pub fn x(this: &SvgfeFloodElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeFloodElement`*"]
    pub fn y(this: &SvgfeFloodElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeFloodElement`*"]
    pub fn width(this: &SvgfeFloodElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeFloodElement`*"]
    pub fn height(this: &SvgfeFloodElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFloodElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeFloodElement`*"]
    pub fn result(this: &SvgfeFloodElement) -> SvgAnimatedString;
}
