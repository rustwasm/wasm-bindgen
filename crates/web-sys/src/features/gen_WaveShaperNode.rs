use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = WaveShaperNode , typescript_name = WaveShaperNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WaveShaperNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode)
    ///
    ///*This API requires the following crate features to be activated: `WaveShaperNode`*
    pub type WaveShaperNode;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WaveShaperNode" , js_name = curve ) ]
    ///Getter for the `curve` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/curve)
    ///
    ///*This API requires the following crate features to be activated: `WaveShaperNode`*
    pub fn curve(this: &WaveShaperNode) -> Option<Vec<f32>>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "WaveShaperNode" , js_name = curve ) ]
    ///Setter for the `curve` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/curve)
    ///
    ///*This API requires the following crate features to be activated: `WaveShaperNode`*
    pub fn set_curve(this: &WaveShaperNode, value: Option<&mut [f32]>);

    #[cfg(feature = "OverSampleType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "WaveShaperNode" , js_name = oversample ) ]
    ///Getter for the `oversample` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/oversample)
    ///
    ///*This API requires the following crate features to be activated: `OverSampleType`, `WaveShaperNode`*
    pub fn oversample(this: &WaveShaperNode) -> OverSampleType;

    #[cfg(feature = "OverSampleType")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "WaveShaperNode" , js_name = oversample ) ]
    ///Setter for the `oversample` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/oversample)
    ///
    ///*This API requires the following crate features to be activated: `OverSampleType`, `WaveShaperNode`*
    pub fn set_oversample(this: &WaveShaperNode, value: OverSampleType);

    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "WaveShaperNode")]
    ///The `new WaveShaperNode(..)` constructor, creating a new instance of `WaveShaperNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/WaveShaperNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `WaveShaperNode`*
    pub fn new(context: &BaseAudioContext) -> Result<WaveShaperNode, JsValue>;

    #[cfg(all(feature = "BaseAudioContext", feature = "WaveShaperOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "WaveShaperNode")]
    ///The `new WaveShaperNode(..)` constructor, creating a new instance of `WaveShaperNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/WaveShaperNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `WaveShaperNode`, `WaveShaperOptions`*
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &WaveShaperOptions,
    ) -> Result<WaveShaperNode, JsValue>;

}
