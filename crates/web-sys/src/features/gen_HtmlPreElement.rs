use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLPreElement , typescript_name = HTMLPreElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlPreElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPreElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlPreElement`*
    pub type HtmlPreElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLPreElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPreElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlPreElement`*
    pub fn width(this: &HtmlPreElement) -> i32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLPreElement" , js_name = width ) ]
    ///Setter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPreElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlPreElement`*
    pub fn set_width(this: &HtmlPreElement, value: i32);

}
