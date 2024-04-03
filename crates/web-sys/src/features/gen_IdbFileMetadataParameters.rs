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
    #[wasm_bindgen(method, setter = "lastModified")]
    fn last_modified_shim(this: &IdbFileMetadataParameters, val: bool);
    #[wasm_bindgen(method, setter = "size")]
    fn size_shim(this: &IdbFileMetadataParameters, val: bool);
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
        self.last_modified_shim(val);
        self
    }
    #[doc = "Change the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    pub fn size(&mut self, val: bool) -> &mut Self {
        self.size_shim(val);
        self
    }
}
impl Default for IdbFileMetadataParameters {
    fn default() -> Self {
        Self::new()
    }
}
