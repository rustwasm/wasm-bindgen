use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTextAreaElement , typescript_name = HTMLTextAreaElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTextAreaElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub type HtmlTextAreaElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = autocomplete ) ]
    #[doc = "Getter for the `autocomplete` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autocomplete)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn autocomplete(this: &HtmlTextAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = autocomplete ) ]
    #[doc = "Setter for the `autocomplete` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autocomplete)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_autocomplete(this: &HtmlTextAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = autofocus ) ]
    #[doc = "Getter for the `autofocus` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autofocus)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn autofocus(this: &HtmlTextAreaElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = autofocus ) ]
    #[doc = "Setter for the `autofocus` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autofocus)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_autofocus(this: &HtmlTextAreaElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = cols ) ]
    #[doc = "Getter for the `cols` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/cols)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn cols(this: &HtmlTextAreaElement) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = cols ) ]
    #[doc = "Setter for the `cols` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/cols)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_cols(this: &HtmlTextAreaElement, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = disabled ) ]
    #[doc = "Getter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn disabled(this: &HtmlTextAreaElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = disabled ) ]
    #[doc = "Setter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_disabled(this: &HtmlTextAreaElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = form ) ]
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/form)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlTextAreaElement`*"]
    pub fn form(this: &HtmlTextAreaElement) -> Option<HtmlFormElement>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = maxLength ) ]
    #[doc = "Getter for the `maxLength` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/maxLength)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn max_length(this: &HtmlTextAreaElement) -> i32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = maxLength ) ]
    #[doc = "Setter for the `maxLength` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/maxLength)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_max_length(this: &HtmlTextAreaElement, value: i32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = minLength ) ]
    #[doc = "Getter for the `minLength` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/minLength)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn min_length(this: &HtmlTextAreaElement) -> i32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = minLength ) ]
    #[doc = "Setter for the `minLength` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/minLength)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_min_length(this: &HtmlTextAreaElement, value: i32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/name)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn name(this: &HtmlTextAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/name)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_name(this: &HtmlTextAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = placeholder ) ]
    #[doc = "Getter for the `placeholder` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/placeholder)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn placeholder(this: &HtmlTextAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = placeholder ) ]
    #[doc = "Setter for the `placeholder` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/placeholder)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_placeholder(this: &HtmlTextAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = readOnly ) ]
    #[doc = "Getter for the `readOnly` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/readOnly)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn read_only(this: &HtmlTextAreaElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = readOnly ) ]
    #[doc = "Setter for the `readOnly` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/readOnly)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_read_only(this: &HtmlTextAreaElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = required ) ]
    #[doc = "Getter for the `required` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/required)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn required(this: &HtmlTextAreaElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = required ) ]
    #[doc = "Setter for the `required` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/required)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_required(this: &HtmlTextAreaElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = rows ) ]
    #[doc = "Getter for the `rows` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/rows)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn rows(this: &HtmlTextAreaElement) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = rows ) ]
    #[doc = "Setter for the `rows` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/rows)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_rows(this: &HtmlTextAreaElement, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = wrap ) ]
    #[doc = "Getter for the `wrap` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/wrap)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn wrap(this: &HtmlTextAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = wrap ) ]
    #[doc = "Setter for the `wrap` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/wrap)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_wrap(this: &HtmlTextAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/type)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn type_(this: &HtmlTextAreaElement) -> String;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLTextAreaElement" , js_name = defaultValue ) ]
    #[doc = "Getter for the `defaultValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/defaultValue)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn default_value(this: &HtmlTextAreaElement) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "HTMLTextAreaElement" , js_name = defaultValue ) ]
    #[doc = "Setter for the `defaultValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/defaultValue)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_default_value(this: &HtmlTextAreaElement, value: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/value)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn value(this: &HtmlTextAreaElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTextAreaElement" , js_name = value ) ]
    #[doc = "Setter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/value)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_value(this: &HtmlTextAreaElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = textLength ) ]
    #[doc = "Getter for the `textLength` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/textLength)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn text_length(this: &HtmlTextAreaElement) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = willValidate ) ]
    #[doc = "Getter for the `willValidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/willValidate)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn will_validate(this: &HtmlTextAreaElement) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = validity ) ]
    #[cfg(feature = "ValidityState")]
    #[doc = "Getter for the `validity` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/validity)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`, `ValidityState`*"]
    pub fn validity(this: &HtmlTextAreaElement) -> ValidityState;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLTextAreaElement" , js_name = validationMessage ) ]
    #[doc = "Getter for the `validationMessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/validationMessage)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn validation_message(this: &HtmlTextAreaElement) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTextAreaElement" , js_name = labels ) ]
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `labels` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/labels)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`, `NodeList`*"]
    pub fn labels(this: &HtmlTextAreaElement) -> NodeList;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLTextAreaElement" , js_name = selectionStart ) ]
    #[doc = "Getter for the `selectionStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionStart)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn selection_start(this: &HtmlTextAreaElement) -> Result<Option<u32>, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "HTMLTextAreaElement" , js_name = selectionStart ) ]
    #[doc = "Setter for the `selectionStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionStart)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_selection_start(
        this: &HtmlTextAreaElement,
        value: Option<u32>,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLTextAreaElement" , js_name = selectionEnd ) ]
    #[doc = "Getter for the `selectionEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionEnd)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn selection_end(this: &HtmlTextAreaElement) -> Result<Option<u32>, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "HTMLTextAreaElement" , js_name = selectionEnd ) ]
    #[doc = "Setter for the `selectionEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionEnd)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_selection_end(this: &HtmlTextAreaElement, value: Option<u32>)
        -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLTextAreaElement" , js_name = selectionDirection ) ]
    #[doc = "Getter for the `selectionDirection` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionDirection)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn selection_direction(this: &HtmlTextAreaElement) -> Result<Option<String>, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "HTMLTextAreaElement" , js_name = selectionDirection ) ]
    #[doc = "Setter for the `selectionDirection` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionDirection)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_selection_direction(
        this: &HtmlTextAreaElement,
        value: Option<&str>,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLTextAreaElement" , js_name = checkValidity ) ]
    #[doc = "The `checkValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/checkValidity)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn check_validity(this: &HtmlTextAreaElement) -> bool;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLTextAreaElement" , js_name = reportValidity ) ]
    #[doc = "The `reportValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/reportValidity)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn report_validity(this: &HtmlTextAreaElement) -> bool;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLTextAreaElement" , js_name = select ) ]
    #[doc = "The `select()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/select)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn select(this: &HtmlTextAreaElement);
    # [ wasm_bindgen ( method , structural , js_class = "HTMLTextAreaElement" , js_name = setCustomValidity ) ]
    #[doc = "The `setCustomValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setCustomValidity)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_custom_validity(this: &HtmlTextAreaElement, error: &str);
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTextAreaElement" , js_name = setRangeText ) ]
    #[doc = "The `setRangeText()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setRangeText)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_range_text(this: &HtmlTextAreaElement, replacement: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTextAreaElement" , js_name = setRangeText ) ]
    #[doc = "The `setRangeText()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setRangeText)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_range_text_with_start_and_end(
        this: &HtmlTextAreaElement,
        replacement: &str,
        start: u32,
        end: u32,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTextAreaElement" , js_name = setSelectionRange ) ]
    #[doc = "The `setSelectionRange()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setSelectionRange)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_selection_range(
        this: &HtmlTextAreaElement,
        start: u32,
        end: u32,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLTextAreaElement" , js_name = setSelectionRange ) ]
    #[doc = "The `setSelectionRange()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setSelectionRange)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    pub fn set_selection_range_with_direction(
        this: &HtmlTextAreaElement,
        start: u32,
        end: u32,
        direction: &str,
    ) -> Result<(), JsValue>;
}
