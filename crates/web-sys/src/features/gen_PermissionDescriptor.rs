#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PermissionDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PermissionDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionDescriptor`*"]
    pub type PermissionDescriptor;
    #[cfg(feature = "PermissionName")]
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &PermissionDescriptor) -> PermissionName;
    #[cfg(feature = "PermissionName")]
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &PermissionDescriptor, val: PermissionName);
}
#[doc = "The trait to access properties on the `PermissionDescriptor` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PermissionDescriptor`*"]
pub trait PermissionDescriptorGetters {
    #[cfg(feature = "PermissionName")]
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionDescriptor`, `PermissionName`*"]
    fn name(&self) -> PermissionName;
}
impl PermissionDescriptorGetters for PermissionDescriptor {
    #[cfg(feature = "PermissionName")]
    fn name(&self) -> PermissionName {
        self.name_shim()
    }
}
impl PermissionDescriptor {
    #[cfg(feature = "PermissionName")]
    #[doc = "Construct a new `PermissionDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionDescriptor`, `PermissionName`*"]
    pub fn new(name: PermissionName) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::name(&mut ret, name);
        ret
    }
    #[cfg(feature = "PermissionName")]
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionDescriptor`, `PermissionName`*"]
    pub fn name(&mut self, val: PermissionName) -> &mut Self {
        self.set_name_shim(val);
        self
    }
}
