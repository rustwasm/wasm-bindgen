use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGMetadataElement , typescript_type = "SVGMetadataElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgMetadataElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMetadataElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgMetadataElement`*
    pub type SvgMetadataElement;

}
