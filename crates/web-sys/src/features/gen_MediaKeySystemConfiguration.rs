#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaKeySystemConfiguration)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeySystemConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub type MediaKeySystemConfiguration;
    #[wasm_bindgen(method, setter = "audioCapabilities")]
    fn audio_capabilities_shim(this: &MediaKeySystemConfiguration, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "MediaKeysRequirement")]
    #[wasm_bindgen(method, setter = "distinctiveIdentifier")]
    fn distinctive_identifier_shim(this: &MediaKeySystemConfiguration, val: MediaKeysRequirement);
    #[wasm_bindgen(method, setter = "initDataTypes")]
    fn init_data_types_shim(this: &MediaKeySystemConfiguration, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "label")]
    fn label_shim(this: &MediaKeySystemConfiguration, val: &str);
    #[cfg(feature = "MediaKeysRequirement")]
    #[wasm_bindgen(method, setter = "persistentState")]
    fn persistent_state_shim(this: &MediaKeySystemConfiguration, val: MediaKeysRequirement);
    #[wasm_bindgen(method, setter = "sessionTypes")]
    fn session_types_shim(this: &MediaKeySystemConfiguration, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "videoCapabilities")]
    fn video_capabilities_shim(this: &MediaKeySystemConfiguration, val: &::wasm_bindgen::JsValue);
}
impl MediaKeySystemConfiguration {
    #[doc = "Construct a new `MediaKeySystemConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `audioCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn audio_capabilities(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.audio_capabilities_shim(val);
        self
    }
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Change the `distinctiveIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
    pub fn distinctive_identifier(&mut self, val: MediaKeysRequirement) -> &mut Self {
        self.distinctive_identifier_shim(val);
        self
    }
    #[doc = "Change the `initDataTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn init_data_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.init_data_types_shim(val);
        self
    }
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.label_shim(val);
        self
    }
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Change the `persistentState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
    pub fn persistent_state(&mut self, val: MediaKeysRequirement) -> &mut Self {
        self.persistent_state_shim(val);
        self
    }
    #[doc = "Change the `sessionTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn session_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.session_types_shim(val);
        self
    }
    #[doc = "Change the `videoCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn video_capabilities(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.video_capabilities_shim(val);
        self
    }
}
impl Default for MediaKeySystemConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
