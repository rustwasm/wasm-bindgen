#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EncodedVideoChunkInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EncodedVideoChunkInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type EncodedVideoChunkInit;
    #[wasm_bindgen(method, getter = "data")]
    fn data_shim(this: &EncodedVideoChunkInit) -> &::js_sys::Object;
    #[wasm_bindgen(method, setter = "data")]
    fn set_data_shim(this: &EncodedVideoChunkInit, val: &::js_sys::Object);
    #[wasm_bindgen(method, getter = "duration")]
    fn duration_shim(this: &EncodedVideoChunkInit) -> f64;
    #[wasm_bindgen(method, setter = "duration")]
    fn set_duration_shim(this: &EncodedVideoChunkInit, val: f64);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &EncodedVideoChunkInit) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &EncodedVideoChunkInit, val: f64);
    #[cfg(feature = "EncodedVideoChunkType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &EncodedVideoChunkInit) -> EncodedVideoChunkType;
    #[cfg(feature = "EncodedVideoChunkType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &EncodedVideoChunkInit, val: EncodedVideoChunkType);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `EncodedVideoChunkInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkInit`*"]
pub trait EncodedVideoChunkInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn data(&self) -> &::js_sys::Object;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn duration(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn timestamp(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedVideoChunkType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkInit`, `EncodedVideoChunkType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn type_(&self) -> EncodedVideoChunkType;
}
#[cfg(web_sys_unstable_apis)]
impl EncodedVideoChunkInitGetters for EncodedVideoChunkInit {
    #[cfg(web_sys_unstable_apis)]
    fn data(&self) -> &::js_sys::Object {
        self.data_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn duration(&self) -> f64 {
        self.duration_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedVideoChunkType")]
    fn type_(&self) -> EncodedVideoChunkType {
        self.type__shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl EncodedVideoChunkInit {
    #[cfg(feature = "EncodedVideoChunkType")]
    #[doc = "Construct a new `EncodedVideoChunkInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkInit`, `EncodedVideoChunkType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(data: &::js_sys::Object, timestamp: f64, type_: EncodedVideoChunkType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.data(data);
        ret.timestamp(timestamp);
        ret.type_(type_);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn data(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_data_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn duration(&mut self, val: f64) -> &mut Self {
        self.set_duration_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedVideoChunkType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkInit`, `EncodedVideoChunkType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn type_(&mut self, val: EncodedVideoChunkType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
