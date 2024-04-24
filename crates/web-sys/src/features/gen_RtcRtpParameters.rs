#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtpParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
    pub type RtcRtpParameters;
    #[wasm_bindgen(method, getter = "codecs")]
    fn codecs_shim(this: &RtcRtpParameters) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "codecs")]
    fn set_codecs_shim(this: &RtcRtpParameters, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "encodings")]
    fn encodings_shim(this: &RtcRtpParameters) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "encodings")]
    fn set_encodings_shim(this: &RtcRtpParameters, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "headerExtensions")]
    fn header_extensions_shim(this: &RtcRtpParameters) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "headerExtensions")]
    fn set_header_extensions_shim(this: &RtcRtpParameters, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "RtcRtcpParameters")]
    #[wasm_bindgen(method, getter = "rtcp")]
    fn rtcp_shim(this: &RtcRtpParameters) -> RtcRtcpParameters;
    #[cfg(feature = "RtcRtcpParameters")]
    #[wasm_bindgen(method, setter = "rtcp")]
    fn set_rtcp_shim(this: &RtcRtpParameters, val: &RtcRtcpParameters);
}
#[doc = "The trait to access properties on the `RtcRtpParameters` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
pub trait RtcRtpParametersGetters {
    #[doc = "Get the `codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
    fn codecs(&self) -> ::js_sys::Array;
    #[doc = "Get the `encodings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
    fn encodings(&self) -> ::js_sys::Array;
    #[doc = "Get the `headerExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
    fn header_extensions(&self) -> ::js_sys::Array;
    #[cfg(feature = "RtcRtcpParameters")]
    #[doc = "Get the `rtcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`, `RtcRtpParameters`*"]
    fn rtcp(&self) -> RtcRtcpParameters;
}
impl RtcRtpParametersGetters for RtcRtpParameters {
    fn codecs(&self) -> ::js_sys::Array {
        self.codecs_shim()
    }
    fn encodings(&self) -> ::js_sys::Array {
        self.encodings_shim()
    }
    fn header_extensions(&self) -> ::js_sys::Array {
        self.header_extensions_shim()
    }
    #[cfg(feature = "RtcRtcpParameters")]
    fn rtcp(&self) -> RtcRtcpParameters {
        self.rtcp_shim()
    }
}
impl RtcRtpParameters {
    #[doc = "Construct a new `RtcRtpParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
    pub fn codecs(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_codecs_shim(val);
        self
    }
    #[doc = "Change the `encodings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
    pub fn encodings(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_encodings_shim(val);
        self
    }
    #[doc = "Change the `headerExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
    pub fn header_extensions(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_header_extensions_shim(val);
        self
    }
    #[cfg(feature = "RtcRtcpParameters")]
    #[doc = "Change the `rtcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`, `RtcRtpParameters`*"]
    pub fn rtcp(&mut self, val: &RtcRtcpParameters) -> &mut Self {
        self.set_rtcp_shim(val);
        self
    }
}
impl Default for RtcRtpParameters {
    fn default() -> Self {
        Self::new()
    }
}
