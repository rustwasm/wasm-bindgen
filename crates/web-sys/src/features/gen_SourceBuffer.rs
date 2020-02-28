use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = SourceBuffer , typescript_name = SourceBuffer ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SourceBuffer` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub type SourceBuffer;
    # [ wasm_bindgen ( structural , method , getter , js_name = mode ) ]
    #[cfg(feature = "SourceBufferAppendMode")]
    #[doc = "Getter for the `mode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/mode)\n\n*This API requires the following crate features to be activated: `SourceBuffer`, `SourceBufferAppendMode`*"]
    pub fn mode(this: &SourceBuffer) -> SourceBufferAppendMode;
    # [ wasm_bindgen ( structural , method , setter , js_name = mode ) ]
    #[cfg(feature = "SourceBufferAppendMode")]
    #[doc = "Setter for the `mode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/mode)\n\n*This API requires the following crate features to be activated: `SourceBuffer`, `SourceBufferAppendMode`*"]
    pub fn set_mode(this: &SourceBuffer, value: SourceBufferAppendMode);
    # [ wasm_bindgen ( structural , method , getter , js_name = updating ) ]
    #[doc = "Getter for the `updating` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/updating)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn updating(this: &SourceBuffer) -> bool;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = buffered ) ]
    #[cfg(feature = "TimeRanges")]
    #[doc = "Getter for the `buffered` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/buffered)\n\n*This API requires the following crate features to be activated: `SourceBuffer`, `TimeRanges`*"]
    pub fn buffered(this: &SourceBuffer) -> Result<TimeRanges, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = timestampOffset ) ]
    #[doc = "Getter for the `timestampOffset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/timestampOffset)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn timestamp_offset(this: &SourceBuffer) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = timestampOffset ) ]
    #[doc = "Setter for the `timestampOffset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/timestampOffset)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn set_timestamp_offset(this: &SourceBuffer, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = appendWindowStart ) ]
    #[doc = "Getter for the `appendWindowStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowStart)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_window_start(this: &SourceBuffer) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = appendWindowStart ) ]
    #[doc = "Setter for the `appendWindowStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowStart)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn set_append_window_start(this: &SourceBuffer, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = appendWindowEnd ) ]
    #[doc = "Getter for the `appendWindowEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowEnd)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_window_end(this: &SourceBuffer) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = appendWindowEnd ) ]
    #[doc = "Setter for the `appendWindowEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowEnd)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn set_append_window_end(this: &SourceBuffer, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = onupdatestart ) ]
    #[doc = "Getter for the `onupdatestart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdatestart)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn onupdatestart(this: &SourceBuffer) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onupdatestart ) ]
    #[doc = "Setter for the `onupdatestart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdatestart)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn set_onupdatestart(this: &SourceBuffer, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onupdate ) ]
    #[doc = "Getter for the `onupdate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdate)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn onupdate(this: &SourceBuffer) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onupdate ) ]
    #[doc = "Setter for the `onupdate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdate)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn set_onupdate(this: &SourceBuffer, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onupdateend ) ]
    #[doc = "Getter for the `onupdateend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdateend)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn onupdateend(this: &SourceBuffer) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onupdateend ) ]
    #[doc = "Setter for the `onupdateend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdateend)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn set_onupdateend(this: &SourceBuffer, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onerror)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn onerror(this: &SourceBuffer) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onerror)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn set_onerror(this: &SourceBuffer, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onabort ) ]
    #[doc = "Getter for the `onabort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onabort)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn onabort(this: &SourceBuffer) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onabort ) ]
    #[doc = "Setter for the `onabort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onabort)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn set_onabort(this: &SourceBuffer, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( catch , method , structural , js_name = abort ) ]
    #[doc = "The `abort()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/abort)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn abort(this: &SourceBuffer) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = appendBuffer ) ]
    #[doc = "The `appendBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_buffer_with_array_buffer(
        this: &SourceBuffer,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = appendBuffer ) ]
    #[doc = "The `appendBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_buffer_with_array_buffer_view(
        this: &SourceBuffer,
        data: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = appendBuffer ) ]
    #[doc = "The `appendBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_buffer_with_u8_array(this: &SourceBuffer, data: &mut [u8])
        -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = appendBufferAsync ) ]
    #[doc = "The `appendBufferAsync()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_buffer_async_with_array_buffer(
        this: &SourceBuffer,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = appendBufferAsync ) ]
    #[doc = "The `appendBufferAsync()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_buffer_async_with_array_buffer_view(
        this: &SourceBuffer,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = appendBufferAsync ) ]
    #[doc = "The `appendBufferAsync()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn append_buffer_async_with_u8_array(
        this: &SourceBuffer,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = changeType ) ]
    #[doc = "The `changeType()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/changeType)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn change_type(this: &SourceBuffer, type_: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = remove ) ]
    #[doc = "The `remove()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/remove)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn remove(this: &SourceBuffer, start: f64, end: f64) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = removeAsync ) ]
    #[doc = "The `removeAsync()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/removeAsync)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    pub fn remove_async(
        this: &SourceBuffer,
        start: f64,
        end: f64,
    ) -> Result<::js_sys::Promise, JsValue>;
}
