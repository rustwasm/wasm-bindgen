#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtcpParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtcpParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
    pub type RtcRtcpParameters;
    #[wasm_bindgen(method, getter = "cname")]
    fn cname_shim(this: &RtcRtcpParameters) -> &str;
    #[wasm_bindgen(method, setter = "cname")]
    fn set_cname_shim(this: &RtcRtcpParameters, val: &str);
    #[wasm_bindgen(method, getter = "reducedSize")]
    fn reduced_size_shim(this: &RtcRtcpParameters) -> bool;
    #[wasm_bindgen(method, setter = "reducedSize")]
    fn set_reduced_size_shim(this: &RtcRtcpParameters, val: bool);
}
#[doc = "The trait to access properties on the `RtcRtcpParameters` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
pub trait RtcRtcpParametersGetters {
    #[doc = "Get the `cname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
    fn cname(&self) -> &str;
    #[doc = "Get the `reducedSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
    fn reduced_size(&self) -> bool;
}
impl RtcRtcpParametersGetters for RtcRtcpParameters {
    fn cname(&self) -> &str {
        self.cname_shim()
    }
    fn reduced_size(&self) -> bool {
        self.reduced_size_shim()
    }
}
impl RtcRtcpParameters {
    #[doc = "Construct a new `RtcRtcpParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `cname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
    pub fn cname(&mut self, val: &str) -> &mut Self {
        self.set_cname_shim(val);
        self
    }
    #[doc = "Change the `reducedSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
    pub fn reduced_size(&mut self, val: bool) -> &mut Self {
        self.set_reduced_size_shim(val);
        self
    }
}
impl Default for RtcRtcpParameters {
    fn default() -> Self {
        Self::new()
    }
}
