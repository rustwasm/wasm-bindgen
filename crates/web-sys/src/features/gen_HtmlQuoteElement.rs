use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLQuoteElement , typescript_type = "HTMLQuoteElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlQuoteElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlQuoteElement`*
    pub type HtmlQuoteElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLQuoteElement" , js_name = cite ) ]
    ///Getter for the `cite` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement/cite)
    ///
    ///*This API requires the following crate features to be activated: `HtmlQuoteElement`*
    pub fn cite(this: &HtmlQuoteElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLQuoteElement" , js_name = cite ) ]
    ///Setter for the `cite` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement/cite)
    ///
    ///*This API requires the following crate features to be activated: `HtmlQuoteElement`*
    pub fn set_cite(this: &HtmlQuoteElement, value: &str);

}
