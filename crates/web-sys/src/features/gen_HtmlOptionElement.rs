use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLOptionElement , typescript_name = HTMLOptionElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlOptionElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub type HtmlOptionElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOptionElement" , js_name = disabled ) ]
    ///Getter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn disabled(this: &HtmlOptionElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOptionElement" , js_name = disabled ) ]
    ///Setter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn set_disabled(this: &HtmlOptionElement, value: bool);

    #[cfg(feature = "HtmlFormElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOptionElement" , js_name = form ) ]
    ///Getter for the `form` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/form)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlOptionElement`*
    pub fn form(this: &HtmlOptionElement) -> Option<HtmlFormElement>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOptionElement" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/label)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn label(this: &HtmlOptionElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOptionElement" , js_name = label ) ]
    ///Setter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/label)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn set_label(this: &HtmlOptionElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOptionElement" , js_name = defaultSelected ) ]
    ///Getter for the `defaultSelected` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/defaultSelected)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn default_selected(this: &HtmlOptionElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOptionElement" , js_name = defaultSelected ) ]
    ///Setter for the `defaultSelected` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/defaultSelected)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn set_default_selected(this: &HtmlOptionElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOptionElement" , js_name = selected ) ]
    ///Getter for the `selected` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/selected)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn selected(this: &HtmlOptionElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOptionElement" , js_name = selected ) ]
    ///Setter for the `selected` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/selected)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn set_selected(this: &HtmlOptionElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOptionElement" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn value(this: &HtmlOptionElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOptionElement" , js_name = value ) ]
    ///Setter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn set_value(this: &HtmlOptionElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOptionElement" , js_name = text ) ]
    ///Getter for the `text` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/text)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn text(this: &HtmlOptionElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOptionElement" , js_name = text ) ]
    ///Setter for the `text` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/text)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn set_text(this: &HtmlOptionElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOptionElement" , js_name = index ) ]
    ///Getter for the `index` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/index)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn index(this: &HtmlOptionElement) -> i32;

    #[wasm_bindgen(catch, constructor, js_class = "HTMLOptionElement")]
    ///The `new HtmlOptionElement(..)` constructor, creating a new instance of `HtmlOptionElement`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn new() -> Result<HtmlOptionElement, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "HTMLOptionElement")]
    ///The `new HtmlOptionElement(..)` constructor, creating a new instance of `HtmlOptionElement`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn new_with_text(text: &str) -> Result<HtmlOptionElement, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "HTMLOptionElement")]
    ///The `new HtmlOptionElement(..)` constructor, creating a new instance of `HtmlOptionElement`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn new_with_text_and_value(text: &str, value: &str) -> Result<HtmlOptionElement, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "HTMLOptionElement")]
    ///The `new HtmlOptionElement(..)` constructor, creating a new instance of `HtmlOptionElement`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn new_with_text_and_value_and_default_selected(
        text: &str,
        value: &str,
        default_selected: bool,
    ) -> Result<HtmlOptionElement, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "HTMLOptionElement")]
    ///The `new HtmlOptionElement(..)` constructor, creating a new instance of `HtmlOptionElement`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOptionElement`*
    pub fn new_with_text_and_value_and_default_selected_and_selected(
        text: &str,
        value: &str,
        default_selected: bool,
        selected: bool,
    ) -> Result<HtmlOptionElement, JsValue>;

}
