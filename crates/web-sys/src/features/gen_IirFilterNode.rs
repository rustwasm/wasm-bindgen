use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = IIRFilterNode , typescript_name = IIRFilterNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IirFilterNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterNode`*"]
    pub type IirFilterNode;
    #[cfg(all(feature = "BaseAudioContext", feature = "IirFilterOptions",))]
    #[wasm_bindgen(catch, js_class = "IIRFilterNode", constructor)]
    #[doc = "The `new IirFilterNode(..)` constructor, creating a new instance of `IirFilterNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode/IIRFilterNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `IirFilterNode`, `IirFilterOptions`*"]
    pub fn new(
        this: &IirFilterNode,
        context: &BaseAudioContext,
        options: &IirFilterOptions,
    ) -> Result<IirFilterNode, JsValue>;
    # [ wasm_bindgen ( method , structural , js_class = "IIRFilterNode" , js_name = getFrequencyResponse ) ]
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterNode`*"]
    pub fn get_frequency_response(
        this: &IirFilterNode,
        frequency_hz: &mut [f32],
        mag_response: &mut [f32],
        phase_response: &mut [f32],
    );
}
