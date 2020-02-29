use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = MediaSource , typescript_type = "MediaSource" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaSource` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`*
    pub type MediaSource;

    #[cfg(feature = "SourceBufferList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaSource" , js_name = sourceBuffers ) ]
    ///Getter for the `sourceBuffers` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/sourceBuffers)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`, `SourceBufferList`*
    pub fn source_buffers(this: &MediaSource) -> SourceBufferList;

    #[cfg(feature = "SourceBufferList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaSource" , js_name = activeSourceBuffers ) ]
    ///Getter for the `activeSourceBuffers` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/activeSourceBuffers)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`, `SourceBufferList`*
    pub fn active_source_buffers(this: &MediaSource) -> SourceBufferList;

    #[cfg(feature = "MediaSourceReadyState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaSource" , js_name = readyState ) ]
    ///Getter for the `readyState` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/readyState)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`, `MediaSourceReadyState`*
    pub fn ready_state(this: &MediaSource) -> MediaSourceReadyState;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaSource" , js_name = duration ) ]
    ///Getter for the `duration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/duration)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`*
    pub fn duration(this: &MediaSource) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaSource" , js_name = duration ) ]
    ///Setter for the `duration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/duration)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`*
    pub fn set_duration(this: &MediaSource, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaSource" , js_name = onsourceopen ) ]
    ///Getter for the `onsourceopen` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceopen)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`*
    pub fn onsourceopen(this: &MediaSource) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaSource" , js_name = onsourceopen ) ]
    ///Setter for the `onsourceopen` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceopen)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`*
    pub fn set_onsourceopen(this: &MediaSource, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaSource" , js_name = onsourceended ) ]
    ///Getter for the `onsourceended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceended)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`*
    pub fn onsourceended(this: &MediaSource) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaSource" , js_name = onsourceended ) ]
    ///Setter for the `onsourceended` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceended)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`*
    pub fn set_onsourceended(this: &MediaSource, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaSource" , js_name = onsourceclosed ) ]
    ///Getter for the `onsourceclosed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceclosed)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`*
    pub fn onsourceclosed(this: &MediaSource) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "MediaSource" , js_name = onsourceclosed ) ]
    ///Setter for the `onsourceclosed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceclosed)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`*
    pub fn set_onsourceclosed(this: &MediaSource, value: Option<&::js_sys::Function>);

    #[wasm_bindgen(catch, constructor, js_class = "MediaSource")]
    ///The `new MediaSource(..)` constructor, creating a new instance of `MediaSource`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/MediaSource)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`*
    pub fn new() -> Result<MediaSource, JsValue>;

    #[cfg(feature = "SourceBuffer")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaSource" , js_name = addSourceBuffer ) ]
    ///The `addSourceBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/addSourceBuffer)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`, `SourceBuffer`*
    pub fn add_source_buffer(this: &MediaSource, type_: &str) -> Result<SourceBuffer, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaSource" , js_name = clearLiveSeekableRange ) ]
    ///The `clearLiveSeekableRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/clearLiveSeekableRange)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`*
    pub fn clear_live_seekable_range(this: &MediaSource) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaSource" , js_name = endOfStream ) ]
    ///The `endOfStream()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/endOfStream)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`*
    pub fn end_of_stream(this: &MediaSource) -> Result<(), JsValue>;

    #[cfg(feature = "MediaSourceEndOfStreamError")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaSource" , js_name = endOfStream ) ]
    ///The `endOfStream()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/endOfStream)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`, `MediaSourceEndOfStreamError`*
    pub fn end_of_stream_with_error(
        this: &MediaSource,
        error: MediaSourceEndOfStreamError,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( static_method_of = MediaSource , js_class = "MediaSource" , js_name = isTypeSupported ) ]
    ///The `isTypeSupported()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/isTypeSupported)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`*
    pub fn is_type_supported(type_: &str) -> bool;

    #[cfg(feature = "SourceBuffer")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaSource" , js_name = removeSourceBuffer ) ]
    ///The `removeSourceBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/removeSourceBuffer)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`, `SourceBuffer`*
    pub fn remove_source_buffer(
        this: &MediaSource,
        source_buffer: &SourceBuffer,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "MediaSource" , js_name = setLiveSeekableRange ) ]
    ///The `setLiveSeekableRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/setLiveSeekableRange)
    ///
    ///*This API requires the following crate features to be activated: `MediaSource`*
    pub fn set_live_seekable_range(this: &MediaSource, start: f64, end: f64)
        -> Result<(), JsValue>;

}
