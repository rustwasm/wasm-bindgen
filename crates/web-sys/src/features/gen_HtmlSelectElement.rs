use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLSelectElement , typescript_type = "HTMLSelectElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlSelectElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub type HtmlSelectElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = autofocus ) ]
    ///Getter for the `autofocus` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autofocus)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn autofocus(this: &HtmlSelectElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = autofocus ) ]
    ///Setter for the `autofocus` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autofocus)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn set_autofocus(this: &HtmlSelectElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = autocomplete ) ]
    ///Getter for the `autocomplete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autocomplete)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn autocomplete(this: &HtmlSelectElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = autocomplete ) ]
    ///Setter for the `autocomplete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autocomplete)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn set_autocomplete(this: &HtmlSelectElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = disabled ) ]
    ///Getter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn disabled(this: &HtmlSelectElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = disabled ) ]
    ///Setter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn set_disabled(this: &HtmlSelectElement, value: bool);

    #[cfg(feature = "HtmlFormElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = form ) ]
    ///Getter for the `form` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/form)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlSelectElement`*
    pub fn form(this: &HtmlSelectElement) -> Option<HtmlFormElement>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = multiple ) ]
    ///Getter for the `multiple` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/multiple)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn multiple(this: &HtmlSelectElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = multiple ) ]
    ///Setter for the `multiple` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/multiple)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn set_multiple(this: &HtmlSelectElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn name(this: &HtmlSelectElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn set_name(this: &HtmlSelectElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = required ) ]
    ///Getter for the `required` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/required)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn required(this: &HtmlSelectElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = required ) ]
    ///Setter for the `required` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/required)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn set_required(this: &HtmlSelectElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = size ) ]
    ///Getter for the `size` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/size)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn size(this: &HtmlSelectElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = size ) ]
    ///Setter for the `size` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/size)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn set_size(this: &HtmlSelectElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn type_(this: &HtmlSelectElement) -> String;

    #[cfg(feature = "HtmlOptionsCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = options ) ]
    ///Getter for the `options` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/options)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionsCollection`, `HtmlSelectElement`*
    pub fn options(this: &HtmlSelectElement) -> HtmlOptionsCollection;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/length)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn length(this: &HtmlSelectElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = length ) ]
    ///Setter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/length)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn set_length(this: &HtmlSelectElement, value: u32);

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = selectedOptions ) ]
    ///Getter for the `selectedOptions` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedOptions)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlSelectElement`*
    pub fn selected_options(this: &HtmlSelectElement) -> HtmlCollection;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = selectedIndex ) ]
    ///Getter for the `selectedIndex` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedIndex)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn selected_index(this: &HtmlSelectElement) -> i32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = selectedIndex ) ]
    ///Setter for the `selectedIndex` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedIndex)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn set_selected_index(this: &HtmlSelectElement, value: i32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn value(this: &HtmlSelectElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSelectElement" , js_name = value ) ]
    ///Setter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn set_value(this: &HtmlSelectElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = willValidate ) ]
    ///Getter for the `willValidate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/willValidate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn will_validate(this: &HtmlSelectElement) -> bool;

    #[cfg(feature = "ValidityState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = validity ) ]
    ///Getter for the `validity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/validity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`, `ValidityState`*
    pub fn validity(this: &HtmlSelectElement) -> ValidityState;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLSelectElement" , js_name = validationMessage ) ]
    ///Getter for the `validationMessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/validationMessage)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn validation_message(this: &HtmlSelectElement) -> Result<String, JsValue>;

    #[cfg(feature = "NodeList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSelectElement" , js_name = labels ) ]
    ///Getter for the `labels` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/labels)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`, `NodeList`*
    pub fn labels(this: &HtmlSelectElement) -> NodeList;

    #[cfg(feature = "HtmlOptionElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLSelectElement" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*
    pub fn add_with_html_option_element(
        this: &HtmlSelectElement,
        element: &HtmlOptionElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlOptGroupElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLSelectElement" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlSelectElement`*
    pub fn add_with_html_opt_group_element(
        this: &HtmlSelectElement,
        element: &HtmlOptGroupElement,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlOptionElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLSelectElement" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*
    pub fn add_with_html_option_element_and_opt_html_element(
        this: &HtmlSelectElement,
        element: &HtmlOptionElement,
        before: Option<&HtmlElement>,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlOptGroupElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLSelectElement" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlSelectElement`*
    pub fn add_with_html_opt_group_element_and_opt_html_element(
        this: &HtmlSelectElement,
        element: &HtmlOptGroupElement,
        before: Option<&HtmlElement>,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlOptionElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLSelectElement" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*
    pub fn add_with_html_option_element_and_opt_i32(
        this: &HtmlSelectElement,
        element: &HtmlOptionElement,
        before: Option<i32>,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "HtmlOptGroupElement")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLSelectElement" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlSelectElement`*
    pub fn add_with_html_opt_group_element_and_opt_i32(
        this: &HtmlSelectElement,
        element: &HtmlOptGroupElement,
        before: Option<i32>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLSelectElement" , js_name = checkValidity ) ]
    ///The `checkValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/checkValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn check_validity(this: &HtmlSelectElement) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLSelectElement" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/item)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn item(this: &HtmlSelectElement, index: u32) -> Option<Element>;

    #[cfg(feature = "HtmlOptionElement")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLSelectElement" , js_name = namedItem ) ]
    ///The `namedItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/namedItem)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*
    pub fn named_item(this: &HtmlSelectElement, name: &str) -> Option<HtmlOptionElement>;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLSelectElement" , js_name = remove ) ]
    ///The `remove()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/remove)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn remove_with_index(this: &HtmlSelectElement, index: i32);

    # [ wasm_bindgen ( method , structural , js_class = "HTMLSelectElement" , js_name = remove ) ]
    ///The `remove()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/remove)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn remove(this: &HtmlSelectElement);

    # [ wasm_bindgen ( method , structural , js_class = "HTMLSelectElement" , js_name = reportValidity ) ]
    ///The `reportValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/reportValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn report_validity(this: &HtmlSelectElement) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLSelectElement" , js_name = setCustomValidity ) ]
    ///The `setCustomValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/setCustomValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn set_custom_validity(this: &HtmlSelectElement, error: &str);

    #[wasm_bindgen(method, structural, js_class = "HTMLSelectElement", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `HtmlSelectElement`*
    pub fn get(this: &HtmlSelectElement, index: u32) -> Option<Element>;

    #[cfg(feature = "HtmlOptionElement")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "HTMLSelectElement",
        indexing_setter
    )]
    ///Indexing setter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*
    pub fn set(
        this: &HtmlSelectElement,
        index: u32,
        option: Option<&HtmlOptionElement>,
    ) -> Result<(), JsValue>;

}
