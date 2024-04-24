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
    #[wasm_bindgen(method, getter = "auth")]
    fn auth_shim(this: &PushSubscriptionKeys) -> String;
    #[wasm_bindgen(method, setter = "auth")]
    fn set_auth_shim(this: &PushSubscriptionKeys, val: &str);
    #[wasm_bindgen(method, getter = "p256dh")]
    fn p256dh_shim(this: &PushSubscriptionKeys) -> String;
    #[wasm_bindgen(method, setter = "p256dh")]
    fn set_p256dh_shim(this: &PushSubscriptionKeys, val: &str);
}
#[doc = "The trait to access properties on the `PushSubscriptionKeys` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
pub trait PushSubscriptionKeysGetters {
    #[doc = "Get the `auth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
    fn auth(&self) -> String;
    #[doc = "Get the `p256dh` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
    fn p256dh(&self) -> String;
}
impl PushSubscriptionKeysGetters for PushSubscriptionKeys {
    fn auth(&self) -> String {
        self.auth_shim()
    }
    fn p256dh(&self) -> String {
        self.p256dh_shim()
    }
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
        self.set_auth_shim(val);
        self
    }
    #[doc = "Change the `p256dh` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
    pub fn p256dh(&mut self, val: &str) -> &mut Self {
        self.set_p256dh_shim(val);
        self
    }
}
impl Default for PushSubscriptionKeys {
    fn default() -> Self {
        Self::new()
    }
}
