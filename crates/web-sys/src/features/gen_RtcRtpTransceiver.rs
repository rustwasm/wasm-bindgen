use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCRtpTransceiver , typescript_name = RTCRtpTransceiver ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcRtpTransceiver` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpTransceiver`*
    pub type RtcRtpTransceiver;

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpTransceiver" , js_name = mid ) ]
    ///Getter for the `mid` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/mid)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpTransceiver`*
    pub fn mid(this: &RtcRtpTransceiver) -> Option<String>;

    #[cfg(feature = "RtcRtpSender")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpTransceiver" , js_name = sender ) ]
    ///Getter for the `sender` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/sender)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpSender`, `RtcRtpTransceiver`*
    pub fn sender(this: &RtcRtpTransceiver) -> RtcRtpSender;

    #[cfg(feature = "RtcRtpReceiver")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpTransceiver" , js_name = receiver ) ]
    ///Getter for the `receiver` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/receiver)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcRtpTransceiver`*
    pub fn receiver(this: &RtcRtpTransceiver) -> RtcRtpReceiver;

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpTransceiver" , js_name = stopped ) ]
    ///Getter for the `stopped` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/stopped)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpTransceiver`*
    pub fn stopped(this: &RtcRtpTransceiver) -> bool;

    #[cfg(feature = "RtcRtpTransceiverDirection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpTransceiver" , js_name = direction ) ]
    ///Getter for the `direction` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/direction)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcRtpTransceiverDirection`*
    pub fn direction(this: &RtcRtpTransceiver) -> RtcRtpTransceiverDirection;

    #[cfg(feature = "RtcRtpTransceiverDirection")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "RTCRtpTransceiver" , js_name = direction ) ]
    ///Setter for the `direction` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/direction)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcRtpTransceiverDirection`*
    pub fn set_direction(this: &RtcRtpTransceiver, value: RtcRtpTransceiverDirection);

    #[cfg(feature = "RtcRtpTransceiverDirection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpTransceiver" , js_name = currentDirection ) ]
    ///Getter for the `currentDirection` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/currentDirection)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcRtpTransceiverDirection`*
    pub fn current_direction(this: &RtcRtpTransceiver) -> Option<RtcRtpTransceiverDirection>;

    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpTransceiver" , js_name = getRemoteTrackId ) ]
    ///The `getRemoteTrackId()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/getRemoteTrackId)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpTransceiver`*
    pub fn get_remote_track_id(this: &RtcRtpTransceiver) -> String;

    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpTransceiver" , js_name = stop ) ]
    ///The `stop()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/stop)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpTransceiver`*
    pub fn stop(this: &RtcRtpTransceiver);

}
