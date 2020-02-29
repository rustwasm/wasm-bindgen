use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLFieldSetElement , typescript_type = "HTMLFieldSetElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlFieldSetElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFieldSetElement`*
    pub type HtmlFieldSetElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFieldSetElement" , js_name = disabled ) ]
    ///Getter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFieldSetElement`*
    pub fn disabled(this: &HtmlFieldSetElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFieldSetElement" , js_name = disabled ) ]
    ///Setter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFieldSetElement`*
    pub fn set_disabled(this: &HtmlFieldSetElement, value: bool);

    #[cfg(feature = "HtmlFormElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFieldSetElement" , js_name = form ) ]
    ///Getter for the `form` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/form)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFieldSetElement`, `HtmlFormElement`*
    pub fn form(this: &HtmlFieldSetElement) -> Option<HtmlFormElement>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFieldSetElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFieldSetElement`*
    pub fn name(this: &HtmlFieldSetElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFieldSetElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFieldSetElement`*
    pub fn set_name(this: &HtmlFieldSetElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFieldSetElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFieldSetElement`*
    pub fn type_(this: &HtmlFieldSetElement) -> String;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFieldSetElement" , js_name = elements ) ]
    ///Getter for the `elements` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/elements)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlFieldSetElement`*
    pub fn elements(this: &HtmlFieldSetElement) -> HtmlCollection;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFieldSetElement" , js_name = willValidate ) ]
    ///Getter for the `willValidate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/willValidate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFieldSetElement`*
    pub fn will_validate(this: &HtmlFieldSetElement) -> bool;

    #[cfg(feature = "ValidityState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFieldSetElement" , js_name = validity ) ]
    ///Getter for the `validity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/validity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFieldSetElement`, `ValidityState`*
    pub fn validity(this: &HtmlFieldSetElement) -> ValidityState;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLFieldSetElement" , js_name = validationMessage ) ]
    ///Getter for the `validationMessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/validationMessage)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFieldSetElement`*
    pub fn validation_message(this: &HtmlFieldSetElement) -> Result<String, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLFieldSetElement" , js_name = checkValidity ) ]
    ///The `checkValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/checkValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFieldSetElement`*
    pub fn check_validity(this: &HtmlFieldSetElement) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLFieldSetElement" , js_name = reportValidity ) ]
    ///The `reportValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/reportValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFieldSetElement`*
    pub fn report_validity(this: &HtmlFieldSetElement) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLFieldSetElement" , js_name = setCustomValidity ) ]
    ///The `setCustomValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/setCustomValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFieldSetElement`*
    pub fn set_custom_validity(this: &HtmlFieldSetElement, error: &str);

}
