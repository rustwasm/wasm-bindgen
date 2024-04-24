#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCFecParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcFecParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
    pub type RtcFecParameters;
    #[wasm_bindgen(method, getter = "ssrc")]
    fn ssrc_shim(this: &RtcFecParameters) -> u32;
    #[wasm_bindgen(method, setter = "ssrc")]
    fn set_ssrc_shim(this: &RtcFecParameters, val: u32);
}
#[doc = "The trait to access properties on the `RtcFecParameters` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
pub trait RtcFecParametersGetters {
    #[doc = "Get the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
    fn ssrc(&self) -> u32;
}
impl RtcFecParametersGetters for RtcFecParameters {
    fn ssrc(&self) -> u32 {
        self.ssrc_shim()
    }
}
impl RtcFecParameters {
    #[doc = "Construct a new `RtcFecParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
    pub fn ssrc(&mut self, val: u32) -> &mut Self {
        self.set_ssrc_shim(val);
        self
    }
}
impl Default for RtcFecParameters {
    fn default() -> Self {
        Self::new()
    }
}
