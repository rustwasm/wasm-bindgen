use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLDataElement , typescript_name = HTMLDataElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDataElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataElement)\n\n*This API requires the following crate features to be activated: `HtmlDataElement`*"]
    pub type HtmlDataElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLDataElement" , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataElement/value)\n\n*This API requires the following crate features to be activated: `HtmlDataElement`*"]
    pub fn value(this: &HtmlDataElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLDataElement" , js_name = value ) ]
    #[doc = "Setter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataElement/value)\n\n*This API requires the following crate features to be activated: `HtmlDataElement`*"]
    pub fn set_value(this: &HtmlDataElement, value: &str);
}
