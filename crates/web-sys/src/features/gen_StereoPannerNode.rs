use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = StereoPannerNode , typescript_name = StereoPannerNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `StereoPannerNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode)
    ///
    ///*This API requires the following crate features to be activated: `StereoPannerNode`*
    pub type StereoPannerNode;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "StereoPannerNode" , js_name = pan ) ]
    ///Getter for the `pan` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode/pan)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `StereoPannerNode`*
    pub fn pan(this: &StereoPannerNode) -> AudioParam;

    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "StereoPannerNode")]
    ///The `new StereoPannerNode(..)` constructor, creating a new instance of `StereoPannerNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode/StereoPannerNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `StereoPannerNode`*
    pub fn new(context: &BaseAudioContext) -> Result<StereoPannerNode, JsValue>;

    #[cfg(all(feature = "BaseAudioContext", feature = "StereoPannerOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "StereoPannerNode")]
    ///The `new StereoPannerNode(..)` constructor, creating a new instance of `StereoPannerNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode/StereoPannerNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `StereoPannerNode`, `StereoPannerOptions`*
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &StereoPannerOptions,
    ) -> Result<StereoPannerNode, JsValue>;

}
