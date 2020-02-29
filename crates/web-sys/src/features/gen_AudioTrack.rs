use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AudioTrack , typescript_type = "AudioTrack" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AudioTrack` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrack`*
    pub type AudioTrack;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioTrack" , js_name = id ) ]
    ///Getter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/id)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrack`*
    pub fn id(this: &AudioTrack) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioTrack" , js_name = kind ) ]
    ///Getter for the `kind` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/kind)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrack`*
    pub fn kind(this: &AudioTrack) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioTrack" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/label)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrack`*
    pub fn label(this: &AudioTrack) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioTrack" , js_name = language ) ]
    ///Getter for the `language` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/language)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrack`*
    pub fn language(this: &AudioTrack) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioTrack" , js_name = enabled ) ]
    ///Getter for the `enabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/enabled)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrack`*
    pub fn enabled(this: &AudioTrack) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "AudioTrack" , js_name = enabled ) ]
    ///Setter for the `enabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/enabled)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrack`*
    pub fn set_enabled(this: &AudioTrack, value: bool);

}
