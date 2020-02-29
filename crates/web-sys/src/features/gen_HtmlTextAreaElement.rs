use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTextAreaElement , typescript_type = "HTMLTextAreaElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlTextAreaElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub type HtmlTextAreaElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = autocomplete ) ]
    ///Getter for the `autocomplete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autocomplete)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn autocomplete(this: &HtmlTextAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = autocomplete ) ]
    ///Setter for the `autocomplete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autocomplete)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_autocomplete(this: &HtmlTextAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = autofocus ) ]
    ///Getter for the `autofocus` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autofocus)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn autofocus(this: &HtmlTextAreaElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = autofocus ) ]
    ///Setter for the `autofocus` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autofocus)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_autofocus(this: &HtmlTextAreaElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = cols ) ]
    ///Getter for the `cols` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/cols)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn cols(this: &HtmlTextAreaElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = cols ) ]
    ///Setter for the `cols` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/cols)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_cols(this: &HtmlTextAreaElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = disabled ) ]
    ///Getter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn disabled(this: &HtmlTextAreaElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = disabled ) ]
    ///Setter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_disabled(this: &HtmlTextAreaElement, value: bool);

    #[cfg(feature = "HtmlFormElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = form ) ]
    ///Getter for the `form` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/form)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlTextAreaElement`*
    pub fn form(this: &HtmlTextAreaElement) -> Option<HtmlFormElement>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = maxLength ) ]
    ///Getter for the `maxLength` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/maxLength)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn max_length(this: &HtmlTextAreaElement) -> i32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = maxLength ) ]
    ///Setter for the `maxLength` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/maxLength)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_max_length(this: &HtmlTextAreaElement, value: i32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = minLength ) ]
    ///Getter for the `minLength` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/minLength)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn min_length(this: &HtmlTextAreaElement) -> i32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = minLength ) ]
    ///Setter for the `minLength` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/minLength)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_min_length(this: &HtmlTextAreaElement, value: i32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn name(this: &HtmlTextAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_name(this: &HtmlTextAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = placeholder ) ]
    ///Getter for the `placeholder` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/placeholder)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn placeholder(this: &HtmlTextAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = placeholder ) ]
    ///Setter for the `placeholder` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/placeholder)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_placeholder(this: &HtmlTextAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = readOnly ) ]
    ///Getter for the `readOnly` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/readOnly)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn read_only(this: &HtmlTextAreaElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = readOnly ) ]
    ///Setter for the `readOnly` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/readOnly)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_read_only(this: &HtmlTextAreaElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = required ) ]
    ///Getter for the `required` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/required)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn required(this: &HtmlTextAreaElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = required ) ]
    ///Setter for the `required` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/required)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_required(this: &HtmlTextAreaElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = rows ) ]
    ///Getter for the `rows` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/rows)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn rows(this: &HtmlTextAreaElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = rows ) ]
    ///Setter for the `rows` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/rows)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_rows(this: &HtmlTextAreaElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = wrap ) ]
    ///Getter for the `wrap` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/wrap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn wrap(this: &HtmlTextAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = wrap ) ]
    ///Setter for the `wrap` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/wrap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_wrap(this: &HtmlTextAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn type_(this: &HtmlTextAreaElement) -> String;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLTextAreaElement" , js_name = defaultValue ) ]
    ///Getter for the `defaultValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/defaultValue)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn default_value(this: &HtmlTextAreaElement) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "HTMLTextAreaElement" , js_name = defaultValue ) ]
    ///Setter for the `defaultValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/defaultValue)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_default_value(this: &HtmlTextAreaElement, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn value(this: &HtmlTextAreaElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = value ) ]
    ///Setter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_value(this: &HtmlTextAreaElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = textLength ) ]
    ///Getter for the `textLength` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/textLength)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn text_length(this: &HtmlTextAreaElement) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = willValidate ) ]
    ///Getter for the `willValidate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/willValidate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn will_validate(this: &HtmlTextAreaElement) -> bool;

    #[cfg(feature = "ValidityState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = validity ) ]
    ///Getter for the `validity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/validity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`, `ValidityState`*
    pub fn validity(this: &HtmlTextAreaElement) -> ValidityState;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLTextAreaElement" , js_name = validationMessage ) ]
    ///Getter for the `validationMessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/validationMessage)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn validation_message(this: &HtmlTextAreaElement) -> Result<String, JsValue>;

    #[cfg(feature = "NodeList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = labels ) ]
    ///Getter for the `labels` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/labels)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`, `NodeList`*
    pub fn labels(this: &HtmlTextAreaElement) -> NodeList;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLTextAreaElement" , js_name = selectionStart ) ]
    ///Getter for the `selectionStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionStart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn selection_start(this: &HtmlTextAreaElement) -> Result<Option<u32>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "HTMLTextAreaElement" , js_name = selectionStart ) ]
    ///Setter for the `selectionStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionStart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_selection_start(
        this: &HtmlTextAreaElement,
        value: Option<u32>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLTextAreaElement" , js_name = selectionEnd ) ]
    ///Getter for the `selectionEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionEnd)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn selection_end(this: &HtmlTextAreaElement) -> Result<Option<u32>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "HTMLTextAreaElement" , js_name = selectionEnd ) ]
    ///Setter for the `selectionEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionEnd)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_selection_end(this: &HtmlTextAreaElement, value: Option<u32>)
        -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLTextAreaElement" , js_name = selectionDirection ) ]
    ///Getter for the `selectionDirection` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionDirection)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn selection_direction(this: &HtmlTextAreaElement) -> Result<Option<String>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "HTMLTextAreaElement" , js_name = selectionDirection ) ]
    ///Setter for the `selectionDirection` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionDirection)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_selection_direction(
        this: &HtmlTextAreaElement,
        value: Option<&str>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLTextAreaElement" , js_name = checkValidity ) ]
    ///The `checkValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/checkValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn check_validity(this: &HtmlTextAreaElement) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLTextAreaElement" , js_name = reportValidity ) ]
    ///The `reportValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/reportValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn report_validity(this: &HtmlTextAreaElement) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLTextAreaElement" , js_name = select ) ]
    ///The `select()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/select)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn select(this: &HtmlTextAreaElement);

    # [ wasm_bindgen ( method , structural , js_class = "HTMLTextAreaElement" , js_name = setCustomValidity ) ]
    ///The `setCustomValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setCustomValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_custom_validity(this: &HtmlTextAreaElement, error: &str);

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTextAreaElement" , js_name = setRangeText ) ]
    ///The `setRangeText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setRangeText)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_range_text(this: &HtmlTextAreaElement, replacement: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTextAreaElement" , js_name = setRangeText ) ]
    ///The `setRangeText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setRangeText)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_range_text_with_start_and_end(
        this: &HtmlTextAreaElement,
        replacement: &str,
        start: u32,
        end: u32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTextAreaElement" , js_name = setSelectionRange ) ]
    ///The `setSelectionRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setSelectionRange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_selection_range(
        this: &HtmlTextAreaElement,
        start: u32,
        end: u32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTextAreaElement" , js_name = setSelectionRange ) ]
    ///The `setSelectionRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setSelectionRange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTextAreaElement`*
    pub fn set_selection_range_with_direction(
        this: &HtmlTextAreaElement,
        start: u32,
        end: u32,
        direction: &str,
    ) -> Result<(), JsValue>;

}
