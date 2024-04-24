#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaKeysPolicy)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeysPolicy` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeysPolicy`*"]
    pub type MediaKeysPolicy;
    #[wasm_bindgen(method, getter = "minHdcpVersion")]
    fn min_hdcp_version_shim(this: &MediaKeysPolicy) -> &str;
    #[wasm_bindgen(method, setter = "minHdcpVersion")]
    fn set_min_hdcp_version_shim(this: &MediaKeysPolicy, val: &str);
}
#[doc = "The trait to access properties on the `MediaKeysPolicy` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaKeysPolicy`*"]
pub trait MediaKeysPolicyGetters {
    #[doc = "Get the `minHdcpVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeysPolicy`*"]
    fn min_hdcp_version(&self) -> &str;
}
impl MediaKeysPolicyGetters for MediaKeysPolicy {
    fn min_hdcp_version(&self) -> &str {
        self.min_hdcp_version_shim()
    }
}
impl MediaKeysPolicy {
    #[doc = "Construct a new `MediaKeysPolicy`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeysPolicy`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `minHdcpVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeysPolicy`*"]
    pub fn min_hdcp_version(&mut self, val: &str) -> &mut Self {
        self.set_min_hdcp_version_shim(val);
        self
    }
}
impl Default for MediaKeysPolicy {
    fn default() -> Self {
        Self::new()
    }
}
