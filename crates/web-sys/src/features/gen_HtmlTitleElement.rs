use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTitleElement , typescript_type = "HTMLTitleElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlTitleElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTitleElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTitleElement`*
    pub type HtmlTitleElement;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLTitleElement" , js_name = text ) ]
    ///Getter for the `text` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTitleElement/text)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTitleElement`*
    pub fn text(this: &HtmlTitleElement) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "HTMLTitleElement" , js_name = text ) ]
    ///Setter for the `text` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTitleElement/text)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTitleElement`*
    pub fn set_text(this: &HtmlTitleElement, value: &str) -> Result<(), JsValue>;

}
