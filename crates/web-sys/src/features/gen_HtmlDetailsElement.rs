use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLDetailsElement , typescript_name = HTMLDetailsElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDetailsElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDetailsElement)\n\n*This API requires the following crate features to be activated: `HtmlDetailsElement`*"]
    pub type HtmlDetailsElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = open ) ]
    #[doc = "Getter for the `open` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDetailsElement/open)\n\n*This API requires the following crate features to be activated: `HtmlDetailsElement`*"]
    pub fn open(this: &HtmlDetailsElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = open ) ]
    #[doc = "Setter for the `open` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDetailsElement/open)\n\n*This API requires the following crate features to be activated: `HtmlDetailsElement`*"]
    pub fn set_open(this: &HtmlDetailsElement, value: bool);
}
