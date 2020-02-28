use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaRecorder , typescript_name = MediaRecorder ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaRecorder` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub type MediaRecorder;
    # [ wasm_bindgen ( structural , method , getter , js_name = stream ) ]
    #[cfg(feature = "MediaStream")]
    #[doc = "Getter for the `stream` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/stream)\n\n*This API requires the following crate features to be activated: `MediaRecorder`, `MediaStream`*"]
    pub fn stream(this: &MediaRecorder) -> MediaStream;
    # [ wasm_bindgen ( structural , method , getter , js_name = state ) ]
    #[cfg(feature = "RecordingState")]
    #[doc = "Getter for the `state` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/state)\n\n*This API requires the following crate features to be activated: `MediaRecorder`, `RecordingState`*"]
    pub fn state(this: &MediaRecorder) -> RecordingState;
    # [ wasm_bindgen ( structural , method , getter , js_name = mimeType ) ]
    #[doc = "Getter for the `mimeType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/mimeType)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn mime_type(this: &MediaRecorder) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = ondataavailable ) ]
    #[doc = "Getter for the `ondataavailable` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/ondataavailable)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn ondataavailable(this: &MediaRecorder) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondataavailable ) ]
    #[doc = "Setter for the `ondataavailable` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/ondataavailable)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn set_ondataavailable(this: &MediaRecorder, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onerror)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn onerror(this: &MediaRecorder) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onerror)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn set_onerror(this: &MediaRecorder, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onstart ) ]
    #[doc = "Getter for the `onstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstart)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn onstart(this: &MediaRecorder) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onstart ) ]
    #[doc = "Setter for the `onstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstart)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn set_onstart(this: &MediaRecorder, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onstop ) ]
    #[doc = "Getter for the `onstop` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstop)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn onstop(this: &MediaRecorder) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onstop ) ]
    #[doc = "Setter for the `onstop` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstop)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn set_onstop(this: &MediaRecorder, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onwarning ) ]
    #[doc = "Getter for the `onwarning` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onwarning)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn onwarning(this: &MediaRecorder) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onwarning ) ]
    #[doc = "Setter for the `onwarning` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onwarning)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn set_onwarning(this: &MediaRecorder, value: Option<&::js_sys::Function>);
    #[cfg(feature = "MediaStream")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)\n\n*This API requires the following crate features to be activated: `MediaRecorder`, `MediaStream`*"]
    pub fn new_with_media_stream(
        this: &MediaRecorder,
        stream: &MediaStream,
    ) -> Result<MediaRecorder, JsValue>;
    #[cfg(all(feature = "MediaRecorderOptions", feature = "MediaStream",))]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)\n\n*This API requires the following crate features to be activated: `MediaRecorder`, `MediaRecorderOptions`, `MediaStream`*"]
    pub fn new_with_media_stream_and_media_recorder_options(
        this: &MediaRecorder,
        stream: &MediaStream,
        options: &MediaRecorderOptions,
    ) -> Result<MediaRecorder, JsValue>;
    #[cfg(feature = "AudioNode")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)\n\n*This API requires the following crate features to be activated: `AudioNode`, `MediaRecorder`*"]
    pub fn new_with_audio_node(
        this: &MediaRecorder,
        node: &AudioNode,
    ) -> Result<MediaRecorder, JsValue>;
    #[cfg(feature = "AudioNode")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)\n\n*This API requires the following crate features to be activated: `AudioNode`, `MediaRecorder`*"]
    pub fn new_with_audio_node_and_u32(
        this: &MediaRecorder,
        node: &AudioNode,
        output: u32,
    ) -> Result<MediaRecorder, JsValue>;
    #[cfg(all(feature = "AudioNode", feature = "MediaRecorderOptions",))]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)\n\n*This API requires the following crate features to be activated: `AudioNode`, `MediaRecorder`, `MediaRecorderOptions`*"]
    pub fn new_with_audio_node_and_u32_and_options(
        this: &MediaRecorder,
        node: &AudioNode,
        output: u32,
        options: &MediaRecorderOptions,
    ) -> Result<MediaRecorder, JsValue>;
    # [ wasm_bindgen ( method , structural , static_method_of = MediaRecorder , js_name = isTypeSupported ) ]
    #[doc = "The `isTypeSupported()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/isTypeSupported)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn is_type_supported(type_: &str) -> bool;
    # [ wasm_bindgen ( catch , method , structural , js_name = pause ) ]
    #[doc = "The `pause()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/pause)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn pause(this: &MediaRecorder) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = requestData ) ]
    #[doc = "The `requestData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/requestData)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn request_data(this: &MediaRecorder) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = resume ) ]
    #[doc = "The `resume()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/resume)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn resume(this: &MediaRecorder) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = start ) ]
    #[doc = "The `start()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/start)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn start(this: &MediaRecorder) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = start ) ]
    #[doc = "The `start()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/start)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn start_with_time_slice(this: &MediaRecorder, time_slice: i32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = stop ) ]
    #[doc = "The `stop()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/stop)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    pub fn stop(this: &MediaRecorder) -> Result<(), JsValue>;
}
