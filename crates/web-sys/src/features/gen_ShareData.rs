#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ShareData)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ShareData` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    pub type ShareData;
    #[wasm_bindgen(method, setter = "files")]
    fn files_shim(this: &ShareData, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "text")]
    fn text_shim(this: &ShareData, val: &str);
    #[wasm_bindgen(method, setter = "title")]
    fn title_shim(this: &ShareData, val: &str);
    #[wasm_bindgen(method, setter = "url")]
    fn url_shim(this: &ShareData, val: &str);
}
impl ShareData {
    #[doc = "Construct a new `ShareData`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `files` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    pub fn files(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.files_shim(val);
        self
    }
    #[doc = "Change the `text` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    pub fn text(&mut self, val: &str) -> &mut Self {
        self.text_shim(val);
        self
    }
    #[doc = "Change the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    pub fn title(&mut self, val: &str) -> &mut Self {
        self.title_shim(val);
        self
    }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    pub fn url(&mut self, val: &str) -> &mut Self {
        self.url_shim(val);
        self
    }
}
impl Default for ShareData {
    fn default() -> Self {
        Self::new()
    }
}
