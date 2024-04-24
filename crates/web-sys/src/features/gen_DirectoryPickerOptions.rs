#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DirectoryPickerOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DirectoryPickerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type DirectoryPickerOptions;
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &DirectoryPickerOptions) -> &str;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &DirectoryPickerOptions, val: &str);
    #[cfg(feature = "FileSystemPermissionMode")]
    #[wasm_bindgen(method, getter = "mode")]
    fn mode_shim(this: &DirectoryPickerOptions) -> FileSystemPermissionMode;
    #[cfg(feature = "FileSystemPermissionMode")]
    #[wasm_bindgen(method, setter = "mode")]
    fn set_mode_shim(this: &DirectoryPickerOptions, val: FileSystemPermissionMode);
    #[wasm_bindgen(method, getter = "startIn")]
    fn start_in_shim(this: &DirectoryPickerOptions) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "startIn")]
    fn set_start_in_shim(this: &DirectoryPickerOptions, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `DirectoryPickerOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`*"]
pub trait DirectoryPickerOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn id(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemPermissionMode")]
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`, `FileSystemPermissionMode`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn mode(&self) -> FileSystemPermissionMode;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `startIn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn start_in(&self) -> &::wasm_bindgen::JsValue;
}
#[cfg(web_sys_unstable_apis)]
impl DirectoryPickerOptionsGetters for DirectoryPickerOptions {
    #[cfg(web_sys_unstable_apis)]
    fn id(&self) -> &str {
        self.id_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemPermissionMode")]
    fn mode(&self) -> FileSystemPermissionMode {
        self.mode_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn start_in(&self) -> &::wasm_bindgen::JsValue {
        self.start_in_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl DirectoryPickerOptions {
    #[doc = "Construct a new `DirectoryPickerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemPermissionMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`, `FileSystemPermissionMode`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn mode(&mut self, val: FileSystemPermissionMode) -> &mut Self {
        self.set_mode_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `startIn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn start_in(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_start_in_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for DirectoryPickerOptions {
    fn default() -> Self {
        Self::new()
    }
}
