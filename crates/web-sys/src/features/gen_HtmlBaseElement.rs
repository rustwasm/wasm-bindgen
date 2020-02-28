use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLBaseElement , typescript_name = HTMLBaseElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlBaseElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement)\n\n*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
    pub type HtmlBaseElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = href ) ]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/href)\n\n*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
    pub fn href(this: &HtmlBaseElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = href ) ]
    #[doc = "Setter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/href)\n\n*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
    pub fn set_href(this: &HtmlBaseElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = target ) ]
    #[doc = "Getter for the `target` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/target)\n\n*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
    pub fn target(this: &HtmlBaseElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = target ) ]
    #[doc = "Setter for the `target` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/target)\n\n*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
    pub fn set_target(this: &HtmlBaseElement, value: String);
}
