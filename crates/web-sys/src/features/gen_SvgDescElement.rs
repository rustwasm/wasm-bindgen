use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGDescElement , typescript_name = SVGDescElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgDescElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGDescElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgDescElement`*
    pub type SvgDescElement;

}
