use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlCollection , extends = :: js_sys :: Object , js_name = HTMLFormControlsCollection , typescript_name = HTMLFormControlsCollection ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlFormControlsCollection` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormControlsCollection)\n\n*This API requires the following crate features to be activated: `HtmlFormControlsCollection`*"]
    pub type HtmlFormControlsCollection;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLFormControlsCollection" , js_name = namedItem ) ]
    #[doc = "The `namedItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormControlsCollection/namedItem)\n\n*This API requires the following crate features to be activated: `HtmlFormControlsCollection`*"]
    pub fn named_item(this: &HtmlFormControlsCollection, name: &str) -> Option<::js_sys::Object>;
    #[wasm_bindgen(
        method,
        structural,
        js_class = "HTMLFormControlsCollection",
        indexing_getter
    )]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `HtmlFormControlsCollection`*"]
    pub fn get(this: &HtmlFormControlsCollection, name: &str) -> Option<::js_sys::Object>;
}
