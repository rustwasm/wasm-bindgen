use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaSource , typescript_name = MediaSource ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaSource` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    pub type MediaSource;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaSource" , js_name = sourceBuffers ) ]
    #[cfg(feature = "SourceBufferList")]
    #[doc = "Getter for the `sourceBuffers` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/sourceBuffers)\n\n*This API requires the following crate features to be activated: `MediaSource`, `SourceBufferList`*"]
    pub fn source_buffers(this: &MediaSource) -> SourceBufferList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaSource" , js_name = activeSourceBuffers ) ]
    #[cfg(feature = "SourceBufferList")]
    #[doc = "Getter for the `activeSourceBuffers` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/activeSourceBuffers)\n\n*This API requires the following crate features to be activated: `MediaSource`, `SourceBufferList`*"]
    pub fn active_source_buffers(this: &MediaSource) -> SourceBufferList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaSource" , js_name = readyState ) ]
    #[cfg(feature = "MediaSourceReadyState")]
    #[doc = "Getter for the `readyState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/readyState)\n\n*This API requires the following crate features to be activated: `MediaSource`, `MediaSourceReadyState`*"]
    pub fn ready_state(this: &MediaSource) -> MediaSourceReadyState;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaSource" , js_name = duration ) ]
    #[doc = "Getter for the `duration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/duration)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn duration(this: &MediaSource) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaSource" , js_name = duration ) ]
    #[doc = "Setter for the `duration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/duration)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn set_duration(this: &MediaSource, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaSource" , js_name = onsourceopen ) ]
    #[doc = "Getter for the `onsourceopen` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceopen)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn onsourceopen(this: &MediaSource) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaSource" , js_name = onsourceopen ) ]
    #[doc = "Setter for the `onsourceopen` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceopen)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn set_onsourceopen(this: &MediaSource, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaSource" , js_name = onsourceended ) ]
    #[doc = "Getter for the `onsourceended` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceended)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn onsourceended(this: &MediaSource) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaSource" , js_name = onsourceended ) ]
    #[doc = "Setter for the `onsourceended` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceended)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn set_onsourceended(this: &MediaSource, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaSource" , js_name = onsourceclosed ) ]
    #[doc = "Getter for the `onsourceclosed` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceclosed)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn onsourceclosed(this: &MediaSource) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaSource" , js_name = onsourceclosed ) ]
    #[doc = "Setter for the `onsourceclosed` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceclosed)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn set_onsourceclosed(this: &MediaSource, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, js_class = "MediaSource", constructor)]
    #[doc = "The `new MediaSource(..)` constructor, creating a new instance of `MediaSource`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/MediaSource)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn new(this: &MediaSource) -> Result<MediaSource, JsValue>;
    #[cfg(feature = "SourceBuffer")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaSource" , js_name = addSourceBuffer ) ]
    #[doc = "The `addSourceBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/addSourceBuffer)\n\n*This API requires the following crate features to be activated: `MediaSource`, `SourceBuffer`*"]
    pub fn add_source_buffer(this: &MediaSource, type_: &str) -> Result<SourceBuffer, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaSource" , js_name = clearLiveSeekableRange ) ]
    #[doc = "The `clearLiveSeekableRange()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/clearLiveSeekableRange)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn clear_live_seekable_range(this: &MediaSource) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaSource" , js_name = endOfStream ) ]
    #[doc = "The `endOfStream()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/endOfStream)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn end_of_stream(this: &MediaSource) -> Result<(), JsValue>;
    #[cfg(feature = "MediaSourceEndOfStreamError")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaSource" , js_name = endOfStream ) ]
    #[doc = "The `endOfStream()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/endOfStream)\n\n*This API requires the following crate features to be activated: `MediaSource`, `MediaSourceEndOfStreamError`*"]
    pub fn end_of_stream_with_error(
        this: &MediaSource,
        error: MediaSourceEndOfStreamError,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( static_method_of = MediaSource , js_class = "MediaSource" , js_name = isTypeSupported ) ]
    #[doc = "The `isTypeSupported()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/isTypeSupported)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn is_type_supported(type_: &str) -> bool;
    #[cfg(feature = "SourceBuffer")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaSource" , js_name = removeSourceBuffer ) ]
    #[doc = "The `removeSourceBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/removeSourceBuffer)\n\n*This API requires the following crate features to be activated: `MediaSource`, `SourceBuffer`*"]
    pub fn remove_source_buffer(
        this: &MediaSource,
        source_buffer: &SourceBuffer,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaSource" , js_name = setLiveSeekableRange ) ]
    #[doc = "The `setLiveSeekableRange()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/setLiveSeekableRange)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    pub fn set_live_seekable_range(this: &MediaSource, start: f64, end: f64)
        -> Result<(), JsValue>;
}
