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
    #[wasm_bindgen(method, getter = "body")]
    fn body_shim(this: &RequestInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "body")]
    fn set_body_shim(this: &RequestInit, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "RequestCache")]
    #[wasm_bindgen(method, getter = "cache")]
    fn cache_shim(this: &RequestInit) -> RequestCache;
    #[cfg(feature = "RequestCache")]
    #[wasm_bindgen(method, setter = "cache")]
    fn set_cache_shim(this: &RequestInit, val: RequestCache);
    #[cfg(feature = "RequestCredentials")]
    #[wasm_bindgen(method, getter = "credentials")]
    fn credentials_shim(this: &RequestInit) -> RequestCredentials;
    #[cfg(feature = "RequestCredentials")]
    #[wasm_bindgen(method, setter = "credentials")]
    fn set_credentials_shim(this: &RequestInit, val: RequestCredentials);
    #[wasm_bindgen(method, getter = "headers")]
    fn headers_shim(this: &RequestInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "headers")]
    fn set_headers_shim(this: &RequestInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "integrity")]
    fn integrity_shim(this: &RequestInit) -> &str;
    #[wasm_bindgen(method, setter = "integrity")]
    fn set_integrity_shim(this: &RequestInit, val: &str);
    #[wasm_bindgen(method, getter = "method")]
    fn method_shim(this: &RequestInit) -> &str;
    #[wasm_bindgen(method, setter = "method")]
    fn set_method_shim(this: &RequestInit, val: &str);
    #[cfg(feature = "RequestMode")]
    #[wasm_bindgen(method, getter = "mode")]
    fn mode_shim(this: &RequestInit) -> RequestMode;
    #[cfg(feature = "RequestMode")]
    #[wasm_bindgen(method, setter = "mode")]
    fn set_mode_shim(this: &RequestInit, val: RequestMode);
    #[cfg(feature = "ObserverCallback")]
    #[wasm_bindgen(method, getter = "observe")]
    fn observe_shim(this: &RequestInit) -> &ObserverCallback;
    #[cfg(feature = "ObserverCallback")]
    #[wasm_bindgen(method, setter = "observe")]
    fn set_observe_shim(this: &RequestInit, val: &ObserverCallback);
    #[cfg(feature = "RequestRedirect")]
    #[wasm_bindgen(method, getter = "redirect")]
    fn redirect_shim(this: &RequestInit) -> RequestRedirect;
    #[cfg(feature = "RequestRedirect")]
    #[wasm_bindgen(method, setter = "redirect")]
    fn set_redirect_shim(this: &RequestInit, val: RequestRedirect);
    #[wasm_bindgen(method, getter = "referrer")]
    fn referrer_shim(this: &RequestInit) -> &str;
    #[wasm_bindgen(method, setter = "referrer")]
    fn set_referrer_shim(this: &RequestInit, val: &str);
    #[cfg(feature = "ReferrerPolicy")]
    #[wasm_bindgen(method, getter = "referrerPolicy")]
    fn referrer_policy_shim(this: &RequestInit) -> ReferrerPolicy;
    #[cfg(feature = "ReferrerPolicy")]
    #[wasm_bindgen(method, setter = "referrerPolicy")]
    fn set_referrer_policy_shim(this: &RequestInit, val: ReferrerPolicy);
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(method, getter = "signal")]
    fn signal_shim(this: &RequestInit) -> Option<&AbortSignal>;
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(method, setter = "signal")]
    fn set_signal_shim(this: &RequestInit, val: Option<&AbortSignal>);
}
#[doc = "The trait to access properties on the `RequestInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
pub trait RequestInitGetters {
    #[doc = "Get the `body` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    fn body(&self) -> Option<&::wasm_bindgen::JsValue>;
    #[cfg(feature = "RequestCache")]
    #[doc = "Get the `cache` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCache`, `RequestInit`*"]
    fn cache(&self) -> RequestCache;
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Get the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `RequestInit`*"]
    fn credentials(&self) -> RequestCredentials;
    #[doc = "Get the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    fn headers(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `integrity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    fn integrity(&self) -> &str;
    #[doc = "Get the `method` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    fn method(&self) -> &str;
    #[cfg(feature = "RequestMode")]
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`, `RequestMode`*"]
    fn mode(&self) -> RequestMode;
    #[cfg(feature = "ObserverCallback")]
    #[doc = "Get the `observe` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ObserverCallback`, `RequestInit`*"]
    fn observe(&self) -> &ObserverCallback;
    #[cfg(feature = "RequestRedirect")]
    #[doc = "Get the `redirect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`, `RequestRedirect`*"]
    fn redirect(&self) -> RequestRedirect;
    #[doc = "Get the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    fn referrer(&self) -> &str;
    #[cfg(feature = "ReferrerPolicy")]
    #[doc = "Get the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReferrerPolicy`, `RequestInit`*"]
    fn referrer_policy(&self) -> ReferrerPolicy;
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `RequestInit`*"]
    fn signal(&self) -> Option<&AbortSignal>;
}
impl RequestInitGetters for RequestInit {
    fn body(&self) -> Option<&::wasm_bindgen::JsValue> {
        self.body_shim()
    }
    #[cfg(feature = "RequestCache")]
    fn cache(&self) -> RequestCache {
        self.cache_shim()
    }
    #[cfg(feature = "RequestCredentials")]
    fn credentials(&self) -> RequestCredentials {
        self.credentials_shim()
    }
    fn headers(&self) -> &::wasm_bindgen::JsValue {
        self.headers_shim()
    }
    fn integrity(&self) -> &str {
        self.integrity_shim()
    }
    fn method(&self) -> &str {
        self.method_shim()
    }
    #[cfg(feature = "RequestMode")]
    fn mode(&self) -> RequestMode {
        self.mode_shim()
    }
    #[cfg(feature = "ObserverCallback")]
    fn observe(&self) -> &ObserverCallback {
        self.observe_shim()
    }
    #[cfg(feature = "RequestRedirect")]
    fn redirect(&self) -> RequestRedirect {
        self.redirect_shim()
    }
    fn referrer(&self) -> &str {
        self.referrer_shim()
    }
    #[cfg(feature = "ReferrerPolicy")]
    fn referrer_policy(&self) -> ReferrerPolicy {
        self.referrer_policy_shim()
    }
    #[cfg(feature = "AbortSignal")]
    fn signal(&self) -> Option<&AbortSignal> {
        self.signal_shim()
    }
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
        self.set_body_shim(val.unwrap_or(&::wasm_bindgen::JsValue::NULL));
        self
    }
    #[cfg(feature = "RequestCache")]
    #[doc = "Change the `cache` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCache`, `RequestInit`*"]
    pub fn cache(&mut self, val: RequestCache) -> &mut Self {
        self.set_cache_shim(val);
        self
    }
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Change the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `RequestInit`*"]
    pub fn credentials(&mut self, val: RequestCredentials) -> &mut Self {
        self.set_credentials_shim(val);
        self
    }
    #[doc = "Change the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub fn headers(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_headers_shim(val);
        self
    }
    #[doc = "Change the `integrity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub fn integrity(&mut self, val: &str) -> &mut Self {
        self.set_integrity_shim(val);
        self
    }
    #[doc = "Change the `method` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub fn method(&mut self, val: &str) -> &mut Self {
        self.set_method_shim(val);
        self
    }
    #[cfg(feature = "RequestMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`, `RequestMode`*"]
    pub fn mode(&mut self, val: RequestMode) -> &mut Self {
        self.set_mode_shim(val);
        self
    }
    #[cfg(feature = "ObserverCallback")]
    #[doc = "Change the `observe` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ObserverCallback`, `RequestInit`*"]
    pub fn observe(&mut self, val: &ObserverCallback) -> &mut Self {
        self.set_observe_shim(val);
        self
    }
    #[cfg(feature = "RequestRedirect")]
    #[doc = "Change the `redirect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`, `RequestRedirect`*"]
    pub fn redirect(&mut self, val: RequestRedirect) -> &mut Self {
        self.set_redirect_shim(val);
        self
    }
    #[doc = "Change the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
    pub fn referrer(&mut self, val: &str) -> &mut Self {
        self.set_referrer_shim(val);
        self
    }
    #[cfg(feature = "ReferrerPolicy")]
    #[doc = "Change the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReferrerPolicy`, `RequestInit`*"]
    pub fn referrer_policy(&mut self, val: ReferrerPolicy) -> &mut Self {
        self.set_referrer_policy_shim(val);
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `RequestInit`*"]
    pub fn signal(&mut self, val: Option<&AbortSignal>) -> &mut Self {
        self.set_signal_shim(val);
        self
    }
}
impl Default for RequestInit {
    fn default() -> Self {
        Self::new()
    }
}
