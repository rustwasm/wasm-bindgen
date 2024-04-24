#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PushSubscriptionInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    pub type PushSubscriptionInit;
    #[wasm_bindgen(method, getter = "appServerKey")]
    fn app_server_key_shim(this: &PushSubscriptionInit) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, setter = "appServerKey")]
    fn set_app_server_key_shim(this: &PushSubscriptionInit, val: Option<&::js_sys::Object>);
    #[wasm_bindgen(method, getter = "authSecret")]
    fn auth_secret_shim(this: &PushSubscriptionInit) -> Option<::js_sys::ArrayBuffer>;
    #[wasm_bindgen(method, setter = "authSecret")]
    fn set_auth_secret_shim(this: &PushSubscriptionInit, val: Option<&::js_sys::ArrayBuffer>);
    #[wasm_bindgen(method, getter = "endpoint")]
    fn endpoint_shim(this: &PushSubscriptionInit) -> String;
    #[wasm_bindgen(method, setter = "endpoint")]
    fn set_endpoint_shim(this: &PushSubscriptionInit, val: &str);
    #[wasm_bindgen(method, getter = "p256dhKey")]
    fn p256dh_key_shim(this: &PushSubscriptionInit) -> Option<::js_sys::ArrayBuffer>;
    #[wasm_bindgen(method, setter = "p256dhKey")]
    fn set_p256dh_key_shim(this: &PushSubscriptionInit, val: Option<&::js_sys::ArrayBuffer>);
    #[wasm_bindgen(method, getter = "scope")]
    fn scope_shim(this: &PushSubscriptionInit) -> String;
    #[wasm_bindgen(method, setter = "scope")]
    fn set_scope_shim(this: &PushSubscriptionInit, val: &str);
}
#[doc = "The trait to access properties on the `PushSubscriptionInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
pub trait PushSubscriptionInitGetters {
    #[doc = "Get the `appServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    fn app_server_key(&self) -> Option<::js_sys::Object>;
    #[doc = "Get the `authSecret` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    fn auth_secret(&self) -> Option<::js_sys::ArrayBuffer>;
    #[doc = "Get the `endpoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    fn endpoint(&self) -> String;
    #[doc = "Get the `p256dhKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    fn p256dh_key(&self) -> Option<::js_sys::ArrayBuffer>;
    #[doc = "Get the `scope` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    fn scope(&self) -> String;
}
impl PushSubscriptionInitGetters for PushSubscriptionInit {
    fn app_server_key(&self) -> Option<::js_sys::Object> {
        self.app_server_key_shim()
    }
    fn auth_secret(&self) -> Option<::js_sys::ArrayBuffer> {
        self.auth_secret_shim()
    }
    fn endpoint(&self) -> String {
        self.endpoint_shim()
    }
    fn p256dh_key(&self) -> Option<::js_sys::ArrayBuffer> {
        self.p256dh_key_shim()
    }
    fn scope(&self) -> String {
        self.scope_shim()
    }
}
impl PushSubscriptionInit {
    #[doc = "Construct a new `PushSubscriptionInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    pub fn new(endpoint: &str, scope: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.endpoint(endpoint);
        ret.scope(scope);
        ret
    }
    #[doc = "Change the `appServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    pub fn app_server_key(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_app_server_key_shim(val);
        self
    }
    #[doc = "Change the `authSecret` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    pub fn auth_secret(&mut self, val: Option<&::js_sys::ArrayBuffer>) -> &mut Self {
        self.set_auth_secret_shim(val);
        self
    }
    #[doc = "Change the `endpoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    pub fn endpoint(&mut self, val: &str) -> &mut Self {
        self.set_endpoint_shim(val);
        self
    }
    #[doc = "Change the `p256dhKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    pub fn p256dh_key(&mut self, val: Option<&::js_sys::ArrayBuffer>) -> &mut Self {
        self.set_p256dh_key_shim(val);
        self
    }
    #[doc = "Change the `scope` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    pub fn scope(&mut self, val: &str) -> &mut Self {
        self.set_scope_shim(val);
        self
    }
}
