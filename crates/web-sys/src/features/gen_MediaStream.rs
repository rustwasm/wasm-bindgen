use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaStream , typescript_type = "MediaStream" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaStream` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`*
    pub type MediaStream;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStream" , js_name = id ) ]
    ///Getter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/id)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`*
    pub fn id(this: &MediaStream) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStream" , js_name = active ) ]
    ///Getter for the `active` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/active)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`*
    pub fn active(this: &MediaStream) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStream" , js_name = onaddtrack ) ]
    ///Getter for the `onaddtrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onaddtrack)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`*
    pub fn onaddtrack(this: &MediaStream) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaStream" , js_name = onaddtrack ) ]
    ///Setter for the `onaddtrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onaddtrack)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`*
    pub fn set_onaddtrack(this: &MediaStream, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStream" , js_name = onremovetrack ) ]
    ///Getter for the `onremovetrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onremovetrack)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`*
    pub fn onremovetrack(this: &MediaStream) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaStream" , js_name = onremovetrack ) ]
    ///Setter for the `onremovetrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onremovetrack)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`*
    pub fn set_onremovetrack(this: &MediaStream, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStream" , js_name = currentTime ) ]
    ///Getter for the `currentTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/currentTime)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`*
    pub fn current_time(this: &MediaStream) -> f64;

    #[wasm_bindgen(catch, constructor, js_class = "MediaStream")]
    ///The `new MediaStream(..)` constructor, creating a new instance of `MediaStream`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/MediaStream)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`*
    pub fn new() -> Result<MediaStream, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "MediaStream")]
    ///The `new MediaStream(..)` constructor, creating a new instance of `MediaStream`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/MediaStream)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`*
    pub fn new_with_stream(stream: &MediaStream) -> Result<MediaStream, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "MediaStream")]
    ///The `new MediaStream(..)` constructor, creating a new instance of `MediaStream`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/MediaStream)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`*
    pub fn new_with_tracks(tracks: &::wasm_bindgen::JsValue) -> Result<MediaStream, JsValue>;

    #[cfg(feature = "MediaStreamTrack")]
    # [ wasm_bindgen ( method , structural , js_class = "MediaStream" , js_name = addTrack ) ]
    ///The `addTrack()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/addTrack)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`*
    pub fn add_track(this: &MediaStream, track: &MediaStreamTrack);

    # [ wasm_bindgen ( method , structural , js_class = "MediaStream" , js_name = clone ) ]
    ///The `clone()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/clone)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`*
    pub fn clone(this: &MediaStream) -> MediaStream;

    # [ wasm_bindgen ( method , structural , js_class = "MediaStream" , js_name = getAudioTracks ) ]
    ///The `getAudioTracks()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getAudioTracks)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`*
    pub fn get_audio_tracks(this: &MediaStream) -> ::js_sys::Array;

    #[cfg(feature = "MediaStreamTrack")]
    # [ wasm_bindgen ( method , structural , js_class = "MediaStream" , js_name = getTrackById ) ]
    ///The `getTrackById()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getTrackById)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`*
    pub fn get_track_by_id(this: &MediaStream, track_id: &str) -> Option<MediaStreamTrack>;

    # [ wasm_bindgen ( method , structural , js_class = "MediaStream" , js_name = getTracks ) ]
    ///The `getTracks()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getTracks)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`*
    pub fn get_tracks(this: &MediaStream) -> ::js_sys::Array;

    # [ wasm_bindgen ( method , structural , js_class = "MediaStream" , js_name = getVideoTracks ) ]
    ///The `getVideoTracks()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getVideoTracks)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`*
    pub fn get_video_tracks(this: &MediaStream) -> ::js_sys::Array;

    #[cfg(feature = "MediaStreamTrack")]
    # [ wasm_bindgen ( method , structural , js_class = "MediaStream" , js_name = removeTrack ) ]
    ///The `removeTrack()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/removeTrack)
    ///
    ///*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`*
    pub fn remove_track(this: &MediaStream, track: &MediaStreamTrack);

}
