use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLQuoteElement , typescript_name = HTMLQuoteElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlQuoteElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement)\n\n*This API requires the following crate features to be activated: `HtmlQuoteElement`*"]
    pub type HtmlQuoteElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = cite ) ]
    #[doc = "Getter for the `cite` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement/cite)\n\n*This API requires the following crate features to be activated: `HtmlQuoteElement`*"]
    pub fn cite(this: &HtmlQuoteElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = cite ) ]
    #[doc = "Setter for the `cite` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement/cite)\n\n*This API requires the following crate features to be activated: `HtmlQuoteElement`*"]
    pub fn set_cite(this: &HtmlQuoteElement, value: String);
}
