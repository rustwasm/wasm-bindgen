use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLHeadElement , typescript_name = HTMLHeadElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlHeadElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlHeadElement`*
    pub type HtmlHeadElement;

}
