#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MemoryBreakdownEntry)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MemoryBreakdownEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MemoryBreakdownEntry;
    #[wasm_bindgen(method, getter = "attribution")]
    fn attribution_shim(this: &MemoryBreakdownEntry) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "attribution")]
    fn set_attribution_shim(this: &MemoryBreakdownEntry, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "bytes")]
    fn bytes_shim(this: &MemoryBreakdownEntry) -> f64;
    #[wasm_bindgen(method, setter = "bytes")]
    fn set_bytes_shim(this: &MemoryBreakdownEntry, val: f64);
    #[wasm_bindgen(method, getter = "types")]
    fn types_shim(this: &MemoryBreakdownEntry) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "types")]
    fn set_types_shim(this: &MemoryBreakdownEntry, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `MemoryBreakdownEntry` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"]
pub trait MemoryBreakdownEntryGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `attribution` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn attribution(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn bytes(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `types` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn types(&self) -> &::wasm_bindgen::JsValue;
}
#[cfg(web_sys_unstable_apis)]
impl MemoryBreakdownEntryGetters for MemoryBreakdownEntry {
    #[cfg(web_sys_unstable_apis)]
    fn attribution(&self) -> &::wasm_bindgen::JsValue {
        self.attribution_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn bytes(&self) -> f64 {
        self.bytes_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn types(&self) -> &::wasm_bindgen::JsValue {
        self.types_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl MemoryBreakdownEntry {
    #[doc = "Construct a new `MemoryBreakdownEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `attribution` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn attribution(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_attribution_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bytes(&mut self, val: f64) -> &mut Self {
        self.set_bytes_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `types` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_types_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for MemoryBreakdownEntry {
    fn default() -> Self {
        Self::new()
    }
}
