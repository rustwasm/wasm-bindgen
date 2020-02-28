use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLBRElement , typescript_name = HTMLBRElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlBrElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBRElement)\n\n*This API requires the following crate features to be activated: `HtmlBrElement`*"]
    pub type HtmlBrElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLBRElement" , js_name = clear ) ]
    #[doc = "Getter for the `clear` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBRElement/clear)\n\n*This API requires the following crate features to be activated: `HtmlBrElement`*"]
    pub fn clear(this: &HtmlBrElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLBRElement" , js_name = clear ) ]
    #[doc = "Setter for the `clear` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBRElement/clear)\n\n*This API requires the following crate features to be activated: `HtmlBrElement`*"]
    pub fn set_clear(this: &HtmlBrElement, value: &str);
}
