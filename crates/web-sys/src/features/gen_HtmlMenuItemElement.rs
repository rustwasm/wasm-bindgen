use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLMenuItemElement , typescript_name = HTMLMenuItemElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlMenuItemElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    pub type HtmlMenuItemElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuItemElement" , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/type)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    pub fn type_(this: &HtmlMenuItemElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuItemElement" , js_name = type ) ]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/type)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    pub fn set_type(this: &HtmlMenuItemElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuItemElement" , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/label)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    pub fn label(this: &HtmlMenuItemElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuItemElement" , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/label)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    pub fn set_label(this: &HtmlMenuItemElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuItemElement" , js_name = icon ) ]
    #[doc = "Getter for the `icon` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/icon)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    pub fn icon(this: &HtmlMenuItemElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuItemElement" , js_name = icon ) ]
    #[doc = "Setter for the `icon` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/icon)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    pub fn set_icon(this: &HtmlMenuItemElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuItemElement" , js_name = disabled ) ]
    #[doc = "Getter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    pub fn disabled(this: &HtmlMenuItemElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuItemElement" , js_name = disabled ) ]
    #[doc = "Setter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    pub fn set_disabled(this: &HtmlMenuItemElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuItemElement" , js_name = checked ) ]
    #[doc = "Getter for the `checked` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/checked)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    pub fn checked(this: &HtmlMenuItemElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuItemElement" , js_name = checked ) ]
    #[doc = "Setter for the `checked` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/checked)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    pub fn set_checked(this: &HtmlMenuItemElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuItemElement" , js_name = radiogroup ) ]
    #[doc = "Getter for the `radiogroup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/radiogroup)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    pub fn radiogroup(this: &HtmlMenuItemElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuItemElement" , js_name = radiogroup ) ]
    #[doc = "Setter for the `radiogroup` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/radiogroup)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    pub fn set_radiogroup(this: &HtmlMenuItemElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuItemElement" , js_name = defaultChecked ) ]
    #[doc = "Getter for the `defaultChecked` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/defaultChecked)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    pub fn default_checked(this: &HtmlMenuItemElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuItemElement" , js_name = defaultChecked ) ]
    #[doc = "Setter for the `defaultChecked` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/defaultChecked)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    pub fn set_default_checked(this: &HtmlMenuItemElement, value: bool);
}
