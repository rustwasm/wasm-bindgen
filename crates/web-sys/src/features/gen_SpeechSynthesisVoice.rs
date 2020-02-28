use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SpeechSynthesisVoice , typescript_name = SpeechSynthesisVoice ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechSynthesisVoice` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    pub type SpeechSynthesisVoice;
    # [ wasm_bindgen ( structural , method , getter , js_name = voiceURI ) ]
    #[doc = "Getter for the `voiceURI` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/voiceURI)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    pub fn voice_uri(this: &SpeechSynthesisVoice) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/name)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    pub fn name(this: &SpeechSynthesisVoice) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = lang ) ]
    #[doc = "Getter for the `lang` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/lang)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    pub fn lang(this: &SpeechSynthesisVoice) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = localService ) ]
    #[doc = "Getter for the `localService` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/localService)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    pub fn local_service(this: &SpeechSynthesisVoice) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = default ) ]
    #[doc = "Getter for the `default` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/default)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
    pub fn default(this: &SpeechSynthesisVoice) -> bool;
}
