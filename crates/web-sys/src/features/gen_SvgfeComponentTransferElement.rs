use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEComponentTransferElement , typescript_name = SVGFEComponentTransferElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeComponentTransferElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement)\n\n*This API requires the following crate features to be activated: `SvgfeComponentTransferElement`*"]
    pub type SvgfeComponentTransferElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = in1 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeComponentTransferElement`*"]
    pub fn in1(this: &SvgfeComponentTransferElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeComponentTransferElement`*"]
    pub fn x(this: &SvgfeComponentTransferElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeComponentTransferElement`*"]
    pub fn y(this: &SvgfeComponentTransferElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeComponentTransferElement`*"]
    pub fn width(this: &SvgfeComponentTransferElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeComponentTransferElement`*"]
    pub fn height(this: &SvgfeComponentTransferElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeComponentTransferElement`*"]
    pub fn result(this: &SvgfeComponentTransferElement) -> SvgAnimatedString;
}
