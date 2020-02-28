use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = AudioTrackList , typescript_name = AudioTrackList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioTrackList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList)\n\n*This API requires the following crate features to be activated: `AudioTrackList`*"]
    pub type AudioTrackList;
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/length)\n\n*This API requires the following crate features to be activated: `AudioTrackList`*"]
    pub fn length(this: &AudioTrackList) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = onchange ) ]
    #[doc = "Getter for the `onchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onchange)\n\n*This API requires the following crate features to be activated: `AudioTrackList`*"]
    pub fn onchange(this: &AudioTrackList) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onchange ) ]
    #[doc = "Setter for the `onchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onchange)\n\n*This API requires the following crate features to be activated: `AudioTrackList`*"]
    pub fn set_onchange(this: &AudioTrackList, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onaddtrack ) ]
    #[doc = "Getter for the `onaddtrack` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onaddtrack)\n\n*This API requires the following crate features to be activated: `AudioTrackList`*"]
    pub fn onaddtrack(this: &AudioTrackList) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onaddtrack ) ]
    #[doc = "Setter for the `onaddtrack` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onaddtrack)\n\n*This API requires the following crate features to be activated: `AudioTrackList`*"]
    pub fn set_onaddtrack(this: &AudioTrackList, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onremovetrack ) ]
    #[doc = "Getter for the `onremovetrack` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onremovetrack)\n\n*This API requires the following crate features to be activated: `AudioTrackList`*"]
    pub fn onremovetrack(this: &AudioTrackList) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onremovetrack ) ]
    #[doc = "Setter for the `onremovetrack` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onremovetrack)\n\n*This API requires the following crate features to be activated: `AudioTrackList`*"]
    pub fn set_onremovetrack(this: &AudioTrackList, value: Option<&::js_sys::Function>);
    #[cfg(feature = "AudioTrack")]
    # [ wasm_bindgen ( method , structural , js_name = getTrackById ) ]
    #[doc = "The `getTrackById()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/getTrackById)\n\n*This API requires the following crate features to be activated: `AudioTrack`, `AudioTrackList`*"]
    pub fn get_track_by_id(this: &AudioTrackList, id: &str) -> Option<AudioTrack>;
    #[cfg(feature = "AudioTrack")]
    #[wasm_bindgen(method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `AudioTrack`, `AudioTrackList`*"]
    pub fn get(this: &AudioTrackList, index: u32) -> Option<AudioTrack>;
}
