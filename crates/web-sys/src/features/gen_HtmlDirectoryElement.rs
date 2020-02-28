use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLDirectoryElement , typescript_name = HTMLDirectoryElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDirectoryElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDirectoryElement)\n\n*This API requires the following crate features to be activated: `HtmlDirectoryElement`*"]
    pub type HtmlDirectoryElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = compact ) ]
    #[doc = "Getter for the `compact` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDirectoryElement/compact)\n\n*This API requires the following crate features to be activated: `HtmlDirectoryElement`*"]
    pub fn compact(this: &HtmlDirectoryElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = compact ) ]
    #[doc = "Setter for the `compact` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDirectoryElement/compact)\n\n*This API requires the following crate features to be activated: `HtmlDirectoryElement`*"]
    pub fn set_compact(this: &HtmlDirectoryElement, value: bool);
}
