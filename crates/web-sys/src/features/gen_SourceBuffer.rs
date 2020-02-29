use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = SourceBuffer , typescript_name = SourceBuffer ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SourceBuffer` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub type SourceBuffer;

    #[cfg(feature = "SourceBufferAppendMode")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SourceBuffer" , js_name = mode ) ]
    ///Getter for the `mode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/mode)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`, `SourceBufferAppendMode`*
    pub fn mode(this: &SourceBuffer) -> SourceBufferAppendMode;

    #[cfg(feature = "SourceBufferAppendMode")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "SourceBuffer" , js_name = mode ) ]
    ///Setter for the `mode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/mode)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`, `SourceBufferAppendMode`*
    pub fn set_mode(this: &SourceBuffer, value: SourceBufferAppendMode);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SourceBuffer" , js_name = updating ) ]
    ///Getter for the `updating` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/updating)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn updating(this: &SourceBuffer) -> bool;

    #[cfg(feature = "TimeRanges")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "SourceBuffer" , js_name = buffered ) ]
    ///Getter for the `buffered` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/buffered)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`, `TimeRanges`*
    pub fn buffered(this: &SourceBuffer) -> Result<TimeRanges, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SourceBuffer" , js_name = timestampOffset ) ]
    ///Getter for the `timestampOffset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/timestampOffset)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn timestamp_offset(this: &SourceBuffer) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SourceBuffer" , js_name = timestampOffset ) ]
    ///Setter for the `timestampOffset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/timestampOffset)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn set_timestamp_offset(this: &SourceBuffer, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SourceBuffer" , js_name = appendWindowStart ) ]
    ///Getter for the `appendWindowStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowStart)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn append_window_start(this: &SourceBuffer) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SourceBuffer" , js_name = appendWindowStart ) ]
    ///Setter for the `appendWindowStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowStart)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn set_append_window_start(this: &SourceBuffer, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SourceBuffer" , js_name = appendWindowEnd ) ]
    ///Getter for the `appendWindowEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowEnd)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn append_window_end(this: &SourceBuffer) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SourceBuffer" , js_name = appendWindowEnd ) ]
    ///Setter for the `appendWindowEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowEnd)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn set_append_window_end(this: &SourceBuffer, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SourceBuffer" , js_name = onupdatestart ) ]
    ///Getter for the `onupdatestart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdatestart)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn onupdatestart(this: &SourceBuffer) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SourceBuffer" , js_name = onupdatestart ) ]
    ///Setter for the `onupdatestart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdatestart)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn set_onupdatestart(this: &SourceBuffer, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SourceBuffer" , js_name = onupdate ) ]
    ///Getter for the `onupdate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdate)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn onupdate(this: &SourceBuffer) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SourceBuffer" , js_name = onupdate ) ]
    ///Setter for the `onupdate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdate)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn set_onupdate(this: &SourceBuffer, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SourceBuffer" , js_name = onupdateend ) ]
    ///Getter for the `onupdateend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdateend)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn onupdateend(this: &SourceBuffer) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SourceBuffer" , js_name = onupdateend ) ]
    ///Setter for the `onupdateend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdateend)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn set_onupdateend(this: &SourceBuffer, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SourceBuffer" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onerror)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn onerror(this: &SourceBuffer) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SourceBuffer" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onerror)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn set_onerror(this: &SourceBuffer, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SourceBuffer" , js_name = onabort ) ]
    ///Getter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onabort)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn onabort(this: &SourceBuffer) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SourceBuffer" , js_name = onabort ) ]
    ///Setter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onabort)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn set_onabort(this: &SourceBuffer, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( catch , method , structural , js_class = "SourceBuffer" , js_name = abort ) ]
    ///The `abort()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/abort)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn abort(this: &SourceBuffer) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SourceBuffer" , js_name = appendBuffer ) ]
    ///The `appendBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn append_buffer_with_array_buffer(
        this: &SourceBuffer,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SourceBuffer" , js_name = appendBuffer ) ]
    ///The `appendBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn append_buffer_with_array_buffer_view(
        this: &SourceBuffer,
        data: &::js_sys::Object,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SourceBuffer" , js_name = appendBuffer ) ]
    ///The `appendBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn append_buffer_with_u8_array(this: &SourceBuffer, data: &mut [u8])
        -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SourceBuffer" , js_name = appendBufferAsync ) ]
    ///The `appendBufferAsync()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn append_buffer_async_with_array_buffer(
        this: &SourceBuffer,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SourceBuffer" , js_name = appendBufferAsync ) ]
    ///The `appendBufferAsync()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn append_buffer_async_with_array_buffer_view(
        this: &SourceBuffer,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SourceBuffer" , js_name = appendBufferAsync ) ]
    ///The `appendBufferAsync()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn append_buffer_async_with_u8_array(
        this: &SourceBuffer,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SourceBuffer" , js_name = changeType ) ]
    ///The `changeType()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/changeType)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn change_type(this: &SourceBuffer, type_: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SourceBuffer" , js_name = remove ) ]
    ///The `remove()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/remove)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn remove(this: &SourceBuffer, start: f64, end: f64) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SourceBuffer" , js_name = removeAsync ) ]
    ///The `removeAsync()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/removeAsync)
    ///
    ///*This API requires the following crate features to be activated: `SourceBuffer`*
    pub fn remove_async(
        this: &SourceBuffer,
        start: f64,
        end: f64,
    ) -> Result<::js_sys::Promise, JsValue>;

}
