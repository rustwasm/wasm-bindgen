#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtxParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtxParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtxParameters`*"]
    pub type RtcRtxParameters;
    #[wasm_bindgen(method, getter = "ssrc")]
    fn ssrc_shim(this: &RtcRtxParameters) -> u32;
    #[wasm_bindgen(method, setter = "ssrc")]
    fn set_ssrc_shim(this: &RtcRtxParameters, val: u32);
}
#[doc = "The trait to access properties on the `RtcRtxParameters` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcRtxParameters`*"]
pub trait RtcRtxParametersGetters {
    #[doc = "Get the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtxParameters`*"]
    fn ssrc(&self) -> u32;
}
impl RtcRtxParametersGetters for RtcRtxParameters {
    fn ssrc(&self) -> u32 {
        self.ssrc_shim()
    }
}
impl RtcRtxParameters {
    #[doc = "Construct a new `RtcRtxParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtxParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtxParameters`*"]
    pub fn ssrc(&mut self, val: u32) -> &mut Self {
        self.set_ssrc_shim(val);
        self
    }
}
impl Default for RtcRtxParameters {
    fn default() -> Self {
        Self::new()
    }
}
