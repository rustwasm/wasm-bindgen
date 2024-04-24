#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtpCapabilities)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpCapabilities` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
    pub type RtcRtpCapabilities;
    #[wasm_bindgen(method, getter = "codecs")]
    fn codecs_shim(this: &RtcRtpCapabilities) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "codecs")]
    fn set_codecs_shim(this: &RtcRtpCapabilities, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "headerExtensions")]
    fn header_extensions_shim(this: &RtcRtpCapabilities) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "headerExtensions")]
    fn set_header_extensions_shim(this: &RtcRtpCapabilities, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `RtcRtpCapabilities` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
pub trait RtcRtpCapabilitiesGetters {
    #[doc = "Get the `codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
    fn codecs(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `headerExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
    fn header_extensions(&self) -> &::wasm_bindgen::JsValue;
}
impl RtcRtpCapabilitiesGetters for RtcRtpCapabilities {
    fn codecs(&self) -> &::wasm_bindgen::JsValue {
        self.codecs_shim()
    }
    fn header_extensions(&self) -> &::wasm_bindgen::JsValue {
        self.header_extensions_shim()
    }
}
impl RtcRtpCapabilities {
    #[doc = "Construct a new `RtcRtpCapabilities`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
    pub fn new(
        codecs: &::wasm_bindgen::JsValue,
        header_extensions: &::wasm_bindgen::JsValue,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.codecs(codecs);
        ret.header_extensions(header_extensions);
        ret
    }
    #[doc = "Change the `codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
    pub fn codecs(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_codecs_shim(val);
        self
    }
    #[doc = "Change the `headerExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
    pub fn header_extensions(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_header_extensions_shim(val);
        self
    }
}
