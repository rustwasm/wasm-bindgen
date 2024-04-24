#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PresentationConnectionAvailableEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnectionAvailableEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
    pub type PresentationConnectionAvailableEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &PresentationConnectionAvailableEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &PresentationConnectionAvailableEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &PresentationConnectionAvailableEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &PresentationConnectionAvailableEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &PresentationConnectionAvailableEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &PresentationConnectionAvailableEventInit, val: bool);
    #[cfg(feature = "PresentationConnection")]
    #[wasm_bindgen(method, getter = "connection")]
    fn connection_shim(this: &PresentationConnectionAvailableEventInit) -> &PresentationConnection;
    #[cfg(feature = "PresentationConnection")]
    #[wasm_bindgen(method, setter = "connection")]
    fn set_connection_shim(
        this: &PresentationConnectionAvailableEventInit,
        val: &PresentationConnection,
    );
}
#[doc = "The trait to access properties on the `PresentationConnectionAvailableEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
pub trait PresentationConnectionAvailableEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "PresentationConnection")]
    #[doc = "Get the `connection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionAvailableEventInit`*"]
    fn connection(&self) -> &PresentationConnection;
}
impl PresentationConnectionAvailableEventInitGetters for PresentationConnectionAvailableEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "PresentationConnection")]
    fn connection(&self) -> &PresentationConnection {
        self.connection_shim()
    }
}
impl PresentationConnectionAvailableEventInit {
    #[cfg(feature = "PresentationConnection")]
    #[doc = "Construct a new `PresentationConnectionAvailableEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionAvailableEventInit`*"]
    pub fn new(connection: &PresentationConnection) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.connection(connection);
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "PresentationConnection")]
    #[doc = "Change the `connection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionAvailableEventInit`*"]
    pub fn connection(&mut self, val: &PresentationConnection) -> &mut Self {
        self.set_connection_shim(val);
        self
    }
}
