use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGDefsElement , typescript_type = "SVGDefsElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgDefsElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGDefsElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgDefsElement`*
    pub type SvgDefsElement;

}
