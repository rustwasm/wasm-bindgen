use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MediaRecorderErrorEvent , typescript_name = MediaRecorderErrorEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaRecorderErrorEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorderErrorEvent)\n\n*This API requires the following crate features to be activated: `MediaRecorderErrorEvent`*"]
    pub type MediaRecorderErrorEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = error ) ]
    #[cfg(feature = "DomException")]
    #[doc = "Getter for the `error` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorderErrorEvent/error)\n\n*This API requires the following crate features to be activated: `DomException`, `MediaRecorderErrorEvent`*"]
    pub fn error(this: &MediaRecorderErrorEvent) -> DomException;
    #[cfg(feature = "MediaRecorderErrorEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new MediaRecorderErrorEvent(..)` constructor, creating a new instance of `MediaRecorderErrorEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorderErrorEvent/MediaRecorderErrorEvent)\n\n*This API requires the following crate features to be activated: `MediaRecorderErrorEvent`, `MediaRecorderErrorEventInit`*"]
    pub fn new(
        this: &MediaRecorderErrorEvent,
        type_: &str,
        event_init_dict: &MediaRecorderErrorEventInit,
    ) -> Result<MediaRecorderErrorEvent, JsValue>;
}
