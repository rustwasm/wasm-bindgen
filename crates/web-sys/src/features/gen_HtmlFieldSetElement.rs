use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLFieldSetElement , typescript_name = HTMLFieldSetElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlFieldSetElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub type HtmlFieldSetElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = disabled ) ]
    #[doc = "Getter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn disabled(this: &HtmlFieldSetElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = disabled ) ]
    #[doc = "Setter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn set_disabled(this: &HtmlFieldSetElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = form ) ]
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/form)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`, `HtmlFormElement`*"]
    pub fn form(this: &HtmlFieldSetElement) -> Option<HtmlFormElement>;
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/name)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn name(this: &HtmlFieldSetElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/name)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn set_name(this: &HtmlFieldSetElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/type)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn type_(this: &HtmlFieldSetElement) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = elements ) ]
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `elements` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/elements)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlFieldSetElement`*"]
    pub fn elements(this: &HtmlFieldSetElement) -> HtmlCollection;
    # [ wasm_bindgen ( structural , method , getter , js_name = willValidate ) ]
    #[doc = "Getter for the `willValidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/willValidate)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn will_validate(this: &HtmlFieldSetElement) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = validity ) ]
    #[cfg(feature = "ValidityState")]
    #[doc = "Getter for the `validity` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/validity)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`, `ValidityState`*"]
    pub fn validity(this: &HtmlFieldSetElement) -> ValidityState;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = validationMessage ) ]
    #[doc = "Getter for the `validationMessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/validationMessage)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn validation_message(this: &HtmlFieldSetElement) -> Result<String, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = checkValidity ) ]
    #[doc = "The `checkValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/checkValidity)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn check_validity(this: &HtmlFieldSetElement) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = reportValidity ) ]
    #[doc = "The `reportValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/reportValidity)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn report_validity(this: &HtmlFieldSetElement) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = setCustomValidity ) ]
    #[doc = "The `setCustomValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/setCustomValidity)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    pub fn set_custom_validity(this: &HtmlFieldSetElement, error: &str);
}
