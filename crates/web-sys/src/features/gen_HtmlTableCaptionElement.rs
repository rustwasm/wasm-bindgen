use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTableCaptionElement , typescript_name = HTMLTableCaptionElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTableCaptionElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCaptionElement)\n\n*This API requires the following crate features to be activated: `HtmlTableCaptionElement`*"]
    pub type HtmlTableCaptionElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = align ) ]
    #[doc = "Getter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCaptionElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableCaptionElement`*"]
    pub fn align(this: &HtmlTableCaptionElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = align ) ]
    #[doc = "Setter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCaptionElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableCaptionElement`*"]
    pub fn set_align(this: &HtmlTableCaptionElement, value: &str);
}
