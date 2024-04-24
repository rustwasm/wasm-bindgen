#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HttpConnInfo)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HttpConnInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    pub type HttpConnInfo;
    #[wasm_bindgen(method, getter = "protocolVersion")]
    fn protocol_version_shim(this: &HttpConnInfo) -> &str;
    #[wasm_bindgen(method, setter = "protocolVersion")]
    fn set_protocol_version_shim(this: &HttpConnInfo, val: &str);
    #[wasm_bindgen(method, getter = "rtt")]
    fn rtt_shim(this: &HttpConnInfo) -> u32;
    #[wasm_bindgen(method, setter = "rtt")]
    fn set_rtt_shim(this: &HttpConnInfo, val: u32);
    #[wasm_bindgen(method, getter = "ttl")]
    fn ttl_shim(this: &HttpConnInfo) -> u32;
    #[wasm_bindgen(method, setter = "ttl")]
    fn set_ttl_shim(this: &HttpConnInfo, val: u32);
}
#[doc = "The trait to access properties on the `HttpConnInfo` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
pub trait HttpConnInfoGetters {
    #[doc = "Get the `protocolVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    fn protocol_version(&self) -> &str;
    #[doc = "Get the `rtt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    fn rtt(&self) -> u32;
    #[doc = "Get the `ttl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    fn ttl(&self) -> u32;
}
impl HttpConnInfoGetters for HttpConnInfo {
    fn protocol_version(&self) -> &str {
        self.protocol_version_shim()
    }
    fn rtt(&self) -> u32 {
        self.rtt_shim()
    }
    fn ttl(&self) -> u32 {
        self.ttl_shim()
    }
}
impl HttpConnInfo {
    #[doc = "Construct a new `HttpConnInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `protocolVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    pub fn protocol_version(&mut self, val: &str) -> &mut Self {
        self.set_protocol_version_shim(val);
        self
    }
    #[doc = "Change the `rtt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    pub fn rtt(&mut self, val: u32) -> &mut Self {
        self.set_rtt_shim(val);
        self
    }
    #[doc = "Change the `ttl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    pub fn ttl(&mut self, val: u32) -> &mut Self {
        self.set_ttl_shim(val);
        self
    }
}
impl Default for HttpConnInfo {
    fn default() -> Self {
        Self::new()
    }
}
