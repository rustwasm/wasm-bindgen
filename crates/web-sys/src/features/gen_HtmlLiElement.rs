use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLLIElement , typescript_name = HTMLLIElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlLiElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLiElement`*
    pub type HtmlLiElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLIElement" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLiElement`*
    pub fn value(this: &HtmlLiElement) -> i32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLIElement" , js_name = value ) ]
    ///Setter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLiElement`*
    pub fn set_value(this: &HtmlLiElement, value: i32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLLIElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLiElement`*
    pub fn type_(this: &HtmlLiElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLLIElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlLiElement`*
    pub fn set_type(this: &HtmlLiElement, value: &str);

}
