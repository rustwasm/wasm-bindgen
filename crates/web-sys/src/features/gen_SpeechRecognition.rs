use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = SpeechRecognition , typescript_type = "SpeechRecognition" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SpeechRecognition` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub type SpeechRecognition;

    #[cfg(feature = "SpeechGrammarList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognition" , js_name = grammars ) ]
    ///Getter for the `grammars` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/grammars)
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammarList`, `SpeechRecognition`*
    pub fn grammars(this: &SpeechRecognition) -> SpeechGrammarList;

    #[cfg(feature = "SpeechGrammarList")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechRecognition" , js_name = grammars ) ]
    ///Setter for the `grammars` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/grammars)
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammarList`, `SpeechRecognition`*
    pub fn set_grammars(this: &SpeechRecognition, value: &SpeechGrammarList);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognition" , js_name = lang ) ]
    ///Getter for the `lang` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/lang)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn lang(this: &SpeechRecognition) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechRecognition" , js_name = lang ) ]
    ///Setter for the `lang` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/lang)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn set_lang(this: &SpeechRecognition, value: &str);

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "SpeechRecognition" , js_name = continuous ) ]
    ///Getter for the `continuous` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/continuous)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn continuous(this: &SpeechRecognition) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "SpeechRecognition" , js_name = continuous ) ]
    ///Setter for the `continuous` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/continuous)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn set_continuous(this: &SpeechRecognition, value: bool) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognition" , js_name = interimResults ) ]
    ///Getter for the `interimResults` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/interimResults)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn interim_results(this: &SpeechRecognition) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechRecognition" , js_name = interimResults ) ]
    ///Setter for the `interimResults` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/interimResults)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn set_interim_results(this: &SpeechRecognition, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognition" , js_name = maxAlternatives ) ]
    ///Getter for the `maxAlternatives` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/maxAlternatives)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn max_alternatives(this: &SpeechRecognition) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechRecognition" , js_name = maxAlternatives ) ]
    ///Setter for the `maxAlternatives` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/maxAlternatives)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn set_max_alternatives(this: &SpeechRecognition, value: u32);

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "SpeechRecognition" , js_name = serviceURI ) ]
    ///Getter for the `serviceURI` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/serviceURI)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn service_uri(this: &SpeechRecognition) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "SpeechRecognition" , js_name = serviceURI ) ]
    ///Setter for the `serviceURI` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/serviceURI)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn set_service_uri(this: &SpeechRecognition, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognition" , js_name = onaudiostart ) ]
    ///Getter for the `onaudiostart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudiostart)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn onaudiostart(this: &SpeechRecognition) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechRecognition" , js_name = onaudiostart ) ]
    ///Setter for the `onaudiostart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudiostart)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn set_onaudiostart(this: &SpeechRecognition, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognition" , js_name = onsoundstart ) ]
    ///Getter for the `onsoundstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundstart)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn onsoundstart(this: &SpeechRecognition) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechRecognition" , js_name = onsoundstart ) ]
    ///Setter for the `onsoundstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundstart)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn set_onsoundstart(this: &SpeechRecognition, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognition" , js_name = onspeechstart ) ]
    ///Getter for the `onspeechstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechstart)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn onspeechstart(this: &SpeechRecognition) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechRecognition" , js_name = onspeechstart ) ]
    ///Setter for the `onspeechstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechstart)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn set_onspeechstart(this: &SpeechRecognition, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognition" , js_name = onspeechend ) ]
    ///Getter for the `onspeechend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechend)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn onspeechend(this: &SpeechRecognition) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechRecognition" , js_name = onspeechend ) ]
    ///Setter for the `onspeechend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechend)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn set_onspeechend(this: &SpeechRecognition, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognition" , js_name = onsoundend ) ]
    ///Getter for the `onsoundend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundend)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn onsoundend(this: &SpeechRecognition) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechRecognition" , js_name = onsoundend ) ]
    ///Setter for the `onsoundend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundend)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn set_onsoundend(this: &SpeechRecognition, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognition" , js_name = onaudioend ) ]
    ///Getter for the `onaudioend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudioend)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn onaudioend(this: &SpeechRecognition) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechRecognition" , js_name = onaudioend ) ]
    ///Setter for the `onaudioend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudioend)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn set_onaudioend(this: &SpeechRecognition, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognition" , js_name = onresult ) ]
    ///Getter for the `onresult` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onresult)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn onresult(this: &SpeechRecognition) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechRecognition" , js_name = onresult ) ]
    ///Setter for the `onresult` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onresult)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn set_onresult(this: &SpeechRecognition, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognition" , js_name = onnomatch ) ]
    ///Getter for the `onnomatch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onnomatch)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn onnomatch(this: &SpeechRecognition) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechRecognition" , js_name = onnomatch ) ]
    ///Setter for the `onnomatch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onnomatch)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn set_onnomatch(this: &SpeechRecognition, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognition" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onerror)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn onerror(this: &SpeechRecognition) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechRecognition" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onerror)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn set_onerror(this: &SpeechRecognition, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognition" , js_name = onstart ) ]
    ///Getter for the `onstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onstart)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn onstart(this: &SpeechRecognition) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechRecognition" , js_name = onstart ) ]
    ///Setter for the `onstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onstart)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn set_onstart(this: &SpeechRecognition, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognition" , js_name = onend ) ]
    ///Getter for the `onend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onend)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn onend(this: &SpeechRecognition) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechRecognition" , js_name = onend ) ]
    ///Setter for the `onend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onend)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn set_onend(this: &SpeechRecognition, value: Option<&::js_sys::Function>);

    #[wasm_bindgen(catch, constructor, js_class = "SpeechRecognition")]
    ///The `new SpeechRecognition(..)` constructor, creating a new instance of `SpeechRecognition`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/SpeechRecognition)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn new() -> Result<SpeechRecognition, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "SpeechRecognition" , js_name = abort ) ]
    ///The `abort()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/abort)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn abort(this: &SpeechRecognition);

    # [ wasm_bindgen ( catch , method , structural , js_class = "SpeechRecognition" , js_name = start ) ]
    ///The `start()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/start)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn start(this: &SpeechRecognition) -> Result<(), JsValue>;

    #[cfg(feature = "MediaStream")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SpeechRecognition" , js_name = start ) ]
    ///The `start()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/start)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`, `SpeechRecognition`*
    pub fn start_with_stream(this: &SpeechRecognition, stream: &MediaStream)
        -> Result<(), JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "SpeechRecognition" , js_name = stop ) ]
    ///The `stop()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/stop)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognition`*
    pub fn stop(this: &SpeechRecognition);

}
