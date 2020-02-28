use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGMPathElement , typescript_name = SVGMPathElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgmPathElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMPathElement)\n\n*This API requires the following crate features to be activated: `SvgmPathElement`*"]
    pub type SvgmPathElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = href ) ]
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMPathElement/href)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgmPathElement`*"]
    pub fn href(this: &SvgmPathElement) -> SvgAnimatedString;
}
