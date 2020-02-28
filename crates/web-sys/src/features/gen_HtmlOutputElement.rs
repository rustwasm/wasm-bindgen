use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLOutputElement , typescript_name = HTMLOutputElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlOutputElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub type HtmlOutputElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = htmlFor ) ]
    #[cfg(feature = "DomTokenList")]
    #[doc = "Getter for the `htmlFor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/htmlFor)\n\n*This API requires the following crate features to be activated: `DomTokenList`, `HtmlOutputElement`*"]
    pub fn html_for(this: &HtmlOutputElement) -> DomTokenList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = form ) ]
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/form)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlOutputElement`*"]
    pub fn form(this: &HtmlOutputElement) -> Option<HtmlFormElement>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/name)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn name(this: &HtmlOutputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOutputElement" , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/name)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn set_name(this: &HtmlOutputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/type)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn type_(this: &HtmlOutputElement) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = defaultValue ) ]
    #[doc = "Getter for the `defaultValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/defaultValue)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn default_value(this: &HtmlOutputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOutputElement" , js_name = defaultValue ) ]
    #[doc = "Setter for the `defaultValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/defaultValue)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn set_default_value(this: &HtmlOutputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/value)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn value(this: &HtmlOutputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOutputElement" , js_name = value ) ]
    #[doc = "Setter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/value)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn set_value(this: &HtmlOutputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = willValidate ) ]
    #[doc = "Getter for the `willValidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/willValidate)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn will_validate(this: &HtmlOutputElement) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = validity ) ]
    #[cfg(feature = "ValidityState")]
    #[doc = "Getter for the `validity` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/validity)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`, `ValidityState`*"]
    pub fn validity(this: &HtmlOutputElement) -> ValidityState;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLOutputElement" , js_name = validationMessage ) ]
    #[doc = "Getter for the `validationMessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/validationMessage)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn validation_message(this: &HtmlOutputElement) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = labels ) ]
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `labels` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/labels)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`, `NodeList`*"]
    pub fn labels(this: &HtmlOutputElement) -> NodeList;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLOutputElement" , js_name = checkValidity ) ]
    #[doc = "The `checkValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/checkValidity)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn check_validity(this: &HtmlOutputElement) -> bool;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLOutputElement" , js_name = reportValidity ) ]
    #[doc = "The `reportValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/reportValidity)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn report_validity(this: &HtmlOutputElement) -> bool;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLOutputElement" , js_name = setCustomValidity ) ]
    #[doc = "The `setCustomValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/setCustomValidity)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    pub fn set_custom_validity(this: &HtmlOutputElement, error: &str);
}
