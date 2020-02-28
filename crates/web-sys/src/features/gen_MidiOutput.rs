use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = MidiPort , extends = EventTarget , extends = :: js_sys :: Object , js_name = MIDIOutput , typescript_name = MIDIOutput ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiOutput` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput)\n\n*This API requires the following crate features to be activated: `MidiOutput`*"]
    pub type MidiOutput;
    # [ wasm_bindgen ( method , structural , js_class = "MIDIOutput" , js_name = clear ) ]
    #[doc = "The `clear()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput/clear)\n\n*This API requires the following crate features to be activated: `MidiOutput`*"]
    pub fn clear(this: &MidiOutput);
    # [ wasm_bindgen ( catch , method , structural , js_class = "MIDIOutput" , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput/send)\n\n*This API requires the following crate features to be activated: `MidiOutput`*"]
    pub fn send(this: &MidiOutput, data: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "MIDIOutput" , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput/send)\n\n*This API requires the following crate features to be activated: `MidiOutput`*"]
    pub fn send_with_timestamp(
        this: &MidiOutput,
        data: &::wasm_bindgen::JsValue,
        timestamp: f64,
    ) -> Result<(), JsValue>;
}
