use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = AudioScheduledSourceNode , typescript_name = AudioScheduledSourceNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AudioScheduledSourceNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode)
    ///
    ///*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*
    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    pub type AudioScheduledSourceNode;

    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "AudioScheduledSourceNode" , js_name = onended ) ]
    ///Getter for the `onended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/onended)
    ///
    ///*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*
    pub fn onended(this: &AudioScheduledSourceNode) -> Option<::js_sys::Function>;

    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "AudioScheduledSourceNode" , js_name = onended ) ]
    ///Setter for the `onended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/onended)
    ///
    ///*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*
    pub fn set_onended(this: &AudioScheduledSourceNode, value: Option<&::js_sys::Function>);

    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioScheduledSourceNode" , js_name = start ) ]
    ///The `start()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/start)
    ///
    ///*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*
    pub fn start(this: &AudioScheduledSourceNode) -> Result<(), JsValue>;

    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioScheduledSourceNode" , js_name = start ) ]
    ///The `start()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/start)
    ///
    ///*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*
    pub fn start_with_when(this: &AudioScheduledSourceNode, when: f64) -> Result<(), JsValue>;

    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioScheduledSourceNode" , js_name = stop ) ]
    ///The `stop()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/stop)
    ///
    ///*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*
    pub fn stop(this: &AudioScheduledSourceNode) -> Result<(), JsValue>;

    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "AudioScheduledSourceNode" , js_name = stop ) ]
    ///The `stop()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/stop)
    ///
    ///*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*
    pub fn stop_with_when(this: &AudioScheduledSourceNode, when: f64) -> Result<(), JsValue>;

}
