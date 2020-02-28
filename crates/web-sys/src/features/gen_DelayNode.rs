use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = DelayNode , typescript_name = DelayNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DelayNode` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode)\n\n*This API requires the following crate features to be activated: `DelayNode`*"]
    pub type DelayNode;
    # [ wasm_bindgen ( structural , method , getter , js_name = delayTime ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `delayTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode/delayTime)\n\n*This API requires the following crate features to be activated: `AudioParam`, `DelayNode`*"]
    pub fn delay_time(this: &DelayNode) -> AudioParam;
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new DelayNode(..)` constructor, creating a new instance of `DelayNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode/DelayNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `DelayNode`*"]
    pub fn new(this: &DelayNode, context: &BaseAudioContext) -> Result<DelayNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "DelayOptions",))]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new DelayNode(..)` constructor, creating a new instance of `DelayNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode/DelayNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `DelayNode`, `DelayOptions`*"]
    pub fn new_with_options(
        this: &DelayNode,
        context: &BaseAudioContext,
        options: &DelayOptions,
    ) -> Result<DelayNode, JsValue>;
}
