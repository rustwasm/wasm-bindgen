#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IDBFileMetadataParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbFileMetadataParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    pub type IdbFileMetadataParameters;
    #[wasm_bindgen(method, getter = "lastModified")]
    fn last_modified_shim(this: &IdbFileMetadataParameters) -> bool;
    #[wasm_bindgen(method, setter = "lastModified")]
    fn set_last_modified_shim(this: &IdbFileMetadataParameters, val: bool);
    #[wasm_bindgen(method, getter = "size")]
    fn size_shim(this: &IdbFileMetadataParameters) -> bool;
    #[wasm_bindgen(method, setter = "size")]
    fn set_size_shim(this: &IdbFileMetadataParameters, val: bool);
}
#[doc = "The trait to access properties on the `IdbFileMetadataParameters` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
pub trait IdbFileMetadataParametersGetters {
    #[doc = "Get the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    fn last_modified(&self) -> bool;
    #[doc = "Get the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    fn size(&self) -> bool;
}
impl IdbFileMetadataParametersGetters for IdbFileMetadataParameters {
    fn last_modified(&self) -> bool {
        self.last_modified_shim()
    }
    fn size(&self) -> bool {
        self.size_shim()
    }
}
impl IdbFileMetadataParameters {
    #[doc = "Construct a new `IdbFileMetadataParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    pub fn last_modified(&mut self, val: bool) -> &mut Self {
        self.set_last_modified_shim(val);
        self
    }
    #[doc = "Change the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    pub fn size(&mut self, val: bool) -> &mut Self {
        self.set_size_shim(val);
        self
    }
}
impl Default for IdbFileMetadataParameters {
    fn default() -> Self {
        Self::new()
    }
}
