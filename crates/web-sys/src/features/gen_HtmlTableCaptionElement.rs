use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTableCaptionElement , typescript_name = HTMLTableCaptionElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlTableCaptionElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCaptionElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCaptionElement`*
    pub type HtmlTableCaptionElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableCaptionElement" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCaptionElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCaptionElement`*
    pub fn align(this: &HtmlTableCaptionElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableCaptionElement" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCaptionElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableCaptionElement`*
    pub fn set_align(this: &HtmlTableCaptionElement, value: &str);

}
