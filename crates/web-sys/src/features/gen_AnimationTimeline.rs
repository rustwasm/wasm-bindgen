use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AnimationTimeline , typescript_name = AnimationTimeline ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AnimationTimeline` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTimeline)
    ///
    ///*This API requires the following crate features to be activated: `AnimationTimeline`*
    pub type AnimationTimeline;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AnimationTimeline" , js_name = currentTime ) ]
    ///Getter for the `currentTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTimeline/currentTime)
    ///
    ///*This API requires the following crate features to be activated: `AnimationTimeline`*
    pub fn current_time(this: &AnimationTimeline) -> Option<f64>;

}
