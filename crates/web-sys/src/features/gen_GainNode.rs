use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = GainNode , typescript_name = GainNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GainNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GainNode`*"]
    pub type GainNode;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GainNode" , js_name = gain ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `gain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode/gain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `GainNode`*"]
    pub fn gain(this: &GainNode) -> AudioParam;
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, js_class = "GainNode", constructor)]
    #[doc = "The `new GainNode(..)` constructor, creating a new instance of `GainNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode/GainNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `GainNode`*"]
    pub fn new(this: &GainNode, context: &BaseAudioContext) -> Result<GainNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "GainOptions",))]
    #[wasm_bindgen(catch, js_class = "GainNode", constructor)]
    #[doc = "The `new GainNode(..)` constructor, creating a new instance of `GainNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode/GainNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `GainNode`, `GainOptions`*"]
    pub fn new_with_options(
        this: &GainNode,
        context: &BaseAudioContext,
        options: &GainOptions,
    ) -> Result<GainNode, JsValue>;
}
