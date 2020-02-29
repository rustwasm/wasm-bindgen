use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SpeechSynthesisVoice , typescript_type = "SpeechSynthesisVoice" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SpeechSynthesisVoice` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*
    pub type SpeechSynthesisVoice;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisVoice" , js_name = voiceURI ) ]
    ///Getter for the `voiceURI` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/voiceURI)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*
    pub fn voice_uri(this: &SpeechSynthesisVoice) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisVoice" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/name)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*
    pub fn name(this: &SpeechSynthesisVoice) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisVoice" , js_name = lang ) ]
    ///Getter for the `lang` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/lang)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*
    pub fn lang(this: &SpeechSynthesisVoice) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisVoice" , js_name = localService ) ]
    ///Getter for the `localService` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/localService)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*
    pub fn local_service(this: &SpeechSynthesisVoice) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisVoice" , js_name = default ) ]
    ///Getter for the `default` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/default)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*
    pub fn default(this: &SpeechSynthesisVoice) -> bool;

}
