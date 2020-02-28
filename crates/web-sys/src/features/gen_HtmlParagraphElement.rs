use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLParagraphElement , typescript_name = HTMLParagraphElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlParagraphElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement)\n\n*This API requires the following crate features to be activated: `HtmlParagraphElement`*"]
    pub type HtmlParagraphElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = align ) ]
    #[doc = "Getter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement/align)\n\n*This API requires the following crate features to be activated: `HtmlParagraphElement`*"]
    pub fn align(this: &HtmlParagraphElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = align ) ]
    #[doc = "Setter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement/align)\n\n*This API requires the following crate features to be activated: `HtmlParagraphElement`*"]
    pub fn set_align(this: &HtmlParagraphElement, value: String);
}
