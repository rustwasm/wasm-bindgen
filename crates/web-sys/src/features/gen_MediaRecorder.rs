use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaRecorder , typescript_name = MediaRecorder ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaRecorder` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub type MediaRecorder;

    #[cfg(feature = "MediaStream")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaRecorder" , js_name = stream ) ]
    ///Getter for the `stream` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/stream)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`, `MediaStream`*
    pub fn stream(this: &MediaRecorder) -> MediaStream;

    #[cfg(feature = "RecordingState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaRecorder" , js_name = state ) ]
    ///Getter for the `state` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/state)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`, `RecordingState`*
    pub fn state(this: &MediaRecorder) -> RecordingState;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaRecorder" , js_name = mimeType ) ]
    ///Getter for the `mimeType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/mimeType)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn mime_type(this: &MediaRecorder) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaRecorder" , js_name = ondataavailable ) ]
    ///Getter for the `ondataavailable` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/ondataavailable)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn ondataavailable(this: &MediaRecorder) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaRecorder" , js_name = ondataavailable ) ]
    ///Setter for the `ondataavailable` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/ondataavailable)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn set_ondataavailable(this: &MediaRecorder, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaRecorder" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onerror)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn onerror(this: &MediaRecorder) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaRecorder" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onerror)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn set_onerror(this: &MediaRecorder, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaRecorder" , js_name = onstart ) ]
    ///Getter for the `onstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstart)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn onstart(this: &MediaRecorder) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaRecorder" , js_name = onstart ) ]
    ///Setter for the `onstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstart)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn set_onstart(this: &MediaRecorder, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaRecorder" , js_name = onstop ) ]
    ///Getter for the `onstop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstop)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn onstop(this: &MediaRecorder) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaRecorder" , js_name = onstop ) ]
    ///Setter for the `onstop` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstop)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn set_onstop(this: &MediaRecorder, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaRecorder" , js_name = onwarning ) ]
    ///Getter for the `onwarning` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onwarning)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn onwarning(this: &MediaRecorder) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaRecorder" , js_name = onwarning ) ]
    ///Setter for the `onwarning` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onwarning)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn set_onwarning(this: &MediaRecorder, value: Option<&::js_sys::Function>);

    #[cfg(feature = "MediaStream")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaRecorder")]
    ///The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`, `MediaStream`*
    pub fn new_with_media_stream(stream: &MediaStream) -> Result<MediaRecorder, JsValue>;

    #[cfg(all(feature = "MediaRecorderOptions", feature = "MediaStream",))]
    #[wasm_bindgen(catch, constructor, js_class = "MediaRecorder")]
    ///The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`, `MediaRecorderOptions`, `MediaStream`*
    pub fn new_with_media_stream_and_media_recorder_options(
        stream: &MediaStream,
        options: &MediaRecorderOptions,
    ) -> Result<MediaRecorder, JsValue>;

    #[cfg(feature = "AudioNode")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaRecorder")]
    ///The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)
    ///
    ///*This API requires the following crate features to be activated: `AudioNode`, `MediaRecorder`*
    pub fn new_with_audio_node(node: &AudioNode) -> Result<MediaRecorder, JsValue>;

    #[cfg(feature = "AudioNode")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaRecorder")]
    ///The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)
    ///
    ///*This API requires the following crate features to be activated: `AudioNode`, `MediaRecorder`*
    pub fn new_with_audio_node_and_u32(
        node: &AudioNode,
        output: u32,
    ) -> Result<MediaRecorder, JsValue>;

    #[cfg(all(feature = "AudioNode", feature = "MediaRecorderOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "MediaRecorder")]
    ///The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)
    ///
    ///*This API requires the following crate features to be activated: `AudioNode`, `MediaRecorder`, `MediaRecorderOptions`*
    pub fn new_with_audio_node_and_u32_and_options(
        node: &AudioNode,
        output: u32,
        options: &MediaRecorderOptions,
    ) -> Result<MediaRecorder, JsValue>;

    # [ wasm_bindgen ( static_method_of = MediaRecorder , js_class = "MediaRecorder" , js_name = isTypeSupported ) ]
    ///The `isTypeSupported()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/isTypeSupported)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn is_type_supported(type_: &str) -> bool;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaRecorder" , js_name = pause ) ]
    ///The `pause()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/pause)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn pause(this: &MediaRecorder) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaRecorder" , js_name = requestData ) ]
    ///The `requestData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/requestData)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn request_data(this: &MediaRecorder) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaRecorder" , js_name = resume ) ]
    ///The `resume()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/resume)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn resume(this: &MediaRecorder) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaRecorder" , js_name = start ) ]
    ///The `start()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/start)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn start(this: &MediaRecorder) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaRecorder" , js_name = start ) ]
    ///The `start()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/start)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn start_with_time_slice(this: &MediaRecorder, time_slice: i32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaRecorder" , js_name = stop ) ]
    ///The `stop()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/stop)
    ///
    ///*This API requires the following crate features to be activated: `MediaRecorder`*
    pub fn stop(this: &MediaRecorder) -> Result<(), JsValue>;

}
