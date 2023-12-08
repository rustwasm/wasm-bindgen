#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtpHeaderExtensionCapability)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpHeaderExtensionCapability` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"]
    pub type RtcRtpHeaderExtensionCapability;
}
impl RtcRtpHeaderExtensionCapability {
    #[doc = "Construct a new `RtcRtpHeaderExtensionCapability`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"]
    pub fn new(uri: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.uri(uri);
        ret
    }
    #[doc = "Change the `uri` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"]
    pub fn uri(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("uri"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
