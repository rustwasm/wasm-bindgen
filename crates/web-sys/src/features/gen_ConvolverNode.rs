use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = ConvolverNode , typescript_name = ConvolverNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConvolverNode` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode)\n\n*This API requires the following crate features to be activated: `ConvolverNode`*"]
    pub type ConvolverNode;
    # [ wasm_bindgen ( structural , method , getter , js_name = buffer ) ]
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Getter for the `buffer` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/buffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `ConvolverNode`*"]
    pub fn buffer(this: &ConvolverNode) -> Option<AudioBuffer>;
    # [ wasm_bindgen ( structural , method , setter , js_name = buffer ) ]
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Setter for the `buffer` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/buffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `ConvolverNode`*"]
    pub fn set_buffer(this: &ConvolverNode, value: Option<&AudioBuffer>);
    # [ wasm_bindgen ( structural , method , getter , js_name = normalize ) ]
    #[doc = "Getter for the `normalize` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/normalize)\n\n*This API requires the following crate features to be activated: `ConvolverNode`*"]
    pub fn normalize(this: &ConvolverNode) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = normalize ) ]
    #[doc = "Setter for the `normalize` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/normalize)\n\n*This API requires the following crate features to be activated: `ConvolverNode`*"]
    pub fn set_normalize(this: &ConvolverNode, value: bool);
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new ConvolverNode(..)` constructor, creating a new instance of `ConvolverNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/ConvolverNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ConvolverNode`*"]
    pub fn new(this: &ConvolverNode, context: &BaseAudioContext) -> Result<ConvolverNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "ConvolverOptions",))]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new ConvolverNode(..)` constructor, creating a new instance of `ConvolverNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/ConvolverNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ConvolverNode`, `ConvolverOptions`*"]
    pub fn new_with_options(
        this: &ConvolverNode,
        context: &BaseAudioContext,
        options: &ConvolverOptions,
    ) -> Result<ConvolverNode, JsValue>;
}
