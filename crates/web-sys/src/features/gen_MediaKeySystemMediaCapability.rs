#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaKeySystemMediaCapability)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeySystemMediaCapability` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
    pub type MediaKeySystemMediaCapability;
    #[wasm_bindgen(method, getter = "contentType")]
    fn content_type_shim(this: &MediaKeySystemMediaCapability) -> String;
    #[wasm_bindgen(method, setter = "contentType")]
    fn set_content_type_shim(this: &MediaKeySystemMediaCapability, val: &str);
    #[wasm_bindgen(method, getter = "robustness")]
    fn robustness_shim(this: &MediaKeySystemMediaCapability) -> String;
    #[wasm_bindgen(method, setter = "robustness")]
    fn set_robustness_shim(this: &MediaKeySystemMediaCapability, val: &str);
}
#[doc = "The trait to access properties on the `MediaKeySystemMediaCapability` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
pub trait MediaKeySystemMediaCapabilityGetters {
    #[doc = "Get the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
    fn content_type(&self) -> String;
    #[doc = "Get the `robustness` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
    fn robustness(&self) -> String;
}
impl MediaKeySystemMediaCapabilityGetters for MediaKeySystemMediaCapability {
    fn content_type(&self) -> String {
        self.content_type_shim()
    }
    fn robustness(&self) -> String {
        self.robustness_shim()
    }
}
impl MediaKeySystemMediaCapability {
    #[doc = "Construct a new `MediaKeySystemMediaCapability`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
    pub fn content_type(&mut self, val: &str) -> &mut Self {
        self.set_content_type_shim(val);
        self
    }
    #[doc = "Change the `robustness` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
    pub fn robustness(&mut self, val: &str) -> &mut Self {
        self.set_robustness_shim(val);
        self
    }
}
impl Default for MediaKeySystemMediaCapability {
    fn default() -> Self {
        Self::new()
    }
}
