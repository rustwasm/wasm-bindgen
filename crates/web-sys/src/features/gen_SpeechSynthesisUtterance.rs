use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = SpeechSynthesisUtterance , typescript_type = "SpeechSynthesisUtterance" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SpeechSynthesisUtterance` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub type SpeechSynthesisUtterance;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = text ) ]
    ///Getter for the `text` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/text)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn text(this: &SpeechSynthesisUtterance) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = text ) ]
    ///Setter for the `text` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/text)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn set_text(this: &SpeechSynthesisUtterance, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = lang ) ]
    ///Getter for the `lang` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/lang)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn lang(this: &SpeechSynthesisUtterance) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = lang ) ]
    ///Setter for the `lang` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/lang)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn set_lang(this: &SpeechSynthesisUtterance, value: &str);

    #[cfg(feature = "SpeechSynthesisVoice")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = voice ) ]
    ///Getter for the `voice` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/voice)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`, `SpeechSynthesisVoice`*
    pub fn voice(this: &SpeechSynthesisUtterance) -> Option<SpeechSynthesisVoice>;

    #[cfg(feature = "SpeechSynthesisVoice")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = voice ) ]
    ///Setter for the `voice` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/voice)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`, `SpeechSynthesisVoice`*
    pub fn set_voice(this: &SpeechSynthesisUtterance, value: Option<&SpeechSynthesisVoice>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = volume ) ]
    ///Getter for the `volume` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/volume)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn volume(this: &SpeechSynthesisUtterance) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = volume ) ]
    ///Setter for the `volume` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/volume)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn set_volume(this: &SpeechSynthesisUtterance, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = rate ) ]
    ///Getter for the `rate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/rate)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn rate(this: &SpeechSynthesisUtterance) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = rate ) ]
    ///Setter for the `rate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/rate)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn set_rate(this: &SpeechSynthesisUtterance, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = pitch ) ]
    ///Getter for the `pitch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/pitch)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn pitch(this: &SpeechSynthesisUtterance) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = pitch ) ]
    ///Setter for the `pitch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/pitch)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn set_pitch(this: &SpeechSynthesisUtterance, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = onstart ) ]
    ///Getter for the `onstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onstart)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn onstart(this: &SpeechSynthesisUtterance) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = onstart ) ]
    ///Setter for the `onstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onstart)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn set_onstart(this: &SpeechSynthesisUtterance, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = onend ) ]
    ///Getter for the `onend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onend)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn onend(this: &SpeechSynthesisUtterance) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = onend ) ]
    ///Setter for the `onend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onend)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn set_onend(this: &SpeechSynthesisUtterance, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onerror)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn onerror(this: &SpeechSynthesisUtterance) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onerror)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn set_onerror(this: &SpeechSynthesisUtterance, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = onpause ) ]
    ///Getter for the `onpause` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onpause)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn onpause(this: &SpeechSynthesisUtterance) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = onpause ) ]
    ///Setter for the `onpause` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onpause)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn set_onpause(this: &SpeechSynthesisUtterance, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = onresume ) ]
    ///Getter for the `onresume` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onresume)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn onresume(this: &SpeechSynthesisUtterance) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = onresume ) ]
    ///Setter for the `onresume` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onresume)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn set_onresume(this: &SpeechSynthesisUtterance, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = onmark ) ]
    ///Getter for the `onmark` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onmark)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn onmark(this: &SpeechSynthesisUtterance) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = onmark ) ]
    ///Setter for the `onmark` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onmark)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn set_onmark(this: &SpeechSynthesisUtterance, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = onboundary ) ]
    ///Getter for the `onboundary` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onboundary)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn onboundary(this: &SpeechSynthesisUtterance) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = onboundary ) ]
    ///Setter for the `onboundary` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onboundary)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn set_onboundary(this: &SpeechSynthesisUtterance, value: Option<&::js_sys::Function>);

    #[wasm_bindgen(catch, constructor, js_class = "SpeechSynthesisUtterance")]
    ///The `new SpeechSynthesisUtterance(..)` constructor, creating a new instance of `SpeechSynthesisUtterance`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/SpeechSynthesisUtterance)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn new() -> Result<SpeechSynthesisUtterance, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "SpeechSynthesisUtterance")]
    ///The `new SpeechSynthesisUtterance(..)` constructor, creating a new instance of `SpeechSynthesisUtterance`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/SpeechSynthesisUtterance)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*
    pub fn new_with_text(text: &str) -> Result<SpeechSynthesisUtterance, JsValue>;

}
