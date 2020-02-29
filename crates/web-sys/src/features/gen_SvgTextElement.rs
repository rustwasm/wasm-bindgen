use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgTextPositioningElement , extends = SvgTextContentElement , extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGTextElement , typescript_type = "SVGTextElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgTextElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgTextElement`*
    pub type SvgTextElement;

}
