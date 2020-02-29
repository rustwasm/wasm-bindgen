use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLDListElement , typescript_name = HTMLDListElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlDListElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDListElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDListElement`*
    pub type HtmlDListElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLDListElement" , js_name = compact ) ]
    ///Getter for the `compact` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDListElement/compact)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDListElement`*
    pub fn compact(this: &HtmlDListElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLDListElement" , js_name = compact ) ]
    ///Setter for the `compact` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDListElement/compact)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDListElement`*
    pub fn set_compact(this: &HtmlDListElement, value: bool);

}
