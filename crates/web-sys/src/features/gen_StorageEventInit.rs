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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &StorageEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &StorageEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &StorageEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &StorageEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &StorageEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &StorageEventInit, val: bool);
    #[wasm_bindgen(method, getter = "key")]
    fn key_shim(this: &StorageEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "key")]
    fn set_key_shim(this: &StorageEventInit, val: Option<&str>);
    #[wasm_bindgen(method, getter = "newValue")]
    fn new_value_shim(this: &StorageEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "newValue")]
    fn set_new_value_shim(this: &StorageEventInit, val: Option<&str>);
    #[wasm_bindgen(method, getter = "oldValue")]
    fn old_value_shim(this: &StorageEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "oldValue")]
    fn set_old_value_shim(this: &StorageEventInit, val: Option<&str>);
    #[cfg(feature = "Storage")]
    #[wasm_bindgen(method, getter = "storageArea")]
    fn storage_area_shim(this: &StorageEventInit) -> Option<Storage>;
    #[cfg(feature = "Storage")]
    #[wasm_bindgen(method, setter = "storageArea")]
    fn set_storage_area_shim(this: &StorageEventInit, val: Option<&Storage>);
    #[wasm_bindgen(method, getter = "url")]
    fn url_shim(this: &StorageEventInit) -> String;
    #[wasm_bindgen(method, setter = "url")]
    fn set_url_shim(this: &StorageEventInit, val: &str);
}
#[doc = "The trait to access properties on the `StorageEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
pub trait StorageEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    fn key(&self) -> Option<String>;
    #[doc = "Get the `newValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    fn new_value(&self) -> Option<String>;
    #[doc = "Get the `oldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    fn old_value(&self) -> Option<String>;
    #[cfg(feature = "Storage")]
    #[doc = "Get the `storageArea` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`, `StorageEventInit`*"]
    fn storage_area(&self) -> Option<Storage>;
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    fn url(&self) -> String;
}
impl StorageEventInitGetters for StorageEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn key(&self) -> Option<String> {
        self.key_shim()
    }
    fn new_value(&self) -> Option<String> {
        self.new_value_shim()
    }
    fn old_value(&self) -> Option<String> {
        self.old_value_shim()
    }
    #[cfg(feature = "Storage")]
    fn storage_area(&self) -> Option<Storage> {
        self.storage_area_shim()
    }
    fn url(&self) -> String {
        self.url_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub fn key(&mut self, val: Option<&str>) -> &mut Self {
        self.set_key_shim(val);
        self
    }
    #[doc = "Change the `newValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub fn new_value(&mut self, val: Option<&str>) -> &mut Self {
        self.set_new_value_shim(val);
        self
    }
    #[doc = "Change the `oldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub fn old_value(&mut self, val: Option<&str>) -> &mut Self {
        self.set_old_value_shim(val);
        self
    }
    #[cfg(feature = "Storage")]
    #[doc = "Change the `storageArea` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`, `StorageEventInit`*"]
    pub fn storage_area(&mut self, val: Option<&Storage>) -> &mut Self {
        self.set_storage_area_shim(val);
        self
    }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub fn url(&mut self, val: &str) -> &mut Self {
        self.set_url_shim(val);
        self
    }
}
impl Default for StorageEventInit {
    fn default() -> Self {
        Self::new()
    }
}
