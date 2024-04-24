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
    #[wasm_bindgen(method, getter = "breakdown")]
    fn breakdown_shim(this: &MemoryMeasurement) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "breakdown")]
    fn set_breakdown_shim(this: &MemoryMeasurement, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "bytes")]
    fn bytes_shim(this: &MemoryMeasurement) -> f64;
    #[wasm_bindgen(method, setter = "bytes")]
    fn set_bytes_shim(this: &MemoryMeasurement, val: f64);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `MemoryMeasurement` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MemoryMeasurement`*"]
pub trait MemoryMeasurementGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `breakdown` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryMeasurement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn breakdown(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryMeasurement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn bytes(&self) -> f64;
}
#[cfg(web_sys_unstable_apis)]
impl MemoryMeasurementGetters for MemoryMeasurement {
    #[cfg(web_sys_unstable_apis)]
    fn breakdown(&self) -> ::js_sys::Array {
        self.breakdown_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn bytes(&self) -> f64 {
        self.bytes_shim()
    }
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
        self.set_breakdown_shim(val);
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
        self.set_bytes_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for MemoryMeasurement {
    fn default() -> Self {
        Self::new()
    }
}
