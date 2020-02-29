use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLSpanElement , typescript_type = "HTMLSpanElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlSpanElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSpanElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSpanElement`*
    pub type HtmlSpanElement;

}
