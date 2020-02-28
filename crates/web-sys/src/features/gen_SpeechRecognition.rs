use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = SpeechRecognition , typescript_name = SpeechRecognition ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognition` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub type SpeechRecognition;
    # [ wasm_bindgen ( structural , method , getter , js_name = grammars ) ]
    #[cfg(feature = "SpeechGrammarList")]
    #[doc = "Getter for the `grammars` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/grammars)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`, `SpeechRecognition`*"]
    pub fn grammars(this: &SpeechRecognition) -> SpeechGrammarList;
    # [ wasm_bindgen ( structural , method , setter , js_name = grammars ) ]
    #[cfg(feature = "SpeechGrammarList")]
    #[doc = "Setter for the `grammars` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/grammars)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`, `SpeechRecognition`*"]
    pub fn set_grammars(this: &SpeechRecognition, value: &SpeechGrammarList);
    # [ wasm_bindgen ( structural , method , getter , js_name = lang ) ]
    #[doc = "Getter for the `lang` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/lang)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn lang(this: &SpeechRecognition) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = lang ) ]
    #[doc = "Setter for the `lang` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/lang)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_lang(this: &SpeechRecognition, value: &str);
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = continuous ) ]
    #[doc = "Getter for the `continuous` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/continuous)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn continuous(this: &SpeechRecognition) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = continuous ) ]
    #[doc = "Setter for the `continuous` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/continuous)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_continuous(this: &SpeechRecognition, value: bool) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = interimResults ) ]
    #[doc = "Getter for the `interimResults` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/interimResults)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn interim_results(this: &SpeechRecognition) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = interimResults ) ]
    #[doc = "Setter for the `interimResults` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/interimResults)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_interim_results(this: &SpeechRecognition, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = maxAlternatives ) ]
    #[doc = "Getter for the `maxAlternatives` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/maxAlternatives)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn max_alternatives(this: &SpeechRecognition) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_name = maxAlternatives ) ]
    #[doc = "Setter for the `maxAlternatives` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/maxAlternatives)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_max_alternatives(this: &SpeechRecognition, value: u32);
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = serviceURI ) ]
    #[doc = "Getter for the `serviceURI` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/serviceURI)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn service_uri(this: &SpeechRecognition) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = serviceURI ) ]
    #[doc = "Setter for the `serviceURI` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/serviceURI)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_service_uri(this: &SpeechRecognition, value: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = onaudiostart ) ]
    #[doc = "Getter for the `onaudiostart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudiostart)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onaudiostart(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onaudiostart ) ]
    #[doc = "Setter for the `onaudiostart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudiostart)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onaudiostart(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onsoundstart ) ]
    #[doc = "Getter for the `onsoundstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundstart)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onsoundstart(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onsoundstart ) ]
    #[doc = "Setter for the `onsoundstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundstart)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onsoundstart(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onspeechstart ) ]
    #[doc = "Getter for the `onspeechstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechstart)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onspeechstart(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onspeechstart ) ]
    #[doc = "Setter for the `onspeechstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechstart)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onspeechstart(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onspeechend ) ]
    #[doc = "Getter for the `onspeechend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechend)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onspeechend(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onspeechend ) ]
    #[doc = "Setter for the `onspeechend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechend)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onspeechend(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onsoundend ) ]
    #[doc = "Getter for the `onsoundend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundend)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onsoundend(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onsoundend ) ]
    #[doc = "Setter for the `onsoundend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundend)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onsoundend(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onaudioend ) ]
    #[doc = "Getter for the `onaudioend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudioend)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onaudioend(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onaudioend ) ]
    #[doc = "Setter for the `onaudioend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudioend)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onaudioend(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onresult ) ]
    #[doc = "Getter for the `onresult` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onresult)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onresult(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onresult ) ]
    #[doc = "Setter for the `onresult` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onresult)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onresult(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onnomatch ) ]
    #[doc = "Getter for the `onnomatch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onnomatch)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onnomatch(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onnomatch ) ]
    #[doc = "Setter for the `onnomatch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onnomatch)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onnomatch(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onerror)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onerror(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onerror)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onerror(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onstart ) ]
    #[doc = "Getter for the `onstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onstart)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onstart(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onstart ) ]
    #[doc = "Setter for the `onstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onstart)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onstart(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onend ) ]
    #[doc = "Getter for the `onend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onend)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onend(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onend ) ]
    #[doc = "Setter for the `onend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onend)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onend(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new SpeechRecognition(..)` constructor, creating a new instance of `SpeechRecognition`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/SpeechRecognition)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn new(this: &SpeechRecognition) -> Result<SpeechRecognition, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = abort ) ]
    #[doc = "The `abort()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/abort)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn abort(this: &SpeechRecognition);
    # [ wasm_bindgen ( catch , method , structural , js_name = start ) ]
    #[doc = "The `start()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/start)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn start(this: &SpeechRecognition) -> Result<(), JsValue>;
    #[cfg(feature = "MediaStream")]
    # [ wasm_bindgen ( catch , method , structural , js_name = start ) ]
    #[doc = "The `start()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/start)\n\n*This API requires the following crate features to be activated: `MediaStream`, `SpeechRecognition`*"]
    pub fn start_with_stream(this: &SpeechRecognition, stream: &MediaStream)
        -> Result<(), JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = stop ) ]
    #[doc = "The `stop()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/stop)\n\n*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn stop(this: &SpeechRecognition);
}
