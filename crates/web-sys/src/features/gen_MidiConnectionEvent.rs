use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MIDIConnectionEvent , typescript_name = MIDIConnectionEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MidiConnectionEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent)
    ///
    ///*This API requires the following crate features to be activated: `MidiConnectionEvent`*
    pub type MidiConnectionEvent;

    #[cfg(feature = "MidiPort")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MIDIConnectionEvent" , js_name = port ) ]
    ///Getter for the `port` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent/port)
    ///
    ///*This API requires the following crate features to be activated: `MidiConnectionEvent`, `MidiPort`*
    pub fn port(this: &MidiConnectionEvent) -> Option<MidiPort>;

    #[wasm_bindgen(catch, constructor, js_class = "MIDIConnectionEvent")]
    ///The `new MidiConnectionEvent(..)` constructor, creating a new instance of `MidiConnectionEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent/MIDIConnectionEvent)
    ///
    ///*This API requires the following crate features to be activated: `MidiConnectionEvent`*
    pub fn new(type_: &str) -> Result<MidiConnectionEvent, JsValue>;

    #[cfg(feature = "MidiConnectionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MIDIConnectionEvent")]
    ///The `new MidiConnectionEvent(..)` constructor, creating a new instance of `MidiConnectionEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent/MIDIConnectionEvent)
    ///
    ///*This API requires the following crate features to be activated: `MidiConnectionEvent`, `MidiConnectionEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &MidiConnectionEventInit,
    ) -> Result<MidiConnectionEvent, JsValue>;

}
