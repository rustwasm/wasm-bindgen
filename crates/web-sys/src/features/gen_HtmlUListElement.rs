use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLUListElement , typescript_type = "HTMLUListElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlUListElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlUListElement`*
    pub type HtmlUListElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLUListElement" , js_name = compact ) ]
    ///Getter for the `compact` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/compact)
    ///
    ///*This API requires the following crate features to be activated: `HtmlUListElement`*
    pub fn compact(this: &HtmlUListElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLUListElement" , js_name = compact ) ]
    ///Setter for the `compact` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/compact)
    ///
    ///*This API requires the following crate features to be activated: `HtmlUListElement`*
    pub fn set_compact(this: &HtmlUListElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLUListElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlUListElement`*
    pub fn type_(this: &HtmlUListElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLUListElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlUListElement`*
    pub fn set_type(this: &HtmlUListElement, value: &str);

}
