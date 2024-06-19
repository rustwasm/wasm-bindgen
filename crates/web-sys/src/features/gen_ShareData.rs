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
    #[doc = "Get the `files` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    #[wasm_bindgen(method, getter = "files")]
    pub fn get_files(this: &ShareData) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "files")]
    fn set_files(this: &ShareData, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `text` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    #[wasm_bindgen(method, getter = "text")]
    pub fn get_text(this: &ShareData) -> Option<String>;
    #[wasm_bindgen(method, setter = "text")]
    fn set_text(this: &ShareData, val: &str);
    #[doc = "Get the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &ShareData) -> Option<String>;
    #[wasm_bindgen(method, setter = "title")]
    fn set_title(this: &ShareData, val: &str);
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &ShareData) -> Option<String>;
    #[wasm_bindgen(method, setter = "url")]
    fn set_url(this: &ShareData, val: &str);
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
        self.set_files(val);
        self
    }
    #[doc = "Change the `text` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    pub fn text(&mut self, val: &str) -> &mut Self {
        self.set_text(val);
        self
    }
    #[doc = "Change the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    pub fn title(&mut self, val: &str) -> &mut Self {
        self.set_title(val);
        self
    }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    pub fn url(&mut self, val: &str) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for ShareData {
    fn default() -> Self {
        Self::new()
    }
}
