use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = AnimationEvent , typescript_type = "AnimationEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AnimationEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent)
    ///
    ///*This API requires the following crate features to be activated: `AnimationEvent`*
    pub type AnimationEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AnimationEvent" , js_name = animationName ) ]
    ///Getter for the `animationName` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/animationName)
    ///
    ///*This API requires the following crate features to be activated: `AnimationEvent`*
    pub fn animation_name(this: &AnimationEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AnimationEvent" , js_name = elapsedTime ) ]
    ///Getter for the `elapsedTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/elapsedTime)
    ///
    ///*This API requires the following crate features to be activated: `AnimationEvent`*
    pub fn elapsed_time(this: &AnimationEvent) -> f32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AnimationEvent" , js_name = pseudoElement ) ]
    ///Getter for the `pseudoElement` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/pseudoElement)
    ///
    ///*This API requires the following crate features to be activated: `AnimationEvent`*
    pub fn pseudo_element(this: &AnimationEvent) -> String;

    #[wasm_bindgen(catch, constructor, js_class = "AnimationEvent")]
    ///The `new AnimationEvent(..)` constructor, creating a new instance of `AnimationEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/AnimationEvent)
    ///
    ///*This API requires the following crate features to be activated: `AnimationEvent`*
    pub fn new(type_: &str) -> Result<AnimationEvent, JsValue>;

    #[cfg(feature = "AnimationEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "AnimationEvent")]
    ///The `new AnimationEvent(..)` constructor, creating a new instance of `AnimationEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/AnimationEvent)
    ///
    ///*This API requires the following crate features to be activated: `AnimationEvent`, `AnimationEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &AnimationEventInit,
    ) -> Result<AnimationEvent, JsValue>;

}
