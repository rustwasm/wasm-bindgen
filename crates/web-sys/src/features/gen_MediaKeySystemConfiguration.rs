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
    #[wasm_bindgen(method, getter = "audioCapabilities")]
    fn audio_capabilities_shim(this: &MediaKeySystemConfiguration) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "audioCapabilities")]
    fn set_audio_capabilities_shim(
        this: &MediaKeySystemConfiguration,
        val: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "MediaKeysRequirement")]
    #[wasm_bindgen(method, getter = "distinctiveIdentifier")]
    fn distinctive_identifier_shim(this: &MediaKeySystemConfiguration) -> MediaKeysRequirement;
    #[cfg(feature = "MediaKeysRequirement")]
    #[wasm_bindgen(method, setter = "distinctiveIdentifier")]
    fn set_distinctive_identifier_shim(
        this: &MediaKeySystemConfiguration,
        val: MediaKeysRequirement,
    );
    #[wasm_bindgen(method, getter = "initDataTypes")]
    fn init_data_types_shim(this: &MediaKeySystemConfiguration) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "initDataTypes")]
    fn set_init_data_types_shim(this: &MediaKeySystemConfiguration, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "label")]
    fn label_shim(this: &MediaKeySystemConfiguration) -> &str;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label_shim(this: &MediaKeySystemConfiguration, val: &str);
    #[cfg(feature = "MediaKeysRequirement")]
    #[wasm_bindgen(method, getter = "persistentState")]
    fn persistent_state_shim(this: &MediaKeySystemConfiguration) -> MediaKeysRequirement;
    #[cfg(feature = "MediaKeysRequirement")]
    #[wasm_bindgen(method, setter = "persistentState")]
    fn set_persistent_state_shim(this: &MediaKeySystemConfiguration, val: MediaKeysRequirement);
    #[wasm_bindgen(method, getter = "sessionTypes")]
    fn session_types_shim(this: &MediaKeySystemConfiguration) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "sessionTypes")]
    fn set_session_types_shim(this: &MediaKeySystemConfiguration, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "videoCapabilities")]
    fn video_capabilities_shim(this: &MediaKeySystemConfiguration) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "videoCapabilities")]
    fn set_video_capabilities_shim(
        this: &MediaKeySystemConfiguration,
        val: &::wasm_bindgen::JsValue,
    );
}
#[doc = "The trait to access properties on the `MediaKeySystemConfiguration` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
pub trait MediaKeySystemConfigurationGetters {
    #[doc = "Get the `audioCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    fn audio_capabilities(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Get the `distinctiveIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
    fn distinctive_identifier(&self) -> MediaKeysRequirement;
    #[doc = "Get the `initDataTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    fn init_data_types(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    fn label(&self) -> &str;
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Get the `persistentState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
    fn persistent_state(&self) -> MediaKeysRequirement;
    #[doc = "Get the `sessionTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    fn session_types(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `videoCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    fn video_capabilities(&self) -> &::wasm_bindgen::JsValue;
}
impl MediaKeySystemConfigurationGetters for MediaKeySystemConfiguration {
    fn audio_capabilities(&self) -> &::wasm_bindgen::JsValue {
        self.audio_capabilities_shim()
    }
    #[cfg(feature = "MediaKeysRequirement")]
    fn distinctive_identifier(&self) -> MediaKeysRequirement {
        self.distinctive_identifier_shim()
    }
    fn init_data_types(&self) -> &::wasm_bindgen::JsValue {
        self.init_data_types_shim()
    }
    fn label(&self) -> &str {
        self.label_shim()
    }
    #[cfg(feature = "MediaKeysRequirement")]
    fn persistent_state(&self) -> MediaKeysRequirement {
        self.persistent_state_shim()
    }
    fn session_types(&self) -> &::wasm_bindgen::JsValue {
        self.session_types_shim()
    }
    fn video_capabilities(&self) -> &::wasm_bindgen::JsValue {
        self.video_capabilities_shim()
    }
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
        self.set_audio_capabilities_shim(val);
        self
    }
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Change the `distinctiveIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
    pub fn distinctive_identifier(&mut self, val: MediaKeysRequirement) -> &mut Self {
        self.set_distinctive_identifier_shim(val);
        self
    }
    #[doc = "Change the `initDataTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn init_data_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_init_data_types_shim(val);
        self
    }
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label_shim(val);
        self
    }
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Change the `persistentState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
    pub fn persistent_state(&mut self, val: MediaKeysRequirement) -> &mut Self {
        self.set_persistent_state_shim(val);
        self
    }
    #[doc = "Change the `sessionTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn session_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_session_types_shim(val);
        self
    }
    #[doc = "Change the `videoCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn video_capabilities(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_video_capabilities_shim(val);
        self
    }
}
impl Default for MediaKeySystemConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
