use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = AnimationPlaybackEvent , typescript_name = AnimationPlaybackEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationPlaybackEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEvent`*"]
    pub type AnimationPlaybackEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "AnimationPlaybackEvent" , js_name = currentTime ) ]
    #[doc = "Getter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEvent`*"]
    pub fn current_time(this: &AnimationPlaybackEvent) -> Option<f64>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "AnimationPlaybackEvent" , js_name = timelineTime ) ]
    #[doc = "Getter for the `timelineTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/timelineTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEvent`*"]
    pub fn timeline_time(this: &AnimationPlaybackEvent) -> Option<f64>;
    #[wasm_bindgen(catch, js_class = "AnimationPlaybackEvent", constructor)]
    #[doc = "The `new AnimationPlaybackEvent(..)` constructor, creating a new instance of `AnimationPlaybackEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/AnimationPlaybackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEvent`*"]
    pub fn new(
        this: &AnimationPlaybackEvent,
        type_: &str,
    ) -> Result<AnimationPlaybackEvent, JsValue>;
    #[cfg(feature = "AnimationPlaybackEventInit")]
    #[wasm_bindgen(catch, js_class = "AnimationPlaybackEvent", constructor)]
    #[doc = "The `new AnimationPlaybackEvent(..)` constructor, creating a new instance of `AnimationPlaybackEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/AnimationPlaybackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEvent`, `AnimationPlaybackEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &AnimationPlaybackEvent,
        type_: &str,
        event_init_dict: &AnimationPlaybackEventInit,
    ) -> Result<AnimationPlaybackEvent, JsValue>;
}
