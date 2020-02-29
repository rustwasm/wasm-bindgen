use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLPictureElement , typescript_type = "HTMLPictureElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlPictureElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPictureElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlPictureElement`*
    pub type HtmlPictureElement;

}
