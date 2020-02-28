use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = SpeechSynthesis , typescript_name = SpeechSynthesis ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechSynthesis` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub type SpeechSynthesis;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesis" , js_name = pending ) ]
    #[doc = "Getter for the `pending` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/pending)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn pending(this: &SpeechSynthesis) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesis" , js_name = speaking ) ]
    #[doc = "Getter for the `speaking` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/speaking)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn speaking(this: &SpeechSynthesis) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesis" , js_name = paused ) ]
    #[doc = "Getter for the `paused` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/paused)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn paused(this: &SpeechSynthesis) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesis" , js_name = onvoiceschanged ) ]
    #[doc = "Getter for the `onvoiceschanged` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/onvoiceschanged)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn onvoiceschanged(this: &SpeechSynthesis) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesis" , js_name = onvoiceschanged ) ]
    #[doc = "Setter for the `onvoiceschanged` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/onvoiceschanged)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn set_onvoiceschanged(this: &SpeechSynthesis, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( method , structural , js_class = "SpeechSynthesis" , js_name = cancel ) ]
    #[doc = "The `cancel()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/cancel)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn cancel(this: &SpeechSynthesis);
    # [ wasm_bindgen ( method , structural , js_class = "SpeechSynthesis" , js_name = getVoices ) ]
    #[doc = "The `getVoices()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/getVoices)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn get_voices(this: &SpeechSynthesis) -> ::js_sys::Array;
    # [ wasm_bindgen ( method , structural , js_class = "SpeechSynthesis" , js_name = pause ) ]
    #[doc = "The `pause()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/pause)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn pause(this: &SpeechSynthesis);
    # [ wasm_bindgen ( method , structural , js_class = "SpeechSynthesis" , js_name = resume ) ]
    #[doc = "The `resume()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/resume)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn resume(this: &SpeechSynthesis);
    #[cfg(feature = "SpeechSynthesisUtterance")]
    # [ wasm_bindgen ( method , structural , js_class = "SpeechSynthesis" , js_name = speak ) ]
    #[doc = "The `speak()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/speak)\n\n*This API requires the following crate features to be activated: `SpeechSynthesis`, `SpeechSynthesisUtterance`*"]
    pub fn speak(this: &SpeechSynthesis, utterance: &SpeechSynthesisUtterance);
}
