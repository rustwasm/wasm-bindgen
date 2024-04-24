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
    #[wasm_bindgen(method, getter = "files")]
    fn files_shim(this: &ShareData) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "files")]
    fn set_files_shim(this: &ShareData, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "text")]
    fn text_shim(this: &ShareData) -> &str;
    #[wasm_bindgen(method, setter = "text")]
    fn set_text_shim(this: &ShareData, val: &str);
    #[wasm_bindgen(method, getter = "title")]
    fn title_shim(this: &ShareData) -> &str;
    #[wasm_bindgen(method, setter = "title")]
    fn set_title_shim(this: &ShareData, val: &str);
    #[wasm_bindgen(method, getter = "url")]
    fn url_shim(this: &ShareData) -> &str;
    #[wasm_bindgen(method, setter = "url")]
    fn set_url_shim(this: &ShareData, val: &str);
}
#[doc = "The trait to access properties on the `ShareData` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
pub trait ShareDataGetters {
    #[doc = "Get the `files` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    fn files(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `text` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    fn text(&self) -> &str;
    #[doc = "Get the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    fn title(&self) -> &str;
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    fn url(&self) -> &str;
}
impl ShareDataGetters for ShareData {
    fn files(&self) -> &::wasm_bindgen::JsValue {
        self.files_shim()
    }
    fn text(&self) -> &str {
        self.text_shim()
    }
    fn title(&self) -> &str {
        self.title_shim()
    }
    fn url(&self) -> &str {
        self.url_shim()
    }
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
        self.set_files_shim(val);
        self
    }
    #[doc = "Change the `text` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    pub fn text(&mut self, val: &str) -> &mut Self {
        self.set_text_shim(val);
        self
    }
    #[doc = "Change the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    pub fn title(&mut self, val: &str) -> &mut Self {
        self.set_title_shim(val);
        self
    }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    pub fn url(&mut self, val: &str) -> &mut Self {
        self.set_url_shim(val);
        self
    }
}
impl Default for ShareData {
    fn default() -> Self {
        Self::new()
    }
}
