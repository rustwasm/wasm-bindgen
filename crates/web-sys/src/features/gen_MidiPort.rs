use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = MIDIPort , typescript_name = MIDIPort ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MidiPort` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort)
    ///
    ///*This API requires the following crate features to be activated: `MidiPort`*
    pub type MidiPort;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MIDIPort" , js_name = id ) ]
    ///Getter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/id)
    ///
    ///*This API requires the following crate features to be activated: `MidiPort`*
    pub fn id(this: &MidiPort) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MIDIPort" , js_name = manufacturer ) ]
    ///Getter for the `manufacturer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/manufacturer)
    ///
    ///*This API requires the following crate features to be activated: `MidiPort`*
    pub fn manufacturer(this: &MidiPort) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MIDIPort" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/name)
    ///
    ///*This API requires the following crate features to be activated: `MidiPort`*
    pub fn name(this: &MidiPort) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MIDIPort" , js_name = version ) ]
    ///Getter for the `version` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/version)
    ///
    ///*This API requires the following crate features to be activated: `MidiPort`*
    pub fn version(this: &MidiPort) -> Option<String>;

    #[cfg(feature = "MidiPortType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MIDIPort" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/type)
    ///
    ///*This API requires the following crate features to be activated: `MidiPort`, `MidiPortType`*
    pub fn type_(this: &MidiPort) -> MidiPortType;

    #[cfg(feature = "MidiPortDeviceState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MIDIPort" , js_name = state ) ]
    ///Getter for the `state` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/state)
    ///
    ///*This API requires the following crate features to be activated: `MidiPort`, `MidiPortDeviceState`*
    pub fn state(this: &MidiPort) -> MidiPortDeviceState;

    #[cfg(feature = "MidiPortConnectionState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MIDIPort" , js_name = connection ) ]
    ///Getter for the `connection` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/connection)
    ///
    ///*This API requires the following crate features to be activated: `MidiPort`, `MidiPortConnectionState`*
    pub fn connection(this: &MidiPort) -> MidiPortConnectionState;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MIDIPort" , js_name = onstatechange ) ]
    ///Getter for the `onstatechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/onstatechange)
    ///
    ///*This API requires the following crate features to be activated: `MidiPort`*
    pub fn onstatechange(this: &MidiPort) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MIDIPort" , js_name = onstatechange ) ]
    ///Setter for the `onstatechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/onstatechange)
    ///
    ///*This API requires the following crate features to be activated: `MidiPort`*
    pub fn set_onstatechange(this: &MidiPort, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( method , structural , js_class = "MIDIPort" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/close)
    ///
    ///*This API requires the following crate features to be activated: `MidiPort`*
    pub fn close(this: &MidiPort) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "MIDIPort" , js_name = open ) ]
    ///The `open()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/open)
    ///
    ///*This API requires the following crate features to be activated: `MidiPort`*
    pub fn open(this: &MidiPort) -> ::js_sys::Promise;

}
