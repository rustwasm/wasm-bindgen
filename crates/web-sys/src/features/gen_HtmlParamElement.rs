use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLParamElement , typescript_name = HTMLParamElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlParamElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement)\n\n*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub type HtmlParamElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/name)\n\n*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn name(this: &HtmlParamElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/name)\n\n*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn set_name(this: &HtmlParamElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/value)\n\n*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn value(this: &HtmlParamElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = value ) ]
    #[doc = "Setter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/value)\n\n*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn set_value(this: &HtmlParamElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/type)\n\n*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn type_(this: &HtmlParamElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = type ) ]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/type)\n\n*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn set_type(this: &HtmlParamElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = valueType ) ]
    #[doc = "Getter for the `valueType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/valueType)\n\n*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn value_type(this: &HtmlParamElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = valueType ) ]
    #[doc = "Setter for the `valueType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/valueType)\n\n*This API requires the following crate features to be activated: `HtmlParamElement`*"]
    pub fn set_value_type(this: &HtmlParamElement, value: &str);
}
