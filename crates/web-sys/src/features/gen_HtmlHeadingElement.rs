use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLHeadingElement , typescript_name = HTMLHeadingElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlHeadingElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement)\n\n*This API requires the following crate features to be activated: `HtmlHeadingElement`*"]
    pub type HtmlHeadingElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = align ) ]
    #[doc = "Getter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement/align)\n\n*This API requires the following crate features to be activated: `HtmlHeadingElement`*"]
    pub fn align(this: &HtmlHeadingElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = align ) ]
    #[doc = "Setter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement/align)\n\n*This API requires the following crate features to be activated: `HtmlHeadingElement`*"]
    pub fn set_align(this: &HtmlHeadingElement, value: &str);
}
