use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MediaStreamTrackEvent , typescript_type = "MediaStreamTrackEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaStreamTrackEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackEvent)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrackEvent`*
    pub type MediaStreamTrackEvent;

    #[cfg(feature = "MediaStreamTrack")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamTrackEvent" , js_name = track ) ]
    ///Getter for the `track` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackEvent/track)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackEvent`*
    pub fn track(this: &MediaStreamTrackEvent) -> MediaStreamTrack;

    #[cfg(feature = "MediaStreamTrackEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaStreamTrackEvent")]
    ///The `new MediaStreamTrackEvent(..)` constructor, creating a new instance of `MediaStreamTrackEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackEvent/MediaStreamTrackEvent)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrackEvent`, `MediaStreamTrackEventInit`*
    pub fn new(
        type_: &str,
        event_init_dict: &MediaStreamTrackEventInit,
    ) -> Result<MediaStreamTrackEvent, JsValue>;

}
