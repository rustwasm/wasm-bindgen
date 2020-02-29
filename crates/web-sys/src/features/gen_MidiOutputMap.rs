use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MIDIOutputMap , typescript_name = MIDIOutputMap ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MidiOutputMap` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap)
    ///
    ///*This API requires the following crate features to be activated: `MidiOutputMap`*
    pub type MidiOutputMap;

}
