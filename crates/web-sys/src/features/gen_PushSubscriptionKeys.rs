#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PushSubscriptionKeys)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionKeys` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
    pub type PushSubscriptionKeys;
    #[wasm_bindgen(method, setter = "auth")]
    fn auth_shim(this: &PushSubscriptionKeys, val: &str);
    #[wasm_bindgen(method, setter = "p256dh")]
    fn p256dh_shim(this: &PushSubscriptionKeys, val: &str);
}
impl PushSubscriptionKeys {
    #[doc = "Construct a new `PushSubscriptionKeys`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `auth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
    pub fn auth(&mut self, val: &str) -> &mut Self {
        self.auth_shim(val);
        self
    }
    #[doc = "Change the `p256dh` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
    pub fn p256dh(&mut self, val: &str) -> &mut Self {
        self.p256dh_shim(val);
        self
    }
}
impl Default for PushSubscriptionKeys {
    fn default() -> Self {
        Self::new()
    }
}
