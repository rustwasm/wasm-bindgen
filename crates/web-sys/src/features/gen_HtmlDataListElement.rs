use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLDataListElement , typescript_name = HTMLDataListElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDataListElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataListElement)\n\n*This API requires the following crate features to be activated: `HtmlDataListElement`*"]
    pub type HtmlDataListElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = options ) ]
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `options` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataListElement/options)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlDataListElement`*"]
    pub fn options(this: &HtmlDataListElement) -> HtmlCollection;
}
