use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = MIDIPort , typescript_name = MIDIPort ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiPort` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    pub type MidiPort;
    # [ wasm_bindgen ( structural , method , getter , js_name = id ) ]
    #[doc = "Getter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/id)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    pub fn id(this: &MidiPort) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = manufacturer ) ]
    #[doc = "Getter for the `manufacturer` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/manufacturer)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    pub fn manufacturer(this: &MidiPort) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/name)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    pub fn name(this: &MidiPort) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = version ) ]
    #[doc = "Getter for the `version` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/version)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    pub fn version(this: &MidiPort) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[cfg(feature = "MidiPortType")]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/type)\n\n*This API requires the following crate features to be activated: `MidiPort`, `MidiPortType`*"]
    pub fn type_(this: &MidiPort) -> MidiPortType;
    # [ wasm_bindgen ( structural , method , getter , js_name = state ) ]
    #[cfg(feature = "MidiPortDeviceState")]
    #[doc = "Getter for the `state` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/state)\n\n*This API requires the following crate features to be activated: `MidiPort`, `MidiPortDeviceState`*"]
    pub fn state(this: &MidiPort) -> MidiPortDeviceState;
    # [ wasm_bindgen ( structural , method , getter , js_name = connection ) ]
    #[cfg(feature = "MidiPortConnectionState")]
    #[doc = "Getter for the `connection` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/connection)\n\n*This API requires the following crate features to be activated: `MidiPort`, `MidiPortConnectionState`*"]
    pub fn connection(this: &MidiPort) -> MidiPortConnectionState;
    # [ wasm_bindgen ( structural , method , getter , js_name = onstatechange ) ]
    #[doc = "Getter for the `onstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/onstatechange)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    pub fn onstatechange(this: &MidiPort) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onstatechange ) ]
    #[doc = "Setter for the `onstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/onstatechange)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    pub fn set_onstatechange(this: &MidiPort, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( method , structural , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/close)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    pub fn close(this: &MidiPort) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_name = open ) ]
    #[doc = "The `open()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/open)\n\n*This API requires the following crate features to be activated: `MidiPort`*"]
    pub fn open(this: &MidiPort) -> ::js_sys::Promise;
}
