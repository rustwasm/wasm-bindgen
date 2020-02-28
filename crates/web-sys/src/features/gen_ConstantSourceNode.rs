use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioScheduledSourceNode , extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = ConstantSourceNode , typescript_name = ConstantSourceNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstantSourceNode` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode)\n\n*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    pub type ConstantSourceNode;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ConstantSourceNode" , js_name = offset ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `offset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/offset)\n\n*This API requires the following crate features to be activated: `AudioParam`, `ConstantSourceNode`*"]
    pub fn offset(this: &ConstantSourceNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ConstantSourceNode" , js_name = onended ) ]
    #[doc = "Getter for the `onended` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/onended)\n\n*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    pub fn onended(this: &ConstantSourceNode) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "ConstantSourceNode" , js_name = onended ) ]
    #[doc = "Setter for the `onended` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/onended)\n\n*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    pub fn set_onended(this: &ConstantSourceNode, value: Option<&::js_sys::Function>);
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, js_class = "ConstantSourceNode", constructor)]
    #[doc = "The `new ConstantSourceNode(..)` constructor, creating a new instance of `ConstantSourceNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/ConstantSourceNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ConstantSourceNode`*"]
    pub fn new(
        this: &ConstantSourceNode,
        context: &BaseAudioContext,
    ) -> Result<ConstantSourceNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "ConstantSourceOptions",))]
    #[wasm_bindgen(catch, js_class = "ConstantSourceNode", constructor)]
    #[doc = "The `new ConstantSourceNode(..)` constructor, creating a new instance of `ConstantSourceNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/ConstantSourceNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ConstantSourceNode`, `ConstantSourceOptions`*"]
    pub fn new_with_options(
        this: &ConstantSourceNode,
        context: &BaseAudioContext,
        options: &ConstantSourceOptions,
    ) -> Result<ConstantSourceNode, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "ConstantSourceNode" , js_name = start ) ]
    #[doc = "The `start()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/start)\n\n*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    pub fn start(this: &ConstantSourceNode) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "ConstantSourceNode" , js_name = start ) ]
    #[doc = "The `start()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/start)\n\n*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    pub fn start_with_when(this: &ConstantSourceNode, when: f64) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "ConstantSourceNode" , js_name = stop ) ]
    #[doc = "The `stop()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/stop)\n\n*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    pub fn stop(this: &ConstantSourceNode) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "ConstantSourceNode" , js_name = stop ) ]
    #[doc = "The `stop()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/stop)\n\n*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    pub fn stop_with_when(this: &ConstantSourceNode, when: f64) -> Result<(), JsValue>;
}
