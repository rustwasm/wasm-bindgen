#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = MapLikeIterator , typescript_type = "MapLikeIterator")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MapLikeIterator` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MapLikeIterator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MapLikeIterator`*"]
    pub type MapLikeIterator;
    #[cfg(feature = "MapLikeIteratorResult")]
    # [wasm_bindgen (catch , method , structural , js_class = "MapLikeIterator" , js_name = next)]
    #[doc = "The `next()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MapLikeIterator/next)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MapLikeIterator`, `MapLikeIteratorResult`*"]
    pub fn next(this: &MapLikeIterator) -> Result<MapLikeIteratorResult, JsValue>;
}
