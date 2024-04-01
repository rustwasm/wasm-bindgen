#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PushSubscriptionJSON)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
    pub type PushSubscriptionJson;
    #[wasm_bindgen(method, setter = "endpoint")]
    fn endpoint_shim(this: &PushSubscriptionJson, val: &str);
    #[cfg(feature = "PushSubscriptionKeys")]
    #[wasm_bindgen(method, setter = "keys")]
    fn keys_shim(this: &PushSubscriptionJson, val: &PushSubscriptionKeys);
}
impl PushSubscriptionJson {
    #[doc = "Construct a new `PushSubscriptionJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `endpoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
    pub fn endpoint(&mut self, val: &str) -> &mut Self {
        self.endpoint_shim(val);
        self
    }
    #[cfg(feature = "PushSubscriptionKeys")]
    #[doc = "Change the `keys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`, `PushSubscriptionKeys`*"]
    pub fn keys(&mut self, val: &PushSubscriptionKeys) -> &mut Self {
        self.keys_shim(val);
        self
    }
}
impl Default for PushSubscriptionJson {
    fn default() -> Self {
        Self::new()
    }
}
