use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLButtonElement , typescript_name = HTMLButtonElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlButtonElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub type HtmlButtonElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = autofocus ) ]
    #[doc = "Getter for the `autofocus` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/autofocus)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn autofocus(this: &HtmlButtonElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = autofocus ) ]
    #[doc = "Setter for the `autofocus` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/autofocus)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_autofocus(this: &HtmlButtonElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = disabled ) ]
    #[doc = "Getter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn disabled(this: &HtmlButtonElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = disabled ) ]
    #[doc = "Setter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_disabled(this: &HtmlButtonElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = form ) ]
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/form)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`, `HtmlFormElement`*"]
    pub fn form(this: &HtmlButtonElement) -> Option<HtmlFormElement>;
    # [ wasm_bindgen ( structural , method , getter , js_name = formAction ) ]
    #[doc = "Getter for the `formAction` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formAction)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn form_action(this: &HtmlButtonElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = formAction ) ]
    #[doc = "Setter for the `formAction` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formAction)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_form_action(this: &HtmlButtonElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = formEnctype ) ]
    #[doc = "Getter for the `formEnctype` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formEnctype)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn form_enctype(this: &HtmlButtonElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = formEnctype ) ]
    #[doc = "Setter for the `formEnctype` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formEnctype)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_form_enctype(this: &HtmlButtonElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = formMethod ) ]
    #[doc = "Getter for the `formMethod` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formMethod)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn form_method(this: &HtmlButtonElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = formMethod ) ]
    #[doc = "Setter for the `formMethod` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formMethod)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_form_method(this: &HtmlButtonElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = formNoValidate ) ]
    #[doc = "Getter for the `formNoValidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formNoValidate)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn form_no_validate(this: &HtmlButtonElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = formNoValidate ) ]
    #[doc = "Setter for the `formNoValidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formNoValidate)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_form_no_validate(this: &HtmlButtonElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = formTarget ) ]
    #[doc = "Getter for the `formTarget` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formTarget)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn form_target(this: &HtmlButtonElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = formTarget ) ]
    #[doc = "Setter for the `formTarget` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formTarget)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_form_target(this: &HtmlButtonElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/name)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn name(this: &HtmlButtonElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/name)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_name(this: &HtmlButtonElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/type)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn type_(this: &HtmlButtonElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = type ) ]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/type)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_type(this: &HtmlButtonElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/value)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn value(this: &HtmlButtonElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = value ) ]
    #[doc = "Setter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/value)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_value(this: &HtmlButtonElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = willValidate ) ]
    #[doc = "Getter for the `willValidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/willValidate)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn will_validate(this: &HtmlButtonElement) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = validity ) ]
    #[cfg(feature = "ValidityState")]
    #[doc = "Getter for the `validity` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/validity)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`, `ValidityState`*"]
    pub fn validity(this: &HtmlButtonElement) -> ValidityState;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = validationMessage ) ]
    #[doc = "Getter for the `validationMessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/validationMessage)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn validation_message(this: &HtmlButtonElement) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = labels ) ]
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `labels` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/labels)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`, `NodeList`*"]
    pub fn labels(this: &HtmlButtonElement) -> NodeList;
    # [ wasm_bindgen ( method , structural , js_name = checkValidity ) ]
    #[doc = "The `checkValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/checkValidity)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn check_validity(this: &HtmlButtonElement) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = reportValidity ) ]
    #[doc = "The `reportValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/reportValidity)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn report_validity(this: &HtmlButtonElement) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = setCustomValidity ) ]
    #[doc = "The `setCustomValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/setCustomValidity)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    pub fn set_custom_validity(this: &HtmlButtonElement, error: &str);
}
