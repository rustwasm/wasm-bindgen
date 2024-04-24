#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ClipboardPermissionDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClipboardPermissionDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ClipboardPermissionDescriptor;
    #[cfg(feature = "PermissionName")]
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &ClipboardPermissionDescriptor) -> PermissionName;
    #[cfg(feature = "PermissionName")]
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &ClipboardPermissionDescriptor, val: PermissionName);
    #[wasm_bindgen(method, getter = "allowWithoutGesture")]
    fn allow_without_gesture_shim(this: &ClipboardPermissionDescriptor) -> bool;
    #[wasm_bindgen(method, setter = "allowWithoutGesture")]
    fn set_allow_without_gesture_shim(this: &ClipboardPermissionDescriptor, val: bool);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `ClipboardPermissionDescriptor` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ClipboardPermissionDescriptor`*"]
pub trait ClipboardPermissionDescriptorGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PermissionName")]
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardPermissionDescriptor`, `PermissionName`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn name(&self) -> PermissionName;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `allowWithoutGesture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn allow_without_gesture(&self) -> bool;
}
#[cfg(web_sys_unstable_apis)]
impl ClipboardPermissionDescriptorGetters for ClipboardPermissionDescriptor {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PermissionName")]
    fn name(&self) -> PermissionName {
        self.name_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn allow_without_gesture(&self) -> bool {
        self.allow_without_gesture_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl ClipboardPermissionDescriptor {
    #[cfg(feature = "PermissionName")]
    #[doc = "Construct a new `ClipboardPermissionDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardPermissionDescriptor`, `PermissionName`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `ClipboardPermissionDescriptor`, `PermissionName`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn name(&mut self, val: PermissionName) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `allowWithoutGesture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardPermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn allow_without_gesture(&mut self, val: bool) -> &mut Self {
        self.set_allow_without_gesture_shim(val);
        self
    }
}
