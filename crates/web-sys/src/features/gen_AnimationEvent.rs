use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = AnimationEvent , typescript_name = AnimationEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent)\n\n*This API requires the following crate features to be activated: `AnimationEvent`*"]
    pub type AnimationEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = animationName ) ]
    #[doc = "Getter for the `animationName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/animationName)\n\n*This API requires the following crate features to be activated: `AnimationEvent`*"]
    pub fn animation_name(this: &AnimationEvent) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = elapsedTime ) ]
    #[doc = "Getter for the `elapsedTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/elapsedTime)\n\n*This API requires the following crate features to be activated: `AnimationEvent`*"]
    pub fn elapsed_time(this: &AnimationEvent) -> f32;
    # [ wasm_bindgen ( structural , method , getter , js_name = pseudoElement ) ]
    #[doc = "Getter for the `pseudoElement` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/pseudoElement)\n\n*This API requires the following crate features to be activated: `AnimationEvent`*"]
    pub fn pseudo_element(this: &AnimationEvent) -> String;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new AnimationEvent(..)` constructor, creating a new instance of `AnimationEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/AnimationEvent)\n\n*This API requires the following crate features to be activated: `AnimationEvent`*"]
    pub fn new(this: &AnimationEvent, type_: &str) -> Result<AnimationEvent, JsValue>;
    #[cfg(feature = "AnimationEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new AnimationEvent(..)` constructor, creating a new instance of `AnimationEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/AnimationEvent)\n\n*This API requires the following crate features to be activated: `AnimationEvent`, `AnimationEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &AnimationEvent,
        type_: &str,
        event_init_dict: &AnimationEventInit,
    ) -> Result<AnimationEvent, JsValue>;
}
