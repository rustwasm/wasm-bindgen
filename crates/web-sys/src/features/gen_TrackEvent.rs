use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = TrackEvent , typescript_type = "TrackEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TrackEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TrackEvent)
    ///
    ///*This API requires the following crate features to be activated: `TrackEvent`*
    pub type TrackEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TrackEvent" , js_name = track ) ]
    ///Getter for the `track` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TrackEvent/track)
    ///
    ///*This API requires the following crate features to be activated: `TrackEvent`*
    pub fn track(this: &TrackEvent) -> Option<::js_sys::Object>;

    #[wasm_bindgen(catch, constructor, js_class = "TrackEvent")]
    ///The `new TrackEvent(..)` constructor, creating a new instance of `TrackEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TrackEvent/TrackEvent)
    ///
    ///*This API requires the following crate features to be activated: `TrackEvent`*
    pub fn new(type_: &str) -> Result<TrackEvent, JsValue>;

    #[cfg(feature = "TrackEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "TrackEvent")]
    ///The `new TrackEvent(..)` constructor, creating a new instance of `TrackEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TrackEvent/TrackEvent)
    ///
    ///*This API requires the following crate features to be activated: `TrackEvent`, `TrackEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &TrackEventInit,
    ) -> Result<TrackEvent, JsValue>;

}
