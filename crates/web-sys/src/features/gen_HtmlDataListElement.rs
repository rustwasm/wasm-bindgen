use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLDataListElement , typescript_type = "HTMLDataListElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlDataListElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataListElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlDataListElement`*
    pub type HtmlDataListElement;

    #[cfg(feature = "HtmlCollection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLDataListElement" , js_name = options ) ]
    ///Getter for the `options` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataListElement/options)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlDataListElement`*
    pub fn options(this: &HtmlDataListElement) -> HtmlCollection;

}
