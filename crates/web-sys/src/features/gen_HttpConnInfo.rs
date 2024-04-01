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
    #[wasm_bindgen(method, setter = "protocolVersion")]
    fn protocol_version_shim(this: &HttpConnInfo, val: &str);
    #[wasm_bindgen(method, setter = "rtt")]
    fn rtt_shim(this: &HttpConnInfo, val: u32);
    #[wasm_bindgen(method, setter = "ttl")]
    fn ttl_shim(this: &HttpConnInfo, val: u32);
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
        self.protocol_version_shim(val);
        self
    }
    #[doc = "Change the `rtt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    pub fn rtt(&mut self, val: u32) -> &mut Self {
        self.rtt_shim(val);
        self
    }
    #[doc = "Change the `ttl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    pub fn ttl(&mut self, val: u32) -> &mut Self {
        self.ttl_shim(val);
        self
    }
}
impl Default for HttpConnInfo {
    fn default() -> Self {
        Self::new()
    }
}
