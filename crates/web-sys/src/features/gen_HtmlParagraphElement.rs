use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLParagraphElement , typescript_type = "HTMLParagraphElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlParagraphElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlParagraphElement`*
    pub type HtmlParagraphElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLParagraphElement" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlParagraphElement`*
    pub fn align(this: &HtmlParagraphElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLParagraphElement" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlParagraphElement`*
    pub fn set_align(this: &HtmlParagraphElement, value: &str);

}
