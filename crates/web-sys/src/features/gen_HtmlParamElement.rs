use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLParamElement , typescript_type = "HTMLParamElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlParamElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlParamElement`*
    pub type HtmlParamElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLParamElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlParamElement`*
    pub fn name(this: &HtmlParamElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLParamElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlParamElement`*
    pub fn set_name(this: &HtmlParamElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLParamElement" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlParamElement`*
    pub fn value(this: &HtmlParamElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLParamElement" , js_name = value ) ]
    ///Setter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlParamElement`*
    pub fn set_value(this: &HtmlParamElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLParamElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlParamElement`*
    pub fn type_(this: &HtmlParamElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLParamElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlParamElement`*
    pub fn set_type(this: &HtmlParamElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLParamElement" , js_name = valueType ) ]
    ///Getter for the `valueType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/valueType)
    ///
    ///*This API requires the following crate features to be activated: `HtmlParamElement`*
    pub fn value_type(this: &HtmlParamElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLParamElement" , js_name = valueType ) ]
    ///Setter for the `valueType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParamElement/valueType)
    ///
    ///*This API requires the following crate features to be activated: `HtmlParamElement`*
    pub fn set_value_type(this: &HtmlParamElement, value: &str);

}
