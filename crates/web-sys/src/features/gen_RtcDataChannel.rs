use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = RTCDataChannel , typescript_type = "RTCDataChannel" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcDataChannel` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub type RtcDataChannel;

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDataChannel" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/label)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn label(this: &RtcDataChannel) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDataChannel" , js_name = reliable ) ]
    ///Getter for the `reliable` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/reliable)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn reliable(this: &RtcDataChannel) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDataChannel" , js_name = maxPacketLifeTime ) ]
    ///Getter for the `maxPacketLifeTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/maxPacketLifeTime)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn max_packet_life_time(this: &RtcDataChannel) -> Option<u16>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDataChannel" , js_name = maxRetransmits ) ]
    ///Getter for the `maxRetransmits` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/maxRetransmits)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn max_retransmits(this: &RtcDataChannel) -> Option<u16>;

    #[cfg(feature = "RtcDataChannelState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDataChannel" , js_name = readyState ) ]
    ///Getter for the `readyState` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/readyState)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelState`*
    pub fn ready_state(this: &RtcDataChannel) -> RtcDataChannelState;

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDataChannel" , js_name = bufferedAmount ) ]
    ///Getter for the `bufferedAmount` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmount)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn buffered_amount(this: &RtcDataChannel) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDataChannel" , js_name = bufferedAmountLowThreshold ) ]
    ///Getter for the `bufferedAmountLowThreshold` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmountLowThreshold)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn buffered_amount_low_threshold(this: &RtcDataChannel) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "RTCDataChannel" , js_name = bufferedAmountLowThreshold ) ]
    ///Setter for the `bufferedAmountLowThreshold` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmountLowThreshold)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn set_buffered_amount_low_threshold(this: &RtcDataChannel, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDataChannel" , js_name = onopen ) ]
    ///Getter for the `onopen` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onopen)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn onopen(this: &RtcDataChannel) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "RTCDataChannel" , js_name = onopen ) ]
    ///Setter for the `onopen` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onopen)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn set_onopen(this: &RtcDataChannel, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDataChannel" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onerror)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn onerror(this: &RtcDataChannel) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "RTCDataChannel" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onerror)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn set_onerror(this: &RtcDataChannel, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDataChannel" , js_name = onclose ) ]
    ///Getter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onclose)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn onclose(this: &RtcDataChannel) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "RTCDataChannel" , js_name = onclose ) ]
    ///Setter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onclose)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn set_onclose(this: &RtcDataChannel, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDataChannel" , js_name = onmessage ) ]
    ///Getter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn onmessage(this: &RtcDataChannel) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "RTCDataChannel" , js_name = onmessage ) ]
    ///Setter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn set_onmessage(this: &RtcDataChannel, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDataChannel" , js_name = onbufferedamountlow ) ]
    ///Getter for the `onbufferedamountlow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onbufferedamountlow)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn onbufferedamountlow(this: &RtcDataChannel) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "RTCDataChannel" , js_name = onbufferedamountlow ) ]
    ///Setter for the `onbufferedamountlow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onbufferedamountlow)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn set_onbufferedamountlow(this: &RtcDataChannel, value: Option<&::js_sys::Function>);

    #[cfg(feature = "RtcDataChannelType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDataChannel" , js_name = binaryType ) ]
    ///Getter for the `binaryType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/binaryType)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelType`*
    pub fn binary_type(this: &RtcDataChannel) -> RtcDataChannelType;

    #[cfg(feature = "RtcDataChannelType")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "RTCDataChannel" , js_name = binaryType ) ]
    ///Setter for the `binaryType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/binaryType)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelType`*
    pub fn set_binary_type(this: &RtcDataChannel, value: RtcDataChannelType);

    # [ wasm_bindgen ( method , structural , js_class = "RTCDataChannel" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/close)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn close(this: &RtcDataChannel);

    # [ wasm_bindgen ( catch , method , structural , js_class = "RTCDataChannel" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn send_with_str(this: &RtcDataChannel, data: &str) -> Result<(), JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "RTCDataChannel" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `RtcDataChannel`*
    pub fn send_with_blob(this: &RtcDataChannel, data: &Blob) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "RTCDataChannel" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn send_with_array_buffer(
        this: &RtcDataChannel,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "RTCDataChannel" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn send_with_array_buffer_view(
        this: &RtcDataChannel,
        data: &::js_sys::Object,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "RTCDataChannel" , js_name = send ) ]
    ///The `send()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`*
    pub fn send_with_u8_array(this: &RtcDataChannel, data: &mut [u8]) -> Result<(), JsValue>;

}
