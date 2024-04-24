#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MIDIConnectionEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiConnectionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
    pub type MidiConnectionEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &MidiConnectionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &MidiConnectionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &MidiConnectionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &MidiConnectionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &MidiConnectionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &MidiConnectionEventInit, val: bool);
    #[cfg(feature = "MidiPort")]
    #[wasm_bindgen(method, getter = "port")]
    fn port_shim(this: &MidiConnectionEventInit) -> Option<&MidiPort>;
    #[cfg(feature = "MidiPort")]
    #[wasm_bindgen(method, setter = "port")]
    fn set_port_shim(this: &MidiConnectionEventInit, val: Option<&MidiPort>);
}
#[doc = "The trait to access properties on the `MidiConnectionEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
pub trait MidiConnectionEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "MidiPort")]
    #[doc = "Get the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`, `MidiPort`*"]
    fn port(&self) -> Option<&MidiPort>;
}
impl MidiConnectionEventInitGetters for MidiConnectionEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "MidiPort")]
    fn port(&self) -> Option<&MidiPort> {
        self.port_shim()
    }
}
impl MidiConnectionEventInit {
    #[doc = "Construct a new `MidiConnectionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "MidiPort")]
    #[doc = "Change the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`, `MidiPort`*"]
    pub fn port(&mut self, val: Option<&MidiPort>) -> &mut Self {
        self.set_port_shim(val);
        self
    }
}
impl Default for MidiConnectionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
