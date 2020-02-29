use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLOutputElement , typescript_type = "HTMLOutputElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlOutputElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOutputElement`*
    pub type HtmlOutputElement;

    #[cfg(feature = "DomTokenList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = htmlFor ) ]
    ///Getter for the `htmlFor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/htmlFor)
    ///
    ///*This API requires the following crate features to be activated: `DomTokenList`, `HtmlOutputElement`*
    pub fn html_for(this: &HtmlOutputElement) -> DomTokenList;

    #[cfg(feature = "HtmlFormElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = form ) ]
    ///Getter for the `form` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/form)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlOutputElement`*
    pub fn form(this: &HtmlOutputElement) -> Option<HtmlFormElement>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOutputElement`*
    pub fn name(this: &HtmlOutputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOutputElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOutputElement`*
    pub fn set_name(this: &HtmlOutputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOutputElement`*
    pub fn type_(this: &HtmlOutputElement) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = defaultValue ) ]
    ///Getter for the `defaultValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/defaultValue)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOutputElement`*
    pub fn default_value(this: &HtmlOutputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOutputElement" , js_name = defaultValue ) ]
    ///Setter for the `defaultValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/defaultValue)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOutputElement`*
    pub fn set_default_value(this: &HtmlOutputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOutputElement`*
    pub fn value(this: &HtmlOutputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOutputElement" , js_name = value ) ]
    ///Setter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOutputElement`*
    pub fn set_value(this: &HtmlOutputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = willValidate ) ]
    ///Getter for the `willValidate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/willValidate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOutputElement`*
    pub fn will_validate(this: &HtmlOutputElement) -> bool;

    #[cfg(feature = "ValidityState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = validity ) ]
    ///Getter for the `validity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/validity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOutputElement`, `ValidityState`*
    pub fn validity(this: &HtmlOutputElement) -> ValidityState;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLOutputElement" , js_name = validationMessage ) ]
    ///Getter for the `validationMessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/validationMessage)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOutputElement`*
    pub fn validation_message(this: &HtmlOutputElement) -> Result<String, JsValue>;

    #[cfg(feature = "NodeList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOutputElement" , js_name = labels ) ]
    ///Getter for the `labels` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/labels)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOutputElement`, `NodeList`*
    pub fn labels(this: &HtmlOutputElement) -> NodeList;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLOutputElement" , js_name = checkValidity ) ]
    ///The `checkValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/checkValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOutputElement`*
    pub fn check_validity(this: &HtmlOutputElement) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLOutputElement" , js_name = reportValidity ) ]
    ///The `reportValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/reportValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOutputElement`*
    pub fn report_validity(this: &HtmlOutputElement) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLOutputElement" , js_name = setCustomValidity ) ]
    ///The `setCustomValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/setCustomValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOutputElement`*
    pub fn set_custom_validity(this: &HtmlOutputElement, error: &str);

}
