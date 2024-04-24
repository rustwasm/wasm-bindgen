#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FilePickerOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FilePickerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type FilePickerOptions;
    #[wasm_bindgen(method, getter = "excludeAcceptAllOption")]
    fn exclude_accept_all_option_shim(this: &FilePickerOptions) -> bool;
    #[wasm_bindgen(method, setter = "excludeAcceptAllOption")]
    fn set_exclude_accept_all_option_shim(this: &FilePickerOptions, val: bool);
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &FilePickerOptions) -> String;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &FilePickerOptions, val: &str);
    #[wasm_bindgen(method, getter = "startIn")]
    fn start_in_shim(this: &FilePickerOptions) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "startIn")]
    fn set_start_in_shim(this: &FilePickerOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "types")]
    fn types_shim(this: &FilePickerOptions) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "types")]
    fn set_types_shim(this: &FilePickerOptions, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `FilePickerOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FilePickerOptions`*"]
pub trait FilePickerOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `excludeAcceptAllOption` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn exclude_accept_all_option(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn id(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `startIn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn start_in(&self) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `types` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn types(&self) -> ::js_sys::Array;
}
#[cfg(web_sys_unstable_apis)]
impl FilePickerOptionsGetters for FilePickerOptions {
    #[cfg(web_sys_unstable_apis)]
    fn exclude_accept_all_option(&self) -> bool {
        self.exclude_accept_all_option_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn id(&self) -> String {
        self.id_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn start_in(&self) -> ::wasm_bindgen::JsValue {
        self.start_in_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn types(&self) -> ::js_sys::Array {
        self.types_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl FilePickerOptions {
    #[doc = "Construct a new `FilePickerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `excludeAcceptAllOption` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn exclude_accept_all_option(&mut self, val: bool) -> &mut Self {
        self.set_exclude_accept_all_option_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `startIn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn start_in(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_start_in_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `types` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_types_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for FilePickerOptions {
    fn default() -> Self {
        Self::new()
    }
}
