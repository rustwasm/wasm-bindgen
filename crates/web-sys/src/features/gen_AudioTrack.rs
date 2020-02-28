use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AudioTrack , typescript_name = AudioTrack ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioTrack` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack)\n\n*This API requires the following crate features to be activated: `AudioTrack`*"]
    pub type AudioTrack;
    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioTrack" , js_name = id ) ]
    #[doc = "Getter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/id)\n\n*This API requires the following crate features to be activated: `AudioTrack`*"]
    pub fn id(this: &AudioTrack) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioTrack" , js_name = kind ) ]
    #[doc = "Getter for the `kind` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/kind)\n\n*This API requires the following crate features to be activated: `AudioTrack`*"]
    pub fn kind(this: &AudioTrack) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioTrack" , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/label)\n\n*This API requires the following crate features to be activated: `AudioTrack`*"]
    pub fn label(this: &AudioTrack) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioTrack" , js_name = language ) ]
    #[doc = "Getter for the `language` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/language)\n\n*This API requires the following crate features to be activated: `AudioTrack`*"]
    pub fn language(this: &AudioTrack) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioTrack" , js_name = enabled ) ]
    #[doc = "Getter for the `enabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/enabled)\n\n*This API requires the following crate features to be activated: `AudioTrack`*"]
    pub fn enabled(this: &AudioTrack) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "AudioTrack" , js_name = enabled ) ]
    #[doc = "Setter for the `enabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/enabled)\n\n*This API requires the following crate features to be activated: `AudioTrack`*"]
    pub fn set_enabled(this: &AudioTrack, value: bool);
}
