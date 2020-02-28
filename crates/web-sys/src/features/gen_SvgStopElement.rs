use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGStopElement , typescript_name = SVGStopElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgStopElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStopElement)\n\n*This API requires the following crate features to be activated: `SvgStopElement`*"]
    pub type SvgStopElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = offset ) ]
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `offset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStopElement/offset)\n\n*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgStopElement`*"]
    pub fn offset(this: &SvgStopElement) -> SvgAnimatedNumber;
}
