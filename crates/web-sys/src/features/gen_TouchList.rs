use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = TouchList , typescript_name = TouchList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TouchList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchList)\n\n*This API requires the following crate features to be activated: `TouchList`*"]
    pub type TouchList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "TouchList" , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchList/length)\n\n*This API requires the following crate features to be activated: `TouchList`*"]
    pub fn length(this: &TouchList) -> u32;
    #[cfg(feature = "Touch")]
    # [ wasm_bindgen ( method , structural , js_class = "TouchList" , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchList/item)\n\n*This API requires the following crate features to be activated: `Touch`, `TouchList`*"]
    pub fn item(this: &TouchList, index: u32) -> Option<Touch>;
    #[cfg(feature = "Touch")]
    #[wasm_bindgen(method, structural, js_class = "TouchList", indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `Touch`, `TouchList`*"]
    pub fn get(this: &TouchList, index: u32) -> Option<Touch>;
}
