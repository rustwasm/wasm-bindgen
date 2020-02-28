use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = RTCTrackEvent , typescript_name = RTCTrackEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcTrackEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent)\n\n*This API requires the following crate features to be activated: `RtcTrackEvent`*"]
    pub type RtcTrackEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCTrackEvent" , js_name = receiver ) ]
    #[cfg(feature = "RtcRtpReceiver")]
    #[doc = "Getter for the `receiver` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/receiver)\n\n*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcTrackEvent`*"]
    pub fn receiver(this: &RtcTrackEvent) -> RtcRtpReceiver;
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCTrackEvent" , js_name = track ) ]
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Getter for the `track` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/track)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcTrackEvent`*"]
    pub fn track(this: &RtcTrackEvent) -> MediaStreamTrack;
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCTrackEvent" , js_name = streams ) ]
    #[doc = "Getter for the `streams` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/streams)\n\n*This API requires the following crate features to be activated: `RtcTrackEvent`*"]
    pub fn streams(this: &RtcTrackEvent) -> ::js_sys::Array;
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCTrackEvent" , js_name = transceiver ) ]
    #[cfg(feature = "RtcRtpTransceiver")]
    #[doc = "Getter for the `transceiver` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/transceiver)\n\n*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcTrackEvent`*"]
    pub fn transceiver(this: &RtcTrackEvent) -> RtcRtpTransceiver;
    #[cfg(feature = "RtcTrackEventInit")]
    #[wasm_bindgen(catch, js_class = "RTCTrackEvent", constructor)]
    #[doc = "The `new RtcTrackEvent(..)` constructor, creating a new instance of `RtcTrackEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/RTCTrackEvent)\n\n*This API requires the following crate features to be activated: `RtcTrackEvent`, `RtcTrackEventInit`*"]
    pub fn new(
        this: &RtcTrackEvent,
        type_: &str,
        event_init_dict: &RtcTrackEventInit,
    ) -> Result<RtcTrackEvent, JsValue>;
}
