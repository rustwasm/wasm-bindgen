use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLLIElement , typescript_name = HTMLLIElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlLiElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement)\n\n*This API requires the following crate features to be activated: `HtmlLiElement`*"]
    pub type HtmlLiElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/value)\n\n*This API requires the following crate features to be activated: `HtmlLiElement`*"]
    pub fn value(this: &HtmlLiElement) -> i32;
    # [ wasm_bindgen ( structural , method , setter , js_name = value ) ]
    #[doc = "Setter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/value)\n\n*This API requires the following crate features to be activated: `HtmlLiElement`*"]
    pub fn set_value(this: &HtmlLiElement, value: i32);
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/type)\n\n*This API requires the following crate features to be activated: `HtmlLiElement`*"]
    pub fn type_(this: &HtmlLiElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = type ) ]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/type)\n\n*This API requires the following crate features to be activated: `HtmlLiElement`*"]
    pub fn set_type(this: &HtmlLiElement, value: &str);
}
