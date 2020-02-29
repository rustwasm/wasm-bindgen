use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLButtonElement , typescript_name = HTMLButtonElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlButtonElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub type HtmlButtonElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLButtonElement" , js_name = autofocus ) ]
    ///Getter for the `autofocus` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/autofocus)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn autofocus(this: &HtmlButtonElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLButtonElement" , js_name = autofocus ) ]
    ///Setter for the `autofocus` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/autofocus)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn set_autofocus(this: &HtmlButtonElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLButtonElement" , js_name = disabled ) ]
    ///Getter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn disabled(this: &HtmlButtonElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLButtonElement" , js_name = disabled ) ]
    ///Setter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn set_disabled(this: &HtmlButtonElement, value: bool);

    #[cfg(feature = "HtmlFormElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLButtonElement" , js_name = form ) ]
    ///Getter for the `form` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/form)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`, `HtmlFormElement`*
    pub fn form(this: &HtmlButtonElement) -> Option<HtmlFormElement>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLButtonElement" , js_name = formAction ) ]
    ///Getter for the `formAction` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formAction)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn form_action(this: &HtmlButtonElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLButtonElement" , js_name = formAction ) ]
    ///Setter for the `formAction` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formAction)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn set_form_action(this: &HtmlButtonElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLButtonElement" , js_name = formEnctype ) ]
    ///Getter for the `formEnctype` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formEnctype)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn form_enctype(this: &HtmlButtonElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLButtonElement" , js_name = formEnctype ) ]
    ///Setter for the `formEnctype` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formEnctype)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn set_form_enctype(this: &HtmlButtonElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLButtonElement" , js_name = formMethod ) ]
    ///Getter for the `formMethod` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formMethod)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn form_method(this: &HtmlButtonElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLButtonElement" , js_name = formMethod ) ]
    ///Setter for the `formMethod` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formMethod)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn set_form_method(this: &HtmlButtonElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLButtonElement" , js_name = formNoValidate ) ]
    ///Getter for the `formNoValidate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formNoValidate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn form_no_validate(this: &HtmlButtonElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLButtonElement" , js_name = formNoValidate ) ]
    ///Setter for the `formNoValidate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formNoValidate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn set_form_no_validate(this: &HtmlButtonElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLButtonElement" , js_name = formTarget ) ]
    ///Getter for the `formTarget` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formTarget)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn form_target(this: &HtmlButtonElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLButtonElement" , js_name = formTarget ) ]
    ///Setter for the `formTarget` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formTarget)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn set_form_target(this: &HtmlButtonElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLButtonElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn name(this: &HtmlButtonElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLButtonElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn set_name(this: &HtmlButtonElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLButtonElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn type_(this: &HtmlButtonElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLButtonElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn set_type(this: &HtmlButtonElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLButtonElement" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn value(this: &HtmlButtonElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLButtonElement" , js_name = value ) ]
    ///Setter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn set_value(this: &HtmlButtonElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLButtonElement" , js_name = willValidate ) ]
    ///Getter for the `willValidate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/willValidate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn will_validate(this: &HtmlButtonElement) -> bool;

    #[cfg(feature = "ValidityState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLButtonElement" , js_name = validity ) ]
    ///Getter for the `validity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/validity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`, `ValidityState`*
    pub fn validity(this: &HtmlButtonElement) -> ValidityState;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLButtonElement" , js_name = validationMessage ) ]
    ///Getter for the `validationMessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/validationMessage)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn validation_message(this: &HtmlButtonElement) -> Result<String, JsValue>;

    #[cfg(feature = "NodeList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLButtonElement" , js_name = labels ) ]
    ///Getter for the `labels` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/labels)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`, `NodeList`*
    pub fn labels(this: &HtmlButtonElement) -> NodeList;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLButtonElement" , js_name = checkValidity ) ]
    ///The `checkValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/checkValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn check_validity(this: &HtmlButtonElement) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLButtonElement" , js_name = reportValidity ) ]
    ///The `reportValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/reportValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn report_validity(this: &HtmlButtonElement) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLButtonElement" , js_name = setCustomValidity ) ]
    ///The `setCustomValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/setCustomValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlButtonElement`*
    pub fn set_custom_validity(this: &HtmlButtonElement, error: &str);

}
