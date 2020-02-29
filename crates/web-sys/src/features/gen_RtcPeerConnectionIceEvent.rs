use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = RTCPeerConnectionIceEvent , typescript_type = "RTCPeerConnectionIceEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcPeerConnectionIceEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceEvent)
    ///
    ///*This API requires the following crate features to be activated: `RtcPeerConnectionIceEvent`*
    pub type RtcPeerConnectionIceEvent;

    #[cfg(feature = "RtcIceCandidate")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCPeerConnectionIceEvent" , js_name = candidate ) ]
    ///Getter for the `candidate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceEvent/candidate)
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnectionIceEvent`*
    pub fn candidate(this: &RtcPeerConnectionIceEvent) -> Option<RtcIceCandidate>;

    #[wasm_bindgen(catch, constructor, js_class = "RTCPeerConnectionIceEvent")]
    ///The `new RtcPeerConnectionIceEvent(..)` constructor, creating a new instance of `RtcPeerConnectionIceEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceEvent/RTCPeerConnectionIceEvent)
    ///
    ///*This API requires the following crate features to be activated: `RtcPeerConnectionIceEvent`*
    pub fn new(type_: &str) -> Result<RtcPeerConnectionIceEvent, JsValue>;

    #[cfg(feature = "RtcPeerConnectionIceEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCPeerConnectionIceEvent")]
    ///The `new RtcPeerConnectionIceEvent(..)` constructor, creating a new instance of `RtcPeerConnectionIceEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceEvent/RTCPeerConnectionIceEvent)
    ///
    ///*This API requires the following crate features to be activated: `RtcPeerConnectionIceEvent`, `RtcPeerConnectionIceEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &RtcPeerConnectionIceEventInit,
    ) -> Result<RtcPeerConnectionIceEvent, JsValue>;

}
