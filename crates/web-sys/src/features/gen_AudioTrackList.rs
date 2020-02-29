use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = AudioTrackList , typescript_type = "AudioTrackList" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AudioTrackList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrackList`*
    pub type AudioTrackList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioTrackList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/length)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrackList`*
    pub fn length(this: &AudioTrackList) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioTrackList" , js_name = onchange ) ]
    ///Getter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onchange)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrackList`*
    pub fn onchange(this: &AudioTrackList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "AudioTrackList" , js_name = onchange ) ]
    ///Setter for the `onchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onchange)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrackList`*
    pub fn set_onchange(this: &AudioTrackList, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioTrackList" , js_name = onaddtrack ) ]
    ///Getter for the `onaddtrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onaddtrack)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrackList`*
    pub fn onaddtrack(this: &AudioTrackList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "AudioTrackList" , js_name = onaddtrack ) ]
    ///Setter for the `onaddtrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onaddtrack)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrackList`*
    pub fn set_onaddtrack(this: &AudioTrackList, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioTrackList" , js_name = onremovetrack ) ]
    ///Getter for the `onremovetrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onremovetrack)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrackList`*
    pub fn onremovetrack(this: &AudioTrackList) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "AudioTrackList" , js_name = onremovetrack ) ]
    ///Setter for the `onremovetrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onremovetrack)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrackList`*
    pub fn set_onremovetrack(this: &AudioTrackList, value: Option<&::js_sys::Function>);

    #[cfg(feature = "AudioTrack")]
    # [ wasm_bindgen ( method , structural , js_class = "AudioTrackList" , js_name = getTrackById ) ]
    ///The `getTrackById()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/getTrackById)
    ///
    ///*This API requires the following crate features to be activated: `AudioTrack`, `AudioTrackList`*
    pub fn get_track_by_id(this: &AudioTrackList, id: &str) -> Option<AudioTrack>;

    #[cfg(feature = "AudioTrack")]
    #[wasm_bindgen(method, structural, js_class = "AudioTrackList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `AudioTrack`, `AudioTrackList`*
    pub fn get(this: &AudioTrackList, index: u32) -> Option<AudioTrack>;

}
