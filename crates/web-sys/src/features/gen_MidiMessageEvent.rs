use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MIDIMessageEvent , typescript_name = MIDIMessageEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MidiMessageEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MidiMessageEvent`*
    pub type MidiMessageEvent;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "MIDIMessageEvent" , js_name = data ) ]
    ///Getter for the `data` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent/data)
    ///
    ///*This API requires the following crate features to be activated: `MidiMessageEvent`*
    pub fn data(this: &MidiMessageEvent) -> Result<Vec<u8>, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "MIDIMessageEvent")]
    ///The `new MidiMessageEvent(..)` constructor, creating a new instance of `MidiMessageEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent/MIDIMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MidiMessageEvent`*
    pub fn new(type_: &str) -> Result<MidiMessageEvent, JsValue>;

    #[cfg(feature = "MidiMessageEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MIDIMessageEvent")]
    ///The `new MidiMessageEvent(..)` constructor, creating a new instance of `MidiMessageEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent/MIDIMessageEvent)
    ///
    ///*This API requires the following crate features to be activated: `MidiMessageEvent`, `MidiMessageEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &MidiMessageEventInit,
    ) -> Result<MidiMessageEvent, JsValue>;

}
