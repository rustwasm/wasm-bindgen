#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MIDIOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOptions`*"]
    pub type MidiOptions;
    #[wasm_bindgen(method, getter = "software")]
    fn software_shim(this: &MidiOptions) -> bool;
    #[wasm_bindgen(method, setter = "software")]
    fn set_software_shim(this: &MidiOptions, val: bool);
    #[wasm_bindgen(method, getter = "sysex")]
    fn sysex_shim(this: &MidiOptions) -> bool;
    #[wasm_bindgen(method, setter = "sysex")]
    fn set_sysex_shim(this: &MidiOptions, val: bool);
}
#[doc = "The trait to access properties on the `MidiOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MidiOptions`*"]
pub trait MidiOptionsGetters {
    #[doc = "Get the `software` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOptions`*"]
    fn software(&self) -> bool;
    #[doc = "Get the `sysex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOptions`*"]
    fn sysex(&self) -> bool;
}
impl MidiOptionsGetters for MidiOptions {
    fn software(&self) -> bool {
        self.software_shim()
    }
    fn sysex(&self) -> bool {
        self.sysex_shim()
    }
}
impl MidiOptions {
    #[doc = "Construct a new `MidiOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `software` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOptions`*"]
    pub fn software(&mut self, val: bool) -> &mut Self {
        self.set_software_shim(val);
        self
    }
    #[doc = "Change the `sysex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOptions`*"]
    pub fn sysex(&mut self, val: bool) -> &mut Self {
        self.set_sysex_shim(val);
        self
    }
}
impl Default for MidiOptions {
    fn default() -> Self {
        Self::new()
    }
}
