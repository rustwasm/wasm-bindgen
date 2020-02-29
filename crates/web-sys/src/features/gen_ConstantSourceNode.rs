use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioScheduledSourceNode , extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = ConstantSourceNode , typescript_name = ConstantSourceNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ConstantSourceNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode)
    ///
    ///*This API requires the following crate features to be activated: `ConstantSourceNode`*
    pub type ConstantSourceNode;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "ConstantSourceNode" , js_name = offset ) ]
    ///Getter for the `offset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/offset)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `ConstantSourceNode`*
    pub fn offset(this: &ConstantSourceNode) -> AudioParam;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ConstantSourceNode" , js_name = onended ) ]
    ///Getter for the `onended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/onended)
    ///
    ///*This API requires the following crate features to be activated: `ConstantSourceNode`*
    pub fn onended(this: &ConstantSourceNode) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ConstantSourceNode" , js_name = onended ) ]
    ///Setter for the `onended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/onended)
    ///
    ///*This API requires the following crate features to be activated: `ConstantSourceNode`*
    pub fn set_onended(this: &ConstantSourceNode, value: Option<&::js_sys::Function>);

    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "ConstantSourceNode")]
    ///The `new ConstantSourceNode(..)` constructor, creating a new instance of `ConstantSourceNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/ConstantSourceNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `ConstantSourceNode`*
    pub fn new(context: &BaseAudioContext) -> Result<ConstantSourceNode, JsValue>;

    #[cfg(all(feature = "BaseAudioContext", feature = "ConstantSourceOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "ConstantSourceNode")]
    ///The `new ConstantSourceNode(..)` constructor, creating a new instance of `ConstantSourceNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/ConstantSourceNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `ConstantSourceNode`, `ConstantSourceOptions`*
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &ConstantSourceOptions,
    ) -> Result<ConstantSourceNode, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "ConstantSourceNode" , js_name = start ) ]
    ///The `start()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/start)
    ///
    ///*This API requires the following crate features to be activated: `ConstantSourceNode`*
    pub fn start(this: &ConstantSourceNode) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "ConstantSourceNode" , js_name = start ) ]
    ///The `start()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/start)
    ///
    ///*This API requires the following crate features to be activated: `ConstantSourceNode`*
    pub fn start_with_when(this: &ConstantSourceNode, when: f64) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "ConstantSourceNode" , js_name = stop ) ]
    ///The `stop()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/stop)
    ///
    ///*This API requires the following crate features to be activated: `ConstantSourceNode`*
    pub fn stop(this: &ConstantSourceNode) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "ConstantSourceNode" , js_name = stop ) ]
    ///The `stop()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/stop)
    ///
    ///*This API requires the following crate features to be activated: `ConstantSourceNode`*
    pub fn stop_with_when(this: &ConstantSourceNode, when: f64) -> Result<(), JsValue>;

}
