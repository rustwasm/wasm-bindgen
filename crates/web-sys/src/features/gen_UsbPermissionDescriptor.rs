#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = USBPermissionDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbPermissionDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbPermissionDescriptor;
    #[cfg(feature = "PermissionName")]
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &UsbPermissionDescriptor) -> PermissionName;
    #[cfg(feature = "PermissionName")]
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &UsbPermissionDescriptor, val: PermissionName);
    #[wasm_bindgen(method, getter = "filters")]
    fn filters_shim(this: &UsbPermissionDescriptor) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "filters")]
    fn set_filters_shim(this: &UsbPermissionDescriptor, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `UsbPermissionDescriptor` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `UsbPermissionDescriptor`*"]
pub trait UsbPermissionDescriptorGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PermissionName")]
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionName`, `UsbPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn name(&self) -> PermissionName;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn filters(&self) -> &::wasm_bindgen::JsValue;
}
#[cfg(web_sys_unstable_apis)]
impl UsbPermissionDescriptorGetters for UsbPermissionDescriptor {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PermissionName")]
    fn name(&self) -> PermissionName {
        self.name_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn filters(&self) -> &::wasm_bindgen::JsValue {
        self.filters_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl UsbPermissionDescriptor {
    #[cfg(feature = "PermissionName")]
    #[doc = "Construct a new `UsbPermissionDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionName`, `UsbPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(name: PermissionName) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PermissionName")]
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionName`, `UsbPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn name(&mut self, val: PermissionName) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn filters(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_filters_shim(val);
        self
    }
}
