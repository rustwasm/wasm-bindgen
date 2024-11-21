#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = UALowEntropyJSON)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UaLowEntropyJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UaLowEntropyJson;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `brands` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "brands")]
    pub fn get_brands(this: &UaLowEntropyJson) -> Option<::js_sys::Array>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `brands` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "brands")]
    pub fn set_brands(this: &UaLowEntropyJson, val: &::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `mobile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "mobile")]
    pub fn get_mobile(this: &UaLowEntropyJson) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mobile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "mobile")]
    pub fn set_mobile(this: &UaLowEntropyJson, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `platform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "platform")]
    pub fn get_platform(this: &UaLowEntropyJson) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `platform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "platform")]
    pub fn set_platform(this: &UaLowEntropyJson, val: &str);
}
#[cfg(web_sys_unstable_apis)]
impl UaLowEntropyJson {
    #[doc = "Construct a new `UaLowEntropyJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_brands()` instead."]
    pub fn brands(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_brands(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_mobile()` instead."]
    pub fn mobile(&mut self, val: bool) -> &mut Self {
        self.set_mobile(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_platform()` instead."]
    pub fn platform(&mut self, val: &str) -> &mut Self {
        self.set_platform(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for UaLowEntropyJson {
    fn default() -> Self {
        Self::new()
    }
}
