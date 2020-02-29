use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = AnimationPlaybackEvent , typescript_type = "AnimationPlaybackEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AnimationPlaybackEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent)
    ///
    ///*This API requires the following crate features to be activated: `AnimationPlaybackEvent`*
    pub type AnimationPlaybackEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AnimationPlaybackEvent" , js_name = currentTime ) ]
    ///Getter for the `currentTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/currentTime)
    ///
    ///*This API requires the following crate features to be activated: `AnimationPlaybackEvent`*
    pub fn current_time(this: &AnimationPlaybackEvent) -> Option<f64>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AnimationPlaybackEvent" , js_name = timelineTime ) ]
    ///Getter for the `timelineTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/timelineTime)
    ///
    ///*This API requires the following crate features to be activated: `AnimationPlaybackEvent`*
    pub fn timeline_time(this: &AnimationPlaybackEvent) -> Option<f64>;

    #[wasm_bindgen(catch, constructor, js_class = "AnimationPlaybackEvent")]
    ///The `new AnimationPlaybackEvent(..)` constructor, creating a new instance of `AnimationPlaybackEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/AnimationPlaybackEvent)
    ///
    ///*This API requires the following crate features to be activated: `AnimationPlaybackEvent`*
    pub fn new(type_: &str) -> Result<AnimationPlaybackEvent, JsValue>;

    #[cfg(feature = "AnimationPlaybackEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "AnimationPlaybackEvent")]
    ///The `new AnimationPlaybackEvent(..)` constructor, creating a new instance of `AnimationPlaybackEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/AnimationPlaybackEvent)
    ///
    ///*This API requires the following crate features to be activated: `AnimationPlaybackEvent`, `AnimationPlaybackEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &AnimationPlaybackEventInit,
    ) -> Result<AnimationPlaybackEvent, JsValue>;

}
