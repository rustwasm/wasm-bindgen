use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLUnknownElement , typescript_name = HTMLUnknownElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlUnknownElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUnknownElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlUnknownElement`*
    pub type HtmlUnknownElement;

}
