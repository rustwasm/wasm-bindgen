use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = MIDIAccess , typescript_type = "MIDIAccess" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MidiAccess` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess)
    ///
    ///*This API requires the following crate features to be activated: `MidiAccess`*
    pub type MidiAccess;

    #[cfg(feature = "MidiInputMap")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MIDIAccess" , js_name = inputs ) ]
    ///Getter for the `inputs` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/inputs)
    ///
    ///*This API requires the following crate features to be activated: `MidiAccess`, `MidiInputMap`*
    pub fn inputs(this: &MidiAccess) -> MidiInputMap;

    #[cfg(feature = "MidiOutputMap")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MIDIAccess" , js_name = outputs ) ]
    ///Getter for the `outputs` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/outputs)
    ///
    ///*This API requires the following crate features to be activated: `MidiAccess`, `MidiOutputMap`*
    pub fn outputs(this: &MidiAccess) -> MidiOutputMap;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MIDIAccess" , js_name = onstatechange ) ]
    ///Getter for the `onstatechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/onstatechange)
    ///
    ///*This API requires the following crate features to be activated: `MidiAccess`*
    pub fn onstatechange(this: &MidiAccess) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MIDIAccess" , js_name = onstatechange ) ]
    ///Setter for the `onstatechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/onstatechange)
    ///
    ///*This API requires the following crate features to be activated: `MidiAccess`*
    pub fn set_onstatechange(this: &MidiAccess, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "MIDIAccess" , js_name = sysexEnabled ) ]
    ///Getter for the `sysexEnabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/sysexEnabled)
    ///
    ///*This API requires the following crate features to be activated: `MidiAccess`*
    pub fn sysex_enabled(this: &MidiAccess) -> bool;

}
