#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DocumentTimelineOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DocumentTimelineOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentTimelineOptions`*"]
    pub type DocumentTimelineOptions;
    #[wasm_bindgen(method, setter = "originTime")]
    fn origin_time_shim(this: &DocumentTimelineOptions, val: f64);
}
impl DocumentTimelineOptions {
    #[doc = "Construct a new `DocumentTimelineOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentTimelineOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `originTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentTimelineOptions`*"]
    pub fn origin_time(&mut self, val: f64) -> &mut Self {
        self.origin_time_shim(val);
        self
    }
}
impl Default for DocumentTimelineOptions {
    fn default() -> Self {
        Self::new()
    }
}
