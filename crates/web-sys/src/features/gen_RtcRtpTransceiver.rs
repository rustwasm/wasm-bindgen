use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCRtpTransceiver , typescript_name = RTCRtpTransceiver ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpTransceiver` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver)\n\n*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
    pub type RtcRtpTransceiver;
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpTransceiver" , js_name = mid ) ]
    #[doc = "Getter for the `mid` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/mid)\n\n*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
    pub fn mid(this: &RtcRtpTransceiver) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpTransceiver" , js_name = sender ) ]
    #[cfg(feature = "RtcRtpSender")]
    #[doc = "Getter for the `sender` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/sender)\n\n*This API requires the following crate features to be activated: `RtcRtpSender`, `RtcRtpTransceiver`*"]
    pub fn sender(this: &RtcRtpTransceiver) -> RtcRtpSender;
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpTransceiver" , js_name = receiver ) ]
    #[cfg(feature = "RtcRtpReceiver")]
    #[doc = "Getter for the `receiver` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/receiver)\n\n*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcRtpTransceiver`*"]
    pub fn receiver(this: &RtcRtpTransceiver) -> RtcRtpReceiver;
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpTransceiver" , js_name = stopped ) ]
    #[doc = "Getter for the `stopped` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/stopped)\n\n*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
    pub fn stopped(this: &RtcRtpTransceiver) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpTransceiver" , js_name = direction ) ]
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[doc = "Getter for the `direction` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/direction)\n\n*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcRtpTransceiverDirection`*"]
    pub fn direction(this: &RtcRtpTransceiver) -> RtcRtpTransceiverDirection;
    # [ wasm_bindgen ( structural , method , setter , js_class = "RTCRtpTransceiver" , js_name = direction ) ]
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[doc = "Setter for the `direction` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/direction)\n\n*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcRtpTransceiverDirection`*"]
    pub fn set_direction(this: &RtcRtpTransceiver, value: RtcRtpTransceiverDirection);
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpTransceiver" , js_name = currentDirection ) ]
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[doc = "Getter for the `currentDirection` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/currentDirection)\n\n*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcRtpTransceiverDirection`*"]
    pub fn current_direction(this: &RtcRtpTransceiver) -> Option<RtcRtpTransceiverDirection>;
    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpTransceiver" , js_name = getRemoteTrackId ) ]
    #[doc = "The `getRemoteTrackId()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/getRemoteTrackId)\n\n*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
    pub fn get_remote_track_id(this: &RtcRtpTransceiver) -> String;
    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpTransceiver" , js_name = stop ) ]
    #[doc = "The `stop()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/stop)\n\n*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
    pub fn stop(this: &RtcRtpTransceiver);
}
