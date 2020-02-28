use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = DynamicsCompressorNode , typescript_name = DynamicsCompressorNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DynamicsCompressorNode` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode)\n\n*This API requires the following crate features to be activated: `DynamicsCompressorNode`*"]
    pub type DynamicsCompressorNode;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DynamicsCompressorNode" , js_name = threshold ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `threshold` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/threshold)\n\n*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
    pub fn threshold(this: &DynamicsCompressorNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DynamicsCompressorNode" , js_name = knee ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `knee` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/knee)\n\n*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
    pub fn knee(this: &DynamicsCompressorNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DynamicsCompressorNode" , js_name = ratio ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `ratio` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/ratio)\n\n*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
    pub fn ratio(this: &DynamicsCompressorNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DynamicsCompressorNode" , js_name = reduction ) ]
    #[doc = "Getter for the `reduction` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/reduction)\n\n*This API requires the following crate features to be activated: `DynamicsCompressorNode`*"]
    pub fn reduction(this: &DynamicsCompressorNode) -> f32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DynamicsCompressorNode" , js_name = attack ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `attack` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/attack)\n\n*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
    pub fn attack(this: &DynamicsCompressorNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DynamicsCompressorNode" , js_name = release ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `release` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/release)\n\n*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
    pub fn release(this: &DynamicsCompressorNode) -> AudioParam;
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, js_class = "DynamicsCompressorNode", constructor)]
    #[doc = "The `new DynamicsCompressorNode(..)` constructor, creating a new instance of `DynamicsCompressorNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/DynamicsCompressorNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `DynamicsCompressorNode`*"]
    pub fn new(
        this: &DynamicsCompressorNode,
        context: &BaseAudioContext,
    ) -> Result<DynamicsCompressorNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "DynamicsCompressorOptions",))]
    #[wasm_bindgen(catch, js_class = "DynamicsCompressorNode", constructor)]
    #[doc = "The `new DynamicsCompressorNode(..)` constructor, creating a new instance of `DynamicsCompressorNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/DynamicsCompressorNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `DynamicsCompressorNode`, `DynamicsCompressorOptions`*"]
    pub fn new_with_options(
        this: &DynamicsCompressorNode,
        context: &BaseAudioContext,
        options: &DynamicsCompressorOptions,
    ) -> Result<DynamicsCompressorNode, JsValue>;
}
