use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLSelectElement , typescript_name = HTMLSelectElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlSelectElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub type HtmlSelectElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = autofocus ) ]
    #[doc = "Getter for the `autofocus` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autofocus)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn autofocus(this: &HtmlSelectElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = autofocus ) ]
    #[doc = "Setter for the `autofocus` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autofocus)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_autofocus(this: &HtmlSelectElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = autocomplete ) ]
    #[doc = "Getter for the `autocomplete` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autocomplete)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn autocomplete(this: &HtmlSelectElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = autocomplete ) ]
    #[doc = "Setter for the `autocomplete` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autocomplete)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_autocomplete(this: &HtmlSelectElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = disabled ) ]
    #[doc = "Getter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn disabled(this: &HtmlSelectElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = disabled ) ]
    #[doc = "Setter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_disabled(this: &HtmlSelectElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = form ) ]
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/form)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlSelectElement`*"]
    pub fn form(this: &HtmlSelectElement) -> Option<HtmlFormElement>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = multiple ) ]
    #[doc = "Getter for the `multiple` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/multiple)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn multiple(this: &HtmlSelectElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = multiple ) ]
    #[doc = "Setter for the `multiple` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/multiple)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_multiple(this: &HtmlSelectElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/name)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn name(this: &HtmlSelectElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/name)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_name(this: &HtmlSelectElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = required ) ]
    #[doc = "Getter for the `required` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/required)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn required(this: &HtmlSelectElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = required ) ]
    #[doc = "Setter for the `required` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/required)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_required(this: &HtmlSelectElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = size ) ]
    #[doc = "Getter for the `size` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/size)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn size(this: &HtmlSelectElement) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = size ) ]
    #[doc = "Setter for the `size` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/size)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_size(this: &HtmlSelectElement, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/type)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn type_(this: &HtmlSelectElement) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = options ) ]
    #[cfg(feature = "HtmlOptionsCollection")]
    #[doc = "Getter for the `options` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/options)\n\n*This API requires the following crate features to be activated: `HtmlOptionsCollection`, `HtmlSelectElement`*"]
    pub fn options(this: &HtmlSelectElement) -> HtmlOptionsCollection;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/length)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn length(this: &HtmlSelectElement) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = length ) ]
    #[doc = "Setter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/length)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_length(this: &HtmlSelectElement, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = selectedOptions ) ]
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `selectedOptions` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedOptions)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlSelectElement`*"]
    pub fn selected_options(this: &HtmlSelectElement) -> HtmlCollection;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = selectedIndex ) ]
    #[doc = "Getter for the `selectedIndex` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedIndex)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn selected_index(this: &HtmlSelectElement) -> i32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = selectedIndex ) ]
    #[doc = "Setter for the `selectedIndex` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedIndex)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_selected_index(this: &HtmlSelectElement, value: i32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/value)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn value(this: &HtmlSelectElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = value ) ]
    #[doc = "Setter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/value)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_value(this: &HtmlSelectElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = willValidate ) ]
    #[doc = "Getter for the `willValidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/willValidate)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn will_validate(this: &HtmlSelectElement) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = validity ) ]
    #[cfg(feature = "ValidityState")]
    #[doc = "Getter for the `validity` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/validity)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`, `ValidityState`*"]
    pub fn validity(this: &HtmlSelectElement) -> ValidityState;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLSelectElement" , js_name = validationMessage ) ]
    #[doc = "Getter for the `validationMessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/validationMessage)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn validation_message(this: &HtmlSelectElement) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = labels ) ]
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `labels` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/labels)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`, `NodeList`*"]
    pub fn labels(this: &HtmlSelectElement) -> NodeList;
    #[cfg(feature = "HtmlOptionElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLSelectElement" , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
    pub fn add_with_html_option_element(
        this: &HtmlSelectElement,
        element: &HtmlOptionElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlOptGroupElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLSelectElement" , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)\n\n*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlSelectElement`*"]
    pub fn add_with_html_opt_group_element(
        this: &HtmlSelectElement,
        element: &HtmlOptGroupElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlOptionElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLSelectElement" , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
    pub fn add_with_html_option_element_and_opt_html_element(
        this: &HtmlSelectElement,
        element: &HtmlOptionElement,
        before: Option<&HtmlElement>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlOptGroupElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLSelectElement" , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)\n\n*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlSelectElement`*"]
    pub fn add_with_html_opt_group_element_and_opt_html_element(
        this: &HtmlSelectElement,
        element: &HtmlOptGroupElement,
        before: Option<&HtmlElement>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlOptionElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLSelectElement" , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
    pub fn add_with_html_option_element_and_opt_i32(
        this: &HtmlSelectElement,
        element: &HtmlOptionElement,
        before: Option<i32>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlOptGroupElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLSelectElement" , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)\n\n*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlSelectElement`*"]
    pub fn add_with_html_opt_group_element_and_opt_i32(
        this: &HtmlSelectElement,
        element: &HtmlOptGroupElement,
        before: Option<i32>,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLSelectElement" , js_name = checkValidity ) ]
    #[doc = "The `checkValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/checkValidity)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn check_validity(this: &HtmlSelectElement) -> bool;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLSelectElement" , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/item)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn item(this: &HtmlSelectElement, index: u32) -> Option<Element>;
    #[cfg(feature = "HtmlOptionElement")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLSelectElement" , js_name = namedItem ) ]
    #[doc = "The `namedItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/namedItem)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
    pub fn named_item(this: &HtmlSelectElement, name: &str) -> Option<HtmlOptionElement>;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLSelectElement" , js_name = remove ) ]
    #[doc = "The `remove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/remove)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn remove_with_index(this: &HtmlSelectElement, index: i32);
    # [ wasm_bindgen ( method , structural , js_class = "HTMLSelectElement" , js_name = remove ) ]
    #[doc = "The `remove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/remove)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn remove(this: &HtmlSelectElement);
    # [ wasm_bindgen ( method , structural , js_class = "HTMLSelectElement" , js_name = reportValidity ) ]
    #[doc = "The `reportValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/reportValidity)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn report_validity(this: &HtmlSelectElement) -> bool;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLSelectElement" , js_name = setCustomValidity ) ]
    #[doc = "The `setCustomValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/setCustomValidity)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn set_custom_validity(this: &HtmlSelectElement, error: &str);
    #[wasm_bindgen(method, structural, js_class = "HTMLSelectElement", indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    pub fn get(this: &HtmlSelectElement, index: u32) -> Option<Element>;
    #[cfg(feature = "HtmlOptionElement")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "HTMLSelectElement",
        indexing_setter
    )]
    #[doc = "Indexing setter.\n\n\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
    pub fn set(
        this: &HtmlSelectElement,
        index: u32,
        option: Option<&HtmlOptionElement>,
    ) -> Result<(), JsValue>;
}
