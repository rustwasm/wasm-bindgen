#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MemoryMeasurement)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MemoryMeasurement` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryMeasurement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MemoryMeasurement;
    #[wasm_bindgen(method, setter = "breakdown")]
    fn breakdown_shim(this: &MemoryMeasurement, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "bytes")]
    fn bytes_shim(this: &MemoryMeasurement, val: f64);
}
#[cfg(web_sys_unstable_apis)]
impl MemoryMeasurement {
    #[doc = "Construct a new `MemoryMeasurement`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryMeasurement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `breakdown` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryMeasurement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn breakdown(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.breakdown_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryMeasurement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bytes(&mut self, val: f64) -> &mut Self {
        self.bytes_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for MemoryMeasurement {
    fn default() -> Self {
        Self::new()
    }
}
