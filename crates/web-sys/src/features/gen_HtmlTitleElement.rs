use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTitleElement , typescript_name = HTMLTitleElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTitleElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTitleElement)\n\n*This API requires the following crate features to be activated: `HtmlTitleElement`*"]
    pub type HtmlTitleElement;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = text ) ]
    #[doc = "Getter for the `text` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTitleElement/text)\n\n*This API requires the following crate features to be activated: `HtmlTitleElement`*"]
    pub fn text(this: &HtmlTitleElement) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = text ) ]
    #[doc = "Setter for the `text` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTitleElement/text)\n\n*This API requires the following crate features to be activated: `HtmlTitleElement`*"]
    pub fn set_text(this: &HtmlTitleElement, value: &str) -> Result<(), JsValue>;
}
