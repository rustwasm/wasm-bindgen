use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLDirectoryElement , typescript_type = "HTMLDirectoryElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlDirectoryElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDirectoryElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDirectoryElement`*
    pub type HtmlDirectoryElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLDirectoryElement" , js_name = compact ) ]
    ///Getter for the `compact` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDirectoryElement/compact)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDirectoryElement`*
    pub fn compact(this: &HtmlDirectoryElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLDirectoryElement" , js_name = compact ) ]
    ///Setter for the `compact` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDirectoryElement/compact)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDirectoryElement`*
    pub fn set_compact(this: &HtmlDirectoryElement, value: bool);

}
