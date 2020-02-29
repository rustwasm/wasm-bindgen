use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGStopElement , typescript_type = "SVGStopElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgStopElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStopElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgStopElement`*
    pub type SvgStopElement;

    #[cfg(feature = "SvgAnimatedNumber")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGStopElement" , js_name = offset ) ]
    ///Getter for the `offset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStopElement/offset)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgStopElement`*
    pub fn offset(this: &SvgStopElement) -> SvgAnimatedNumber;

}
