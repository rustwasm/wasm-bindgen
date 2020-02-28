use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = RTCDataChannel , typescript_name = RTCDataChannel ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcDataChannel` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub type RtcDataChannel;
    # [ wasm_bindgen ( structural , method , getter , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/label)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn label(this: &RtcDataChannel) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = reliable ) ]
    #[doc = "Getter for the `reliable` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/reliable)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn reliable(this: &RtcDataChannel) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = maxPacketLifeTime ) ]
    #[doc = "Getter for the `maxPacketLifeTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/maxPacketLifeTime)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn max_packet_life_time(this: &RtcDataChannel) -> Option<u16>;
    # [ wasm_bindgen ( structural , method , getter , js_name = maxRetransmits ) ]
    #[doc = "Getter for the `maxRetransmits` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/maxRetransmits)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn max_retransmits(this: &RtcDataChannel) -> Option<u16>;
    # [ wasm_bindgen ( structural , method , getter , js_name = readyState ) ]
    #[cfg(feature = "RtcDataChannelState")]
    #[doc = "Getter for the `readyState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/readyState)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelState`*"]
    pub fn ready_state(this: &RtcDataChannel) -> RtcDataChannelState;
    # [ wasm_bindgen ( structural , method , getter , js_name = bufferedAmount ) ]
    #[doc = "Getter for the `bufferedAmount` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmount)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn buffered_amount(this: &RtcDataChannel) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = bufferedAmountLowThreshold ) ]
    #[doc = "Getter for the `bufferedAmountLowThreshold` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmountLowThreshold)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn buffered_amount_low_threshold(this: &RtcDataChannel) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_name = bufferedAmountLowThreshold ) ]
    #[doc = "Setter for the `bufferedAmountLowThreshold` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmountLowThreshold)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn set_buffered_amount_low_threshold(this: &RtcDataChannel, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_name = onopen ) ]
    #[doc = "Getter for the `onopen` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onopen)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn onopen(this: &RtcDataChannel) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onopen ) ]
    #[doc = "Setter for the `onopen` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onopen)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn set_onopen(this: &RtcDataChannel, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onerror)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn onerror(this: &RtcDataChannel) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onerror)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn set_onerror(this: &RtcDataChannel, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onclose ) ]
    #[doc = "Getter for the `onclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onclose)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn onclose(this: &RtcDataChannel) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onclose ) ]
    #[doc = "Setter for the `onclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onclose)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn set_onclose(this: &RtcDataChannel, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmessage ) ]
    #[doc = "Getter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onmessage)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn onmessage(this: &RtcDataChannel) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmessage ) ]
    #[doc = "Setter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onmessage)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn set_onmessage(this: &RtcDataChannel, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onbufferedamountlow ) ]
    #[doc = "Getter for the `onbufferedamountlow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onbufferedamountlow)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn onbufferedamountlow(this: &RtcDataChannel) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onbufferedamountlow ) ]
    #[doc = "Setter for the `onbufferedamountlow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onbufferedamountlow)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn set_onbufferedamountlow(this: &RtcDataChannel, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = binaryType ) ]
    #[cfg(feature = "RtcDataChannelType")]
    #[doc = "Getter for the `binaryType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/binaryType)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelType`*"]
    pub fn binary_type(this: &RtcDataChannel) -> RtcDataChannelType;
    # [ wasm_bindgen ( structural , method , setter , js_name = binaryType ) ]
    #[cfg(feature = "RtcDataChannelType")]
    #[doc = "Setter for the `binaryType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/binaryType)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelType`*"]
    pub fn set_binary_type(this: &RtcDataChannel, value: RtcDataChannelType);
    # [ wasm_bindgen ( method , structural , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/close)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn close(this: &RtcDataChannel);
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn send_with_str(this: &RtcDataChannel, data: &str) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)\n\n*This API requires the following crate features to be activated: `Blob`, `RtcDataChannel`*"]
    pub fn send_with_blob(this: &RtcDataChannel, data: &Blob) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn send_with_array_buffer(
        this: &RtcDataChannel,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn send_with_array_buffer_view(
        this: &RtcDataChannel,
        data: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = send ) ]
    #[doc = "The `send()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    pub fn send_with_u8_array(this: &RtcDataChannel, data: &mut [u8]) -> Result<(), JsValue>;
}
