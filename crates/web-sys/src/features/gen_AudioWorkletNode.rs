use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = AudioWorkletNode , typescript_type = "AudioWorkletNode" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AudioWorkletNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode)
    ///
    ///*This API requires the following crate features to be activated: `AudioWorkletNode`*
    pub type AudioWorkletNode;

    #[cfg(feature = "AudioParamMap")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "AudioWorkletNode" , js_name = parameters ) ]
    ///Getter for the `parameters` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/parameters)
    ///
    ///*This API requires the following crate features to be activated: `AudioParamMap`, `AudioWorkletNode`*
    pub fn parameters(this: &AudioWorkletNode) -> Result<AudioParamMap, JsValue>;

    #[cfg(feature = "MessagePort")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "AudioWorkletNode" , js_name = port ) ]
    ///Getter for the `port` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/port)
    ///
    ///*This API requires the following crate features to be activated: `AudioWorkletNode`, `MessagePort`*
    pub fn port(this: &AudioWorkletNode) -> Result<MessagePort, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioWorkletNode" , js_name = onprocessorerror ) ]
    ///Getter for the `onprocessorerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/onprocessorerror)
    ///
    ///*This API requires the following crate features to be activated: `AudioWorkletNode`*
    pub fn onprocessorerror(this: &AudioWorkletNode) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "AudioWorkletNode" , js_name = onprocessorerror ) ]
    ///Setter for the `onprocessorerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/onprocessorerror)
    ///
    ///*This API requires the following crate features to be activated: `AudioWorkletNode`*
    pub fn set_onprocessorerror(this: &AudioWorkletNode, value: Option<&::js_sys::Function>);

    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "AudioWorkletNode")]
    ///The `new AudioWorkletNode(..)` constructor, creating a new instance of `AudioWorkletNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/AudioWorkletNode)
    ///
    ///*This API requires the following crate features to be activated: `AudioWorkletNode`, `BaseAudioContext`*
    pub fn new(context: &BaseAudioContext, name: &str) -> Result<AudioWorkletNode, JsValue>;

    #[cfg(all(feature = "AudioWorkletNodeOptions", feature = "BaseAudioContext",))]
    #[wasm_bindgen(catch, constructor, js_class = "AudioWorkletNode")]
    ///The `new AudioWorkletNode(..)` constructor, creating a new instance of `AudioWorkletNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/AudioWorkletNode)
    ///
    ///*This API requires the following crate features to be activated: `AudioWorkletNode`, `AudioWorkletNodeOptions`, `BaseAudioContext`*
    pub fn new_with_options(
        context: &BaseAudioContext,
        name: &str,
        options: &AudioWorkletNodeOptions,
    ) -> Result<AudioWorkletNode, JsValue>;

}
