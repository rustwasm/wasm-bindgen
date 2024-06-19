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
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &PresentationConnectionAvailableEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &PresentationConnectionAvailableEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &PresentationConnectionAvailableEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &PresentationConnectionAvailableEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &PresentationConnectionAvailableEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &PresentationConnectionAvailableEventInit, val: bool);
    #[cfg(feature = "PresentationConnection")]
    #[doc = "Get the `connection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionAvailableEventInit`*"]
    #[wasm_bindgen(method, getter = "connection")]
    pub fn get_connection(
        this: &PresentationConnectionAvailableEventInit,
    ) -> PresentationConnection;
    #[cfg(feature = "PresentationConnection")]
    #[wasm_bindgen(method, setter = "connection")]
    fn set_connection(
        this: &PresentationConnectionAvailableEventInit,
        val: &PresentationConnection,
    );
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
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "PresentationConnection")]
    #[doc = "Change the `connection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionAvailableEventInit`*"]
    pub fn connection(&mut self, val: &PresentationConnection) -> &mut Self {
        self.set_connection(val);
        self
    }
}
