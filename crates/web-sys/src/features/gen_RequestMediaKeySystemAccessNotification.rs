#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RequestMediaKeySystemAccessNotification)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RequestMediaKeySystemAccessNotification` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestMediaKeySystemAccessNotification`*"]
    pub type RequestMediaKeySystemAccessNotification;
    #[wasm_bindgen(method, setter = "keySystem")]
    fn key_system_shim(this: &RequestMediaKeySystemAccessNotification, val: &str);
    #[cfg(feature = "MediaKeySystemStatus")]
    #[wasm_bindgen(method, setter = "status")]
    fn status_shim(this: &RequestMediaKeySystemAccessNotification, val: MediaKeySystemStatus);
}
impl RequestMediaKeySystemAccessNotification {
    #[cfg(feature = "MediaKeySystemStatus")]
    #[doc = "Construct a new `RequestMediaKeySystemAccessNotification`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemStatus`, `RequestMediaKeySystemAccessNotification`*"]
    pub fn new(key_system: &str, status: MediaKeySystemStatus) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.key_system(key_system);
        ret.status(status);
        ret
    }
    #[doc = "Change the `keySystem` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestMediaKeySystemAccessNotification`*"]
    pub fn key_system(&mut self, val: &str) -> &mut Self {
        self.key_system_shim(val);
        self
    }
    #[cfg(feature = "MediaKeySystemStatus")]
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemStatus`, `RequestMediaKeySystemAccessNotification`*"]
    pub fn status(&mut self, val: MediaKeySystemStatus) -> &mut Self {
        self.status_shim(val);
        self
    }
}
