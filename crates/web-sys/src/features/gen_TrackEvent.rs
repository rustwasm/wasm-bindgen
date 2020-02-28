use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = TrackEvent , typescript_name = TrackEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TrackEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TrackEvent)\n\n*This API requires the following crate features to be activated: `TrackEvent`*"]
    pub type TrackEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = track ) ]
    #[doc = "Getter for the `track` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TrackEvent/track)\n\n*This API requires the following crate features to be activated: `TrackEvent`*"]
    pub fn track(this: &TrackEvent) -> Option<::js_sys::Object>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TrackEvent(..)` constructor, creating a new instance of `TrackEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TrackEvent/TrackEvent)\n\n*This API requires the following crate features to be activated: `TrackEvent`*"]
    pub fn new(this: &TrackEvent, type_: &str) -> Result<TrackEvent, JsValue>;
    #[cfg(feature = "TrackEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TrackEvent(..)` constructor, creating a new instance of `TrackEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TrackEvent/TrackEvent)\n\n*This API requires the following crate features to be activated: `TrackEvent`, `TrackEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &TrackEvent,
        type_: &str,
        event_init_dict: &TrackEventInit,
    ) -> Result<TrackEvent, JsValue>;
}
