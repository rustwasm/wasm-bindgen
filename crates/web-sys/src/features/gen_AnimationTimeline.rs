use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AnimationTimeline , typescript_name = AnimationTimeline ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationTimeline` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTimeline)\n\n*This API requires the following crate features to be activated: `AnimationTimeline`*"]
    pub type AnimationTimeline;
    # [ wasm_bindgen ( structural , method , getter , js_name = currentTime ) ]
    #[doc = "Getter for the `currentTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTimeline/currentTime)\n\n*This API requires the following crate features to be activated: `AnimationTimeline`*"]
    pub fn current_time(this: &AnimationTimeline) -> Option<f64>;
}
