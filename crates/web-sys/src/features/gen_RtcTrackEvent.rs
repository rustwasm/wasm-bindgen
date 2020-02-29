use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = RTCTrackEvent , typescript_type = "RTCTrackEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcTrackEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent)
    ///
    ///*This API requires the following crate features to be activated: `RtcTrackEvent`*
    pub type RtcTrackEvent;

    #[cfg(feature = "RtcRtpReceiver")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCTrackEvent" , js_name = receiver ) ]
    ///Getter for the `receiver` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/receiver)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcTrackEvent`*
    pub fn receiver(this: &RtcTrackEvent) -> RtcRtpReceiver;

    #[cfg(feature = "MediaStreamTrack")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCTrackEvent" , js_name = track ) ]
    ///Getter for the `track` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/track)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcTrackEvent`*
    pub fn track(this: &RtcTrackEvent) -> MediaStreamTrack;

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCTrackEvent" , js_name = streams ) ]
    ///Getter for the `streams` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/streams)
    ///
    ///*This API requires the following crate features to be activated: `RtcTrackEvent`*
    pub fn streams(this: &RtcTrackEvent) -> ::js_sys::Array;

    #[cfg(feature = "RtcRtpTransceiver")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCTrackEvent" , js_name = transceiver ) ]
    ///Getter for the `transceiver` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/transceiver)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcTrackEvent`*
    pub fn transceiver(this: &RtcTrackEvent) -> RtcRtpTransceiver;

    #[cfg(feature = "RtcTrackEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCTrackEvent")]
    ///The `new RtcTrackEvent(..)` constructor, creating a new instance of `RtcTrackEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/RTCTrackEvent)
    ///
    ///*This API requires the following crate features to be activated: `RtcTrackEvent`, `RtcTrackEventInit`*
    pub fn new(type_: &str, event_init_dict: &RtcTrackEventInit) -> Result<RtcTrackEvent, JsValue>;

}
