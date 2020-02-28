use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = MIDIAccess , typescript_name = MIDIAccess ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiAccess` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess)\n\n*This API requires the following crate features to be activated: `MidiAccess`*"]
    pub type MidiAccess;
    # [ wasm_bindgen ( structural , method , getter , js_name = inputs ) ]
    #[cfg(feature = "MidiInputMap")]
    #[doc = "Getter for the `inputs` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/inputs)\n\n*This API requires the following crate features to be activated: `MidiAccess`, `MidiInputMap`*"]
    pub fn inputs(this: &MidiAccess) -> MidiInputMap;
    # [ wasm_bindgen ( structural , method , getter , js_name = outputs ) ]
    #[cfg(feature = "MidiOutputMap")]
    #[doc = "Getter for the `outputs` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/outputs)\n\n*This API requires the following crate features to be activated: `MidiAccess`, `MidiOutputMap`*"]
    pub fn outputs(this: &MidiAccess) -> MidiOutputMap;
    # [ wasm_bindgen ( structural , method , getter , js_name = onstatechange ) ]
    #[doc = "Getter for the `onstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/onstatechange)\n\n*This API requires the following crate features to be activated: `MidiAccess`*"]
    pub fn onstatechange(this: &MidiAccess) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onstatechange ) ]
    #[doc = "Setter for the `onstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/onstatechange)\n\n*This API requires the following crate features to be activated: `MidiAccess`*"]
    pub fn set_onstatechange(this: &MidiAccess, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = sysexEnabled ) ]
    #[doc = "Getter for the `sysexEnabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/sysexEnabled)\n\n*This API requires the following crate features to be activated: `MidiAccess`*"]
    pub fn sysex_enabled(this: &MidiAccess) -> bool;
}
