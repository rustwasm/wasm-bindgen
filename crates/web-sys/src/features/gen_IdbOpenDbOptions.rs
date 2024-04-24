#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IDBOpenDBOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbOpenDbOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`*"]
    pub type IdbOpenDbOptions;
    #[cfg(feature = "StorageType")]
    #[wasm_bindgen(method, getter = "storage")]
    fn storage_shim(this: &IdbOpenDbOptions) -> StorageType;
    #[cfg(feature = "StorageType")]
    #[wasm_bindgen(method, setter = "storage")]
    fn set_storage_shim(this: &IdbOpenDbOptions, val: StorageType);
    #[wasm_bindgen(method, getter = "version")]
    fn version_shim(this: &IdbOpenDbOptions) -> f64;
    #[wasm_bindgen(method, setter = "version")]
    fn set_version_shim(this: &IdbOpenDbOptions, val: f64);
}
#[doc = "The trait to access properties on the `IdbOpenDbOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`*"]
pub trait IdbOpenDbOptionsGetters {
    #[cfg(feature = "StorageType")]
    #[doc = "Get the `storage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`, `StorageType`*"]
    fn storage(&self) -> StorageType;
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`*"]
    fn version(&self) -> f64;
}
impl IdbOpenDbOptionsGetters for IdbOpenDbOptions {
    #[cfg(feature = "StorageType")]
    fn storage(&self) -> StorageType {
        self.storage_shim()
    }
    fn version(&self) -> f64 {
        self.version_shim()
    }
}
impl IdbOpenDbOptions {
    #[doc = "Construct a new `IdbOpenDbOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "StorageType")]
    #[doc = "Change the `storage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`, `StorageType`*"]
    pub fn storage(&mut self, val: StorageType) -> &mut Self {
        self.set_storage_shim(val);
        self
    }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`*"]
    pub fn version(&mut self, val: f64) -> &mut Self {
        self.set_version_shim(val);
        self
    }
}
impl Default for IdbOpenDbOptions {
    fn default() -> Self {
        Self::new()
    }
}
