use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLUListElement , typescript_name = HTMLUListElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlUListElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement)\n\n*This API requires the following crate features to be activated: `HtmlUListElement`*"]
    pub type HtmlUListElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLUListElement" , js_name = compact ) ]
    #[doc = "Getter for the `compact` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/compact)\n\n*This API requires the following crate features to be activated: `HtmlUListElement`*"]
    pub fn compact(this: &HtmlUListElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLUListElement" , js_name = compact ) ]
    #[doc = "Setter for the `compact` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/compact)\n\n*This API requires the following crate features to be activated: `HtmlUListElement`*"]
    pub fn set_compact(this: &HtmlUListElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLUListElement" , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/type)\n\n*This API requires the following crate features to be activated: `HtmlUListElement`*"]
    pub fn type_(this: &HtmlUListElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLUListElement" , js_name = type ) ]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/type)\n\n*This API requires the following crate features to be activated: `HtmlUListElement`*"]
    pub fn set_type(this: &HtmlUListElement, value: &str);
}
