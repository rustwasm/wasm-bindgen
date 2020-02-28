use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaStream , typescript_name = MediaStream ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
    pub type MediaStream;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStream" , js_name = id ) ]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
    pub fn id(this: &MediaStream) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStream" , js_name = active ) ]
    #[doc = "Getter for the `active` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/active)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
    pub fn active(this: &MediaStream) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStream" , js_name = onaddtrack ) ]
    #[doc = "Getter for the `onaddtrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onaddtrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
    pub fn onaddtrack(this: &MediaStream) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaStream" , js_name = onaddtrack ) ]
    #[doc = "Setter for the `onaddtrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onaddtrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
    pub fn set_onaddtrack(this: &MediaStream, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStream" , js_name = onremovetrack ) ]
    #[doc = "Getter for the `onremovetrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onremovetrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
    pub fn onremovetrack(this: &MediaStream) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaStream" , js_name = onremovetrack ) ]
    #[doc = "Setter for the `onremovetrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onremovetrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
    pub fn set_onremovetrack(this: &MediaStream, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStream" , js_name = currentTime ) ]
    #[doc = "Getter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
    pub fn current_time(this: &MediaStream) -> f64;
    #[wasm_bindgen(catch, js_class = "MediaStream", constructor)]
    #[doc = "The `new MediaStream(..)` constructor, creating a new instance of `MediaStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/MediaStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
    pub fn new(this: &MediaStream) -> Result<MediaStream, JsValue>;
    #[wasm_bindgen(catch, js_class = "MediaStream", constructor)]
    #[doc = "The `new MediaStream(..)` constructor, creating a new instance of `MediaStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/MediaStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
    pub fn new_with_stream(
        this: &MediaStream,
        stream: &MediaStream,
    ) -> Result<MediaStream, JsValue>;
    #[wasm_bindgen(catch, js_class = "MediaStream", constructor)]
    #[doc = "The `new MediaStream(..)` constructor, creating a new instance of `MediaStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/MediaStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
    pub fn new_with_tracks(
        this: &MediaStream,
        tracks: &::wasm_bindgen::JsValue,
    ) -> Result<MediaStream, JsValue>;
    #[cfg(feature = "MediaStreamTrack")]
    # [ wasm_bindgen ( method , structural , js_class = "MediaStream" , js_name = addTrack ) ]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`*"]
    pub fn add_track(this: &MediaStream, track: &MediaStreamTrack);
    # [ wasm_bindgen ( method , structural , js_class = "MediaStream" , js_name = clone ) ]
    #[doc = "The `clone()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/clone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
    pub fn clone(this: &MediaStream) -> MediaStream;
    # [ wasm_bindgen ( method , structural , js_class = "MediaStream" , js_name = getAudioTracks ) ]
    #[doc = "The `getAudioTracks()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getAudioTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
    pub fn get_audio_tracks(this: &MediaStream) -> ::js_sys::Array;
    #[cfg(feature = "MediaStreamTrack")]
    # [ wasm_bindgen ( method , structural , js_class = "MediaStream" , js_name = getTrackById ) ]
    #[doc = "The `getTrackById()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getTrackById)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`*"]
    pub fn get_track_by_id(this: &MediaStream, track_id: &str) -> Option<MediaStreamTrack>;
    # [ wasm_bindgen ( method , structural , js_class = "MediaStream" , js_name = getTracks ) ]
    #[doc = "The `getTracks()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
    pub fn get_tracks(this: &MediaStream) -> ::js_sys::Array;
    # [ wasm_bindgen ( method , structural , js_class = "MediaStream" , js_name = getVideoTracks ) ]
    #[doc = "The `getVideoTracks()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getVideoTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
    pub fn get_video_tracks(this: &MediaStream) -> ::js_sys::Array;
    #[cfg(feature = "MediaStreamTrack")]
    # [ wasm_bindgen ( method , structural , js_class = "MediaStream" , js_name = removeTrack ) ]
    #[doc = "The `removeTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/removeTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`*"]
    pub fn remove_track(this: &MediaStream, track: &MediaStreamTrack);
}
