#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PresentationConnectionCloseEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnectionCloseEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    pub type PresentationConnectionCloseEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &PresentationConnectionCloseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &PresentationConnectionCloseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &PresentationConnectionCloseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &PresentationConnectionCloseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &PresentationConnectionCloseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &PresentationConnectionCloseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "message")]
    fn message_shim(this: &PresentationConnectionCloseEventInit) -> String;
    #[wasm_bindgen(method, setter = "message")]
    fn set_message_shim(this: &PresentationConnectionCloseEventInit, val: &str);
    #[cfg(feature = "PresentationConnectionClosedReason")]
    #[wasm_bindgen(method, getter = "reason")]
    fn reason_shim(
        this: &PresentationConnectionCloseEventInit,
    ) -> PresentationConnectionClosedReason;
    #[cfg(feature = "PresentationConnectionClosedReason")]
    #[wasm_bindgen(method, setter = "reason")]
    fn set_reason_shim(
        this: &PresentationConnectionCloseEventInit,
        val: PresentationConnectionClosedReason,
    );
}
#[doc = "The trait to access properties on the `PresentationConnectionCloseEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
pub trait PresentationConnectionCloseEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    fn message(&self) -> String;
    #[cfg(feature = "PresentationConnectionClosedReason")]
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`, `PresentationConnectionClosedReason`*"]
    fn reason(&self) -> PresentationConnectionClosedReason;
}
impl PresentationConnectionCloseEventInitGetters for PresentationConnectionCloseEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn message(&self) -> String {
        self.message_shim()
    }
    #[cfg(feature = "PresentationConnectionClosedReason")]
    fn reason(&self) -> PresentationConnectionClosedReason {
        self.reason_shim()
    }
}
impl PresentationConnectionCloseEventInit {
    #[cfg(feature = "PresentationConnectionClosedReason")]
    #[doc = "Construct a new `PresentationConnectionCloseEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`, `PresentationConnectionClosedReason`*"]
    pub fn new(reason: PresentationConnectionClosedReason) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.reason(reason);
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    pub fn message(&mut self, val: &str) -> &mut Self {
        self.set_message_shim(val);
        self
    }
    #[cfg(feature = "PresentationConnectionClosedReason")]
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`, `PresentationConnectionClosedReason`*"]
    pub fn reason(&mut self, val: PresentationConnectionClosedReason) -> &mut Self {
        self.set_reason_shim(val);
        self
    }
}
