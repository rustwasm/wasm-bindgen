#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PushSubscriptionOptionsInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionOptionsInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    pub type PushSubscriptionOptionsInit;
    #[wasm_bindgen(method, getter = "applicationServerKey")]
    fn application_server_key_shim(this: &PushSubscriptionOptionsInit) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "applicationServerKey")]
    fn set_application_server_key_shim(
        this: &PushSubscriptionOptionsInit,
        val: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(method, getter = "userVisibleOnly")]
    fn user_visible_only_shim(this: &PushSubscriptionOptionsInit) -> bool;
    #[wasm_bindgen(method, setter = "userVisibleOnly")]
    fn set_user_visible_only_shim(this: &PushSubscriptionOptionsInit, val: bool);
}
#[doc = "The trait to access properties on the `PushSubscriptionOptionsInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
pub trait PushSubscriptionOptionsInitGetters {
    #[doc = "Get the `applicationServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    fn application_server_key(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `userVisibleOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    fn user_visible_only(&self) -> bool;
}
impl PushSubscriptionOptionsInitGetters for PushSubscriptionOptionsInit {
    fn application_server_key(&self) -> ::wasm_bindgen::JsValue {
        self.application_server_key_shim()
    }
    fn user_visible_only(&self) -> bool {
        self.user_visible_only_shim()
    }
}
impl PushSubscriptionOptionsInit {
    #[doc = "Construct a new `PushSubscriptionOptionsInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `applicationServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    pub fn application_server_key(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        self.set_application_server_key_shim(val.unwrap_or(&::wasm_bindgen::JsValue::NULL));
        self
    }
    #[doc = "Change the `userVisibleOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    pub fn user_visible_only(&mut self, val: bool) -> &mut Self {
        self.set_user_visible_only_shim(val);
        self
    }
}
impl Default for PushSubscriptionOptionsInit {
    fn default() -> Self {
        Self::new()
    }
}
