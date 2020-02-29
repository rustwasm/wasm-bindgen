use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = SpeechSynthesis , typescript_name = SpeechSynthesis ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SpeechSynthesis` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesis`*
    pub type SpeechSynthesis;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesis" , js_name = pending ) ]
    ///Getter for the `pending` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/pending)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesis`*
    pub fn pending(this: &SpeechSynthesis) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesis" , js_name = speaking ) ]
    ///Getter for the `speaking` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/speaking)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesis`*
    pub fn speaking(this: &SpeechSynthesis) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesis" , js_name = paused ) ]
    ///Getter for the `paused` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/paused)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesis`*
    pub fn paused(this: &SpeechSynthesis) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesis" , js_name = onvoiceschanged ) ]
    ///Getter for the `onvoiceschanged` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/onvoiceschanged)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesis`*
    pub fn onvoiceschanged(this: &SpeechSynthesis) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesis" , js_name = onvoiceschanged ) ]
    ///Setter for the `onvoiceschanged` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/onvoiceschanged)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesis`*
    pub fn set_onvoiceschanged(this: &SpeechSynthesis, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( method , structural , js_class = "SpeechSynthesis" , js_name = cancel ) ]
    ///The `cancel()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/cancel)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesis`*
    pub fn cancel(this: &SpeechSynthesis);

    # [ wasm_bindgen ( method , structural , js_class = "SpeechSynthesis" , js_name = getVoices ) ]
    ///The `getVoices()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/getVoices)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesis`*
    pub fn get_voices(this: &SpeechSynthesis) -> ::js_sys::Array;

    # [ wasm_bindgen ( method , structural , js_class = "SpeechSynthesis" , js_name = pause ) ]
    ///The `pause()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/pause)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesis`*
    pub fn pause(this: &SpeechSynthesis);

    # [ wasm_bindgen ( method , structural , js_class = "SpeechSynthesis" , js_name = resume ) ]
    ///The `resume()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/resume)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesis`*
    pub fn resume(this: &SpeechSynthesis);

    #[cfg(feature = "SpeechSynthesisUtterance")]
    # [ wasm_bindgen ( method , structural , js_class = "SpeechSynthesis" , js_name = speak ) ]
    ///The `speak()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/speak)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesis`, `SpeechSynthesisUtterance`*
    pub fn speak(this: &SpeechSynthesis, utterance: &SpeechSynthesisUtterance);

}
