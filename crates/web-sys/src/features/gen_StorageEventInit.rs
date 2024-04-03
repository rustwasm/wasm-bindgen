#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = StorageEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StorageEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub type StorageEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &StorageEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &StorageEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &StorageEventInit, val: bool);
    #[wasm_bindgen(method, setter = "key")]
    fn key_shim(this: &StorageEventInit, val: Option<&str>);
    #[wasm_bindgen(method, setter = "newValue")]
    fn new_value_shim(this: &StorageEventInit, val: Option<&str>);
    #[wasm_bindgen(method, setter = "oldValue")]
    fn old_value_shim(this: &StorageEventInit, val: Option<&str>);
    #[cfg(feature = "Storage")]
    #[wasm_bindgen(method, setter = "storageArea")]
    fn storage_area_shim(this: &StorageEventInit, val: Option<&Storage>);
    #[wasm_bindgen(method, setter = "url")]
    fn url_shim(this: &StorageEventInit, val: &str);
}
impl StorageEventInit {
    #[doc = "Construct a new `StorageEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub fn key(&mut self, val: Option<&str>) -> &mut Self {
        self.key_shim(val);
        self
    }
    #[doc = "Change the `newValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub fn new_value(&mut self, val: Option<&str>) -> &mut Self {
        self.new_value_shim(val);
        self
    }
    #[doc = "Change the `oldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub fn old_value(&mut self, val: Option<&str>) -> &mut Self {
        self.old_value_shim(val);
        self
    }
    #[cfg(feature = "Storage")]
    #[doc = "Change the `storageArea` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`, `StorageEventInit`*"]
    pub fn storage_area(&mut self, val: Option<&Storage>) -> &mut Self {
        self.storage_area_shim(val);
        self
    }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub fn url(&mut self, val: &str) -> &mut Self {
        self.url_shim(val);
        self
    }
}
impl Default for StorageEventInit {
    fn default() -> Self {
        Self::new()
    }
}
