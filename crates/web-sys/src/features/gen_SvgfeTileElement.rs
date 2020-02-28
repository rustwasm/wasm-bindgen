use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFETileElement , typescript_name = SVGFETileElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeTileElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement)\n\n*This API requires the following crate features to be activated: `SvgfeTileElement`*"]
    pub type SvgfeTileElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETileElement" , js_name = in1 ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/in1)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeTileElement`*"]
    pub fn in1(this: &SvgfeTileElement) -> SvgAnimatedString;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETileElement" , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTileElement`*"]
    pub fn x(this: &SvgfeTileElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETileElement" , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTileElement`*"]
    pub fn y(this: &SvgfeTileElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETileElement" , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTileElement`*"]
    pub fn width(this: &SvgfeTileElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETileElement" , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTileElement`*"]
    pub fn height(this: &SvgfeTileElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGFETileElement" , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETileElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeTileElement`*"]
    pub fn result(this: &SvgfeTileElement) -> SvgAnimatedString;
}
