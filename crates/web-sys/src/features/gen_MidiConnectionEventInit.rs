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
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &MidiConnectionEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &MidiConnectionEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &MidiConnectionEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &MidiConnectionEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &MidiConnectionEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &MidiConnectionEventInit, val: bool);
    #[cfg(feature = "MidiPort")]
    #[doc = "Get the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`, `MidiPort`*"]
    #[wasm_bindgen(method, getter = "port")]
    pub fn get_port(this: &MidiConnectionEventInit) -> Option<MidiPort>;
    #[cfg(feature = "MidiPort")]
    #[wasm_bindgen(method, setter = "port")]
    fn set_port(this: &MidiConnectionEventInit, val: Option<&MidiPort>);
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
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "MidiPort")]
    #[doc = "Change the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`, `MidiPort`*"]
    pub fn port(&mut self, val: Option<&MidiPort>) -> &mut Self {
        self.set_port(val);
        self
    }
}
impl Default for MidiConnectionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
