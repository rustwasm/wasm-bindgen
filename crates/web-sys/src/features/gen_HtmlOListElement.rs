use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLOListElement , typescript_type = "HTMLOListElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlOListElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOListElement`*
    pub type HtmlOListElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOListElement" , js_name = reversed ) ]
    ///Getter for the `reversed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/reversed)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOListElement`*
    pub fn reversed(this: &HtmlOListElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOListElement" , js_name = reversed ) ]
    ///Setter for the `reversed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/reversed)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOListElement`*
    pub fn set_reversed(this: &HtmlOListElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOListElement" , js_name = start ) ]
    ///Getter for the `start` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/start)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOListElement`*
    pub fn start(this: &HtmlOListElement) -> i32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOListElement" , js_name = start ) ]
    ///Setter for the `start` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/start)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOListElement`*
    pub fn set_start(this: &HtmlOListElement, value: i32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOListElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOListElement`*
    pub fn type_(this: &HtmlOListElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOListElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOListElement`*
    pub fn set_type(this: &HtmlOListElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLOListElement" , js_name = compact ) ]
    ///Getter for the `compact` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/compact)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOListElement`*
    pub fn compact(this: &HtmlOListElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLOListElement" , js_name = compact ) ]
    ///Setter for the `compact` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/compact)
    ///
    ///*This API requires the following crate features to be activated: `HtmlOListElement`*
    pub fn set_compact(this: &HtmlOListElement, value: bool);

}
