use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = DynamicsCompressorNode , typescript_name = DynamicsCompressorNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DynamicsCompressorNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode)
    ///
    ///*This API requires the following crate features to be activated: `DynamicsCompressorNode`*
    pub type DynamicsCompressorNode;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "DynamicsCompressorNode" , js_name = threshold ) ]
    ///Getter for the `threshold` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/threshold)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*
    pub fn threshold(this: &DynamicsCompressorNode) -> AudioParam;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "DynamicsCompressorNode" , js_name = knee ) ]
    ///Getter for the `knee` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/knee)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*
    pub fn knee(this: &DynamicsCompressorNode) -> AudioParam;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "DynamicsCompressorNode" , js_name = ratio ) ]
    ///Getter for the `ratio` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/ratio)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*
    pub fn ratio(this: &DynamicsCompressorNode) -> AudioParam;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DynamicsCompressorNode" , js_name = reduction ) ]
    ///Getter for the `reduction` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/reduction)
    ///
    ///*This API requires the following crate features to be activated: `DynamicsCompressorNode`*
    pub fn reduction(this: &DynamicsCompressorNode) -> f32;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "DynamicsCompressorNode" , js_name = attack ) ]
    ///Getter for the `attack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/attack)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*
    pub fn attack(this: &DynamicsCompressorNode) -> AudioParam;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "DynamicsCompressorNode" , js_name = release ) ]
    ///Getter for the `release` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/release)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*
    pub fn release(this: &DynamicsCompressorNode) -> AudioParam;

    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "DynamicsCompressorNode")]
    ///The `new DynamicsCompressorNode(..)` constructor, creating a new instance of `DynamicsCompressorNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/DynamicsCompressorNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `DynamicsCompressorNode`*
    pub fn new(context: &BaseAudioContext) -> Result<DynamicsCompressorNode, JsValue>;

    #[cfg(all(feature = "BaseAudioContext", feature = "DynamicsCompressorOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "DynamicsCompressorNode")]
    ///The `new DynamicsCompressorNode(..)` constructor, creating a new instance of `DynamicsCompressorNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/DynamicsCompressorNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `DynamicsCompressorNode`, `DynamicsCompressorOptions`*
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &DynamicsCompressorOptions,
    ) -> Result<DynamicsCompressorNode, JsValue>;

}
