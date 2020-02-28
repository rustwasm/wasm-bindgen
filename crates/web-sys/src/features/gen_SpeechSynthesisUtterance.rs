use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = SpeechSynthesisUtterance , typescript_name = SpeechSynthesisUtterance ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechSynthesisUtterance` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub type SpeechSynthesisUtterance;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = text ) ]
    #[doc = "Getter for the `text` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/text)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn text(this: &SpeechSynthesisUtterance) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = text ) ]
    #[doc = "Setter for the `text` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/text)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn set_text(this: &SpeechSynthesisUtterance, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = lang ) ]
    #[doc = "Getter for the `lang` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/lang)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn lang(this: &SpeechSynthesisUtterance) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = lang ) ]
    #[doc = "Setter for the `lang` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/lang)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn set_lang(this: &SpeechSynthesisUtterance, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = voice ) ]
    #[cfg(feature = "SpeechSynthesisVoice")]
    #[doc = "Getter for the `voice` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/voice)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`, `SpeechSynthesisVoice`*"]
    pub fn voice(this: &SpeechSynthesisUtterance) -> Option<SpeechSynthesisVoice>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = voice ) ]
    #[cfg(feature = "SpeechSynthesisVoice")]
    #[doc = "Setter for the `voice` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/voice)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`, `SpeechSynthesisVoice`*"]
    pub fn set_voice(this: &SpeechSynthesisUtterance, value: Option<&SpeechSynthesisVoice>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = volume ) ]
    #[doc = "Getter for the `volume` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/volume)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn volume(this: &SpeechSynthesisUtterance) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = volume ) ]
    #[doc = "Setter for the `volume` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/volume)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn set_volume(this: &SpeechSynthesisUtterance, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = rate ) ]
    #[doc = "Getter for the `rate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/rate)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn rate(this: &SpeechSynthesisUtterance) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = rate ) ]
    #[doc = "Setter for the `rate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/rate)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn set_rate(this: &SpeechSynthesisUtterance, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = pitch ) ]
    #[doc = "Getter for the `pitch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/pitch)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn pitch(this: &SpeechSynthesisUtterance) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = pitch ) ]
    #[doc = "Setter for the `pitch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/pitch)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn set_pitch(this: &SpeechSynthesisUtterance, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = onstart ) ]
    #[doc = "Getter for the `onstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onstart)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn onstart(this: &SpeechSynthesisUtterance) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = onstart ) ]
    #[doc = "Setter for the `onstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onstart)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn set_onstart(this: &SpeechSynthesisUtterance, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = onend ) ]
    #[doc = "Getter for the `onend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onend)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn onend(this: &SpeechSynthesisUtterance) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = onend ) ]
    #[doc = "Setter for the `onend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onend)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn set_onend(this: &SpeechSynthesisUtterance, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onerror)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn onerror(this: &SpeechSynthesisUtterance) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onerror)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn set_onerror(this: &SpeechSynthesisUtterance, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = onpause ) ]
    #[doc = "Getter for the `onpause` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onpause)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn onpause(this: &SpeechSynthesisUtterance) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = onpause ) ]
    #[doc = "Setter for the `onpause` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onpause)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn set_onpause(this: &SpeechSynthesisUtterance, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = onresume ) ]
    #[doc = "Getter for the `onresume` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onresume)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn onresume(this: &SpeechSynthesisUtterance) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = onresume ) ]
    #[doc = "Setter for the `onresume` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onresume)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn set_onresume(this: &SpeechSynthesisUtterance, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = onmark ) ]
    #[doc = "Getter for the `onmark` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onmark)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn onmark(this: &SpeechSynthesisUtterance) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = onmark ) ]
    #[doc = "Setter for the `onmark` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onmark)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn set_onmark(this: &SpeechSynthesisUtterance, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisUtterance" , js_name = onboundary ) ]
    #[doc = "Getter for the `onboundary` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onboundary)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn onboundary(this: &SpeechSynthesisUtterance) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SpeechSynthesisUtterance" , js_name = onboundary ) ]
    #[doc = "Setter for the `onboundary` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onboundary)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn set_onboundary(this: &SpeechSynthesisUtterance, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, js_class = "SpeechSynthesisUtterance", constructor)]
    #[doc = "The `new SpeechSynthesisUtterance(..)` constructor, creating a new instance of `SpeechSynthesisUtterance`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/SpeechSynthesisUtterance)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn new(this: &SpeechSynthesisUtterance) -> Result<SpeechSynthesisUtterance, JsValue>;
    #[wasm_bindgen(catch, js_class = "SpeechSynthesisUtterance", constructor)]
    #[doc = "The `new SpeechSynthesisUtterance(..)` constructor, creating a new instance of `SpeechSynthesisUtterance`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/SpeechSynthesisUtterance)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
    pub fn new_with_text(
        this: &SpeechSynthesisUtterance,
        text: &str,
    ) -> Result<SpeechSynthesisUtterance, JsValue>;
}
