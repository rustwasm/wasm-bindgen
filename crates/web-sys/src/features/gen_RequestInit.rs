#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RequestInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RequestInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub type RequestInit;
    #[wasm_bindgen(method, setter = "body")]
    fn body_shim(this: &RequestInit, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "RequestCache")]
    #[wasm_bindgen(method, setter = "cache")]
    fn cache_shim(this: &RequestInit, val: RequestCache);
    #[cfg(feature = "RequestCredentials")]
    #[wasm_bindgen(method, setter = "credentials")]
    fn credentials_shim(this: &RequestInit, val: RequestCredentials);
    #[wasm_bindgen(method, setter = "headers")]
    fn headers_shim(this: &RequestInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "integrity")]
    fn integrity_shim(this: &RequestInit, val: &str);
    #[wasm_bindgen(method, setter = "method")]
    fn method_shim(this: &RequestInit, val: &str);
    #[cfg(feature = "RequestMode")]
    #[wasm_bindgen(method, setter = "mode")]
    fn mode_shim(this: &RequestInit, val: RequestMode);
    #[cfg(feature = "ObserverCallback")]
    #[wasm_bindgen(method, setter = "observe")]
    fn observe_shim(this: &RequestInit, val: &ObserverCallback);
    #[cfg(feature = "RequestRedirect")]
    #[wasm_bindgen(method, setter = "redirect")]
    fn redirect_shim(this: &RequestInit, val: RequestRedirect);
    #[wasm_bindgen(method, setter = "referrer")]
    fn referrer_shim(this: &RequestInit, val: &str);
    #[cfg(feature = "ReferrerPolicy")]
    #[wasm_bindgen(method, setter = "referrerPolicy")]
    fn referrer_policy_shim(this: &RequestInit, val: ReferrerPolicy);
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(method, setter = "signal")]
    fn signal_shim(this: &RequestInit, val: Option<&AbortSignal>);
}
impl RequestInit {
    #[doc = "Construct a new `RequestInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `body` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub fn body(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        self.body_shim(val.unwrap_or(&::wasm_bindgen::JsValue::NULL));
        self
    }
    #[cfg(feature = "RequestCache")]
    #[doc = "Change the `cache` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCache`, `RequestInit`*"]
    pub fn cache(&mut self, val: RequestCache) -> &mut Self {
        self.cache_shim(val);
        self
    }
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Change the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `RequestInit`*"]
    pub fn credentials(&mut self, val: RequestCredentials) -> &mut Self {
        self.credentials_shim(val);
        self
    }
    #[doc = "Change the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub fn headers(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.headers_shim(val);
        self
    }
    #[doc = "Change the `integrity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub fn integrity(&mut self, val: &str) -> &mut Self {
        self.integrity_shim(val);
        self
    }
    #[doc = "Change the `method` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub fn method(&mut self, val: &str) -> &mut Self {
        self.method_shim(val);
        self
    }
    #[cfg(feature = "RequestMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`, `RequestMode`*"]
    pub fn mode(&mut self, val: RequestMode) -> &mut Self {
        self.mode_shim(val);
        self
    }
    #[cfg(feature = "ObserverCallback")]
    #[doc = "Change the `observe` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ObserverCallback`, `RequestInit`*"]
    pub fn observe(&mut self, val: &ObserverCallback) -> &mut Self {
        self.observe_shim(val);
        self
    }
    #[cfg(feature = "RequestRedirect")]
    #[doc = "Change the `redirect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`, `RequestRedirect`*"]
    pub fn redirect(&mut self, val: RequestRedirect) -> &mut Self {
        self.redirect_shim(val);
        self
    }
    #[doc = "Change the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub fn referrer(&mut self, val: &str) -> &mut Self {
        self.referrer_shim(val);
        self
    }
    #[cfg(feature = "ReferrerPolicy")]
    #[doc = "Change the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReferrerPolicy`, `RequestInit`*"]
    pub fn referrer_policy(&mut self, val: ReferrerPolicy) -> &mut Self {
        self.referrer_policy_shim(val);
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `RequestInit`*"]
    pub fn signal(&mut self, val: Option<&AbortSignal>) -> &mut Self {
        self.signal_shim(val);
        self
    }
}
impl Default for RequestInit {
    fn default() -> Self {
        Self::new()
    }
}
