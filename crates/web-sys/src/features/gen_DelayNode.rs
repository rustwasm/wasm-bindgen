use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = DelayNode , typescript_name = DelayNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DelayNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode)
    ///
    ///*This API requires the following crate features to be activated: `DelayNode`*
    pub type DelayNode;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "DelayNode" , js_name = delayTime ) ]
    ///Getter for the `delayTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode/delayTime)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `DelayNode`*
    pub fn delay_time(this: &DelayNode) -> AudioParam;

    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "DelayNode")]
    ///The `new DelayNode(..)` constructor, creating a new instance of `DelayNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode/DelayNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `DelayNode`*
    pub fn new(context: &BaseAudioContext) -> Result<DelayNode, JsValue>;

    #[cfg(all(feature = "BaseAudioContext", feature = "DelayOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "DelayNode")]
    ///The `new DelayNode(..)` constructor, creating a new instance of `DelayNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode/DelayNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `DelayNode`, `DelayOptions`*
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &DelayOptions,
    ) -> Result<DelayNode, JsValue>;

}
