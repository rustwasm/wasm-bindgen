use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLHeadingElement , typescript_name = HTMLHeadingElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlHeadingElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlHeadingElement`*
    pub type HtmlHeadingElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLHeadingElement" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlHeadingElement`*
    pub fn align(this: &HtmlHeadingElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLHeadingElement" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlHeadingElement`*
    pub fn set_align(this: &HtmlHeadingElement, value: &str);

}
