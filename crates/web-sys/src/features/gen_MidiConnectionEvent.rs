use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MIDIConnectionEvent , typescript_name = MIDIConnectionEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiConnectionEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent)\n\n*This API requires the following crate features to be activated: `MidiConnectionEvent`*"]
    pub type MidiConnectionEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = port ) ]
    #[cfg(feature = "MidiPort")]
    #[doc = "Getter for the `port` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent/port)\n\n*This API requires the following crate features to be activated: `MidiConnectionEvent`, `MidiPort`*"]
    pub fn port(this: &MidiConnectionEvent) -> Option<MidiPort>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new MidiConnectionEvent(..)` constructor, creating a new instance of `MidiConnectionEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent/MIDIConnectionEvent)\n\n*This API requires the following crate features to be activated: `MidiConnectionEvent`*"]
    pub fn new(this: &MidiConnectionEvent, type_: &str) -> Result<MidiConnectionEvent, JsValue>;
    #[cfg(feature = "MidiConnectionEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new MidiConnectionEvent(..)` constructor, creating a new instance of `MidiConnectionEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent/MIDIConnectionEvent)\n\n*This API requires the following crate features to be activated: `MidiConnectionEvent`, `MidiConnectionEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &MidiConnectionEvent,
        type_: &str,
        event_init_dict: &MidiConnectionEventInit,
    ) -> Result<MidiConnectionEvent, JsValue>;
}
