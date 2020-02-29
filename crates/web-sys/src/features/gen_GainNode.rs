use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = GainNode , typescript_type = "GainNode" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GainNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode)
    ///
    ///*This API requires the following crate features to be activated: `GainNode`*
    pub type GainNode;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GainNode" , js_name = gain ) ]
    ///Getter for the `gain` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode/gain)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `GainNode`*
    pub fn gain(this: &GainNode) -> AudioParam;

    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "GainNode")]
    ///The `new GainNode(..)` constructor, creating a new instance of `GainNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode/GainNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `GainNode`*
    pub fn new(context: &BaseAudioContext) -> Result<GainNode, JsValue>;

    #[cfg(all(feature = "BaseAudioContext", feature = "GainOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "GainNode")]
    ///The `new GainNode(..)` constructor, creating a new instance of `GainNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode/GainNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `GainNode`, `GainOptions`*
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &GainOptions,
    ) -> Result<GainNode, JsValue>;

}
