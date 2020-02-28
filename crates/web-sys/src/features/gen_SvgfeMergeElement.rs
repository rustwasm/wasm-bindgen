use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGFEMergeElement , typescript_name = SVGFEMergeElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeMergeElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeElement)\n\n*This API requires the following crate features to be activated: `SvgfeMergeElement`*"]
    pub type SvgfeMergeElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMergeElement`*"]
    pub fn x(this: &SvgfeMergeElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMergeElement`*"]
    pub fn y(this: &SvgfeMergeElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMergeElement`*"]
    pub fn width(this: &SvgfeMergeElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMergeElement`*"]
    pub fn height(this: &SvgfeMergeElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = result ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeElement/result)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeMergeElement`*"]
    pub fn result(this: &SvgfeMergeElement) -> SvgAnimatedString;
}
