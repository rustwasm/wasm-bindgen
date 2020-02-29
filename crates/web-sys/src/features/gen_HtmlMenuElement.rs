use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLMenuElement , typescript_type = "HTMLMenuElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlMenuElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuElement`*
    pub type HtmlMenuElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuElement`*
    pub fn type_(this: &HtmlMenuElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuElement`*
    pub fn set_type(this: &HtmlMenuElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuElement" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement/label)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuElement`*
    pub fn label(this: &HtmlMenuElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuElement" , js_name = label ) ]
    ///Setter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement/label)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuElement`*
    pub fn set_label(this: &HtmlMenuElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMenuElement" , js_name = compact ) ]
    ///Getter for the `compact` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement/compact)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuElement`*
    pub fn compact(this: &HtmlMenuElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMenuElement" , js_name = compact ) ]
    ///Setter for the `compact` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement/compact)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMenuElement`*
    pub fn set_compact(this: &HtmlMenuElement, value: bool);

}
