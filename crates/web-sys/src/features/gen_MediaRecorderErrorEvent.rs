use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MediaRecorderErrorEvent , typescript_type = "MediaRecorderErrorEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaRecorderErrorEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorderErrorEvent)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorderErrorEvent`*
    pub type MediaRecorderErrorEvent;

    #[cfg(feature = "DomException")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaRecorderErrorEvent" , js_name = error ) ]
    ///Getter for the `error` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorderErrorEvent/error)
    ///
    ///*This API requires the following crate features to be activated: `DomException`, `MediaRecorderErrorEvent`*
    pub fn error(this: &MediaRecorderErrorEvent) -> DomException;

    #[cfg(feature = "MediaRecorderErrorEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaRecorderErrorEvent")]
    ///The `new MediaRecorderErrorEvent(..)` constructor, creating a new instance of `MediaRecorderErrorEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorderErrorEvent/MediaRecorderErrorEvent)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorderErrorEvent`, `MediaRecorderErrorEventInit`*
    pub fn new(
        type_: &str,
        event_init_dict: &MediaRecorderErrorEventInit,
    ) -> Result<MediaRecorderErrorEvent, JsValue>;

}
