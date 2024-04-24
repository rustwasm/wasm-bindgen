#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = XRPermissionDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrPermissionDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrPermissionDescriptor;
    #[cfg(feature = "PermissionName")]
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &XrPermissionDescriptor) -> PermissionName;
    #[cfg(feature = "PermissionName")]
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &XrPermissionDescriptor, val: PermissionName);
    #[cfg(feature = "XrSessionMode")]
    #[wasm_bindgen(method, getter = "mode")]
    fn mode_shim(this: &XrPermissionDescriptor) -> XrSessionMode;
    #[cfg(feature = "XrSessionMode")]
    #[wasm_bindgen(method, setter = "mode")]
    fn set_mode_shim(this: &XrPermissionDescriptor, val: XrSessionMode);
    #[wasm_bindgen(method, getter = "optionalFeatures")]
    fn optional_features_shim(this: &XrPermissionDescriptor) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "optionalFeatures")]
    fn set_optional_features_shim(this: &XrPermissionDescriptor, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "requiredFeatures")]
    fn required_features_shim(this: &XrPermissionDescriptor) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "requiredFeatures")]
    fn set_required_features_shim(this: &XrPermissionDescriptor, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `XrPermissionDescriptor` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `XrPermissionDescriptor`*"]
pub trait XrPermissionDescriptorGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PermissionName")]
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionName`, `XrPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn name(&self) -> PermissionName;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSessionMode")]
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrPermissionDescriptor`, `XrSessionMode`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn mode(&self) -> XrSessionMode;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `optionalFeatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn optional_features(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `requiredFeatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn required_features(&self) -> ::js_sys::Array;
}
#[cfg(web_sys_unstable_apis)]
impl XrPermissionDescriptorGetters for XrPermissionDescriptor {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PermissionName")]
    fn name(&self) -> PermissionName {
        self.name_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSessionMode")]
    fn mode(&self) -> XrSessionMode {
        self.mode_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn optional_features(&self) -> ::js_sys::Array {
        self.optional_features_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn required_features(&self) -> ::js_sys::Array {
        self.required_features_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl XrPermissionDescriptor {
    #[cfg(feature = "PermissionName")]
    #[doc = "Construct a new `XrPermissionDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionName`, `XrPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(name: PermissionName) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::name(&mut ret, name);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PermissionName")]
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionName`, `XrPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn name(&mut self, val: PermissionName) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSessionMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrPermissionDescriptor`, `XrSessionMode`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn mode(&mut self, val: XrSessionMode) -> &mut Self {
        self.set_mode_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `optionalFeatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn optional_features(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_optional_features_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `requiredFeatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn required_features(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_required_features_shim(val);
        self
    }
}
