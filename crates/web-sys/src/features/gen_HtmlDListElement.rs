use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLDListElement , typescript_name = HTMLDListElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDListElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDListElement)\n\n*This API requires the following crate features to be activated: `HtmlDListElement`*"]
    pub type HtmlDListElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = compact ) ]
    #[doc = "Getter for the `compact` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDListElement/compact)\n\n*This API requires the following crate features to be activated: `HtmlDListElement`*"]
    pub fn compact(this: &HtmlDListElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = compact ) ]
    #[doc = "Setter for the `compact` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDListElement/compact)\n\n*This API requires the following crate features to be activated: `HtmlDListElement`*"]
    pub fn set_compact(this: &HtmlDListElement, value: bool);
}
