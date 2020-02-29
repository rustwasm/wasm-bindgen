use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = MidiPort , extends = EventTarget , extends = :: js_sys :: Object , js_name = MIDIInput , typescript_type = "MIDIInput" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MidiInput` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInput)
    ///
    ///*This API requires the following crate features to be activated: `MidiInput`*
    pub type MidiInput;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MIDIInput" , js_name = onmidimessage ) ]
    ///Getter for the `onmidimessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInput/onmidimessage)
    ///
    ///*This API requires the following crate features to be activated: `MidiInput`*
    pub fn onmidimessage(this: &MidiInput) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MIDIInput" , js_name = onmidimessage ) ]
    ///Setter for the `onmidimessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInput/onmidimessage)
    ///
    ///*This API requires the following crate features to be activated: `MidiInput`*
    pub fn set_onmidimessage(this: &MidiInput, value: Option<&::js_sys::Function>);

}
