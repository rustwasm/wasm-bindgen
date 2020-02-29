use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLDivElement , typescript_name = HTMLDivElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlDivElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDivElement`*
    pub type HtmlDivElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLDivElement" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDivElement`*
    pub fn align(this: &HtmlDivElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLDivElement" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDivElement`*
    pub fn set_align(this: &HtmlDivElement, value: &str);

}
