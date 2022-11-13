#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = MapLikeIteratorResult , typescript_type = "MapLikeIteratorResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MapLikeIteratorResult` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MapLikeIteratorResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MapLikeIteratorResult`*"]
    pub type MapLikeIteratorResult;
    # [wasm_bindgen (structural , method , getter , js_class = "MapLikeIteratorResult" , js_name = value)]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MapLikeIteratorResult/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MapLikeIteratorResult`*"]
    pub fn value(this: &MapLikeIteratorResult) -> ::wasm_bindgen::JsValue;
    # [wasm_bindgen (structural , method , setter , js_class = "MapLikeIteratorResult" , js_name = value)]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MapLikeIteratorResult/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MapLikeIteratorResult`*"]
    pub fn set_value(this: &MapLikeIteratorResult, value: &::wasm_bindgen::JsValue);
    # [wasm_bindgen (structural , method , getter , js_class = "MapLikeIteratorResult" , js_name = done)]
    #[doc = "Getter for the `done` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MapLikeIteratorResult/done)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MapLikeIteratorResult`*"]
    pub fn done(this: &MapLikeIteratorResult) -> bool;
    # [wasm_bindgen (structural , method , setter , js_class = "MapLikeIteratorResult" , js_name = done)]
    #[doc = "Setter for the `done` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MapLikeIteratorResult/done)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MapLikeIteratorResult`*"]
    pub fn set_done(this: &MapLikeIteratorResult, value: bool);
}
