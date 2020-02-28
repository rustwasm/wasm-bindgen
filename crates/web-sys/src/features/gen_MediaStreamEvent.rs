use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MediaStreamEvent , typescript_name = MediaStreamEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamEvent)\n\n*This API requires the following crate features to be activated: `MediaStreamEvent`*"]
    pub type MediaStreamEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = stream ) ]
    #[cfg(feature = "MediaStream")]
    #[doc = "Getter for the `stream` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamEvent/stream)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamEvent`*"]
    pub fn stream(this: &MediaStreamEvent) -> Option<MediaStream>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new MediaStreamEvent(..)` constructor, creating a new instance of `MediaStreamEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamEvent/MediaStreamEvent)\n\n*This API requires the following crate features to be activated: `MediaStreamEvent`*"]
    pub fn new(this: &MediaStreamEvent, type_: &str) -> Result<MediaStreamEvent, JsValue>;
    #[cfg(feature = "MediaStreamEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new MediaStreamEvent(..)` constructor, creating a new instance of `MediaStreamEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamEvent/MediaStreamEvent)\n\n*This API requires the following crate features to be activated: `MediaStreamEvent`, `MediaStreamEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &MediaStreamEvent,
        type_: &str,
        event_init_dict: &MediaStreamEventInit,
    ) -> Result<MediaStreamEvent, JsValue>;
}
