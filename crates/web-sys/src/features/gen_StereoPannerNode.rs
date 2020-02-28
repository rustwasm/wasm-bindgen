use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = StereoPannerNode , typescript_name = StereoPannerNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StereoPannerNode` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode)\n\n*This API requires the following crate features to be activated: `StereoPannerNode`*"]
    pub type StereoPannerNode;
    # [ wasm_bindgen ( structural , method , getter , js_class = "StereoPannerNode" , js_name = pan ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `pan` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode/pan)\n\n*This API requires the following crate features to be activated: `AudioParam`, `StereoPannerNode`*"]
    pub fn pan(this: &StereoPannerNode) -> AudioParam;
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, js_class = "StereoPannerNode", constructor)]
    #[doc = "The `new StereoPannerNode(..)` constructor, creating a new instance of `StereoPannerNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode/StereoPannerNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `StereoPannerNode`*"]
    pub fn new(
        this: &StereoPannerNode,
        context: &BaseAudioContext,
    ) -> Result<StereoPannerNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "StereoPannerOptions",))]
    #[wasm_bindgen(catch, js_class = "StereoPannerNode", constructor)]
    #[doc = "The `new StereoPannerNode(..)` constructor, creating a new instance of `StereoPannerNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode/StereoPannerNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `StereoPannerNode`, `StereoPannerOptions`*"]
    pub fn new_with_options(
        this: &StereoPannerNode,
        context: &BaseAudioContext,
        options: &StereoPannerOptions,
    ) -> Result<StereoPannerNode, JsValue>;
}
