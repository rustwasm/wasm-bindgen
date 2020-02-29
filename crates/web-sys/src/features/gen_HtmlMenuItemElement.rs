use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLMenuItemElement , typescript_name = HTMLMenuItemElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlMenuItemElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuItemElement`*
    pub type HtmlMenuItemElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuItemElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuItemElement`*
    pub fn type_(this: &HtmlMenuItemElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuItemElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuItemElement`*
    pub fn set_type(this: &HtmlMenuItemElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuItemElement" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/label)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuItemElement`*
    pub fn label(this: &HtmlMenuItemElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuItemElement" , js_name = label ) ]
    ///Setter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/label)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuItemElement`*
    pub fn set_label(this: &HtmlMenuItemElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuItemElement" , js_name = icon ) ]
    ///Getter for the `icon` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/icon)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuItemElement`*
    pub fn icon(this: &HtmlMenuItemElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuItemElement" , js_name = icon ) ]
    ///Setter for the `icon` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/icon)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuItemElement`*
    pub fn set_icon(this: &HtmlMenuItemElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuItemElement" , js_name = disabled ) ]
    ///Getter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuItemElement`*
    pub fn disabled(this: &HtmlMenuItemElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuItemElement" , js_name = disabled ) ]
    ///Setter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuItemElement`*
    pub fn set_disabled(this: &HtmlMenuItemElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuItemElement" , js_name = checked ) ]
    ///Getter for the `checked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/checked)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuItemElement`*
    pub fn checked(this: &HtmlMenuItemElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuItemElement" , js_name = checked ) ]
    ///Setter for the `checked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/checked)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuItemElement`*
    pub fn set_checked(this: &HtmlMenuItemElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuItemElement" , js_name = radiogroup ) ]
    ///Getter for the `radiogroup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/radiogroup)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuItemElement`*
    pub fn radiogroup(this: &HtmlMenuItemElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuItemElement" , js_name = radiogroup ) ]
    ///Setter for the `radiogroup` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/radiogroup)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuItemElement`*
    pub fn set_radiogroup(this: &HtmlMenuItemElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuItemElement" , js_name = defaultChecked ) ]
    ///Getter for the `defaultChecked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/defaultChecked)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuItemElement`*
    pub fn default_checked(this: &HtmlMenuItemElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuItemElement" , js_name = defaultChecked ) ]
    ///Setter for the `defaultChecked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/defaultChecked)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuItemElement`*
    pub fn set_default_checked(this: &HtmlMenuItemElement, value: bool);

}
