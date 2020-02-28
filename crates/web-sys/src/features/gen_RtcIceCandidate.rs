use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCIceCandidate , typescript_name = RTCIceCandidate ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceCandidate` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    pub type RtcIceCandidate;
    # [ wasm_bindgen ( structural , method , getter , js_name = candidate ) ]
    #[doc = "Getter for the `candidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/candidate)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    pub fn candidate(this: &RtcIceCandidate) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = candidate ) ]
    #[doc = "Setter for the `candidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/candidate)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    pub fn set_candidate(this: &RtcIceCandidate, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = sdpMid ) ]
    #[doc = "Getter for the `sdpMid` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMid)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    pub fn sdp_mid(this: &RtcIceCandidate) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_name = sdpMid ) ]
    #[doc = "Setter for the `sdpMid` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMid)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    pub fn set_sdp_mid(this: &RtcIceCandidate, value: Option<String>);
    # [ wasm_bindgen ( structural , method , getter , js_name = sdpMLineIndex ) ]
    #[doc = "Getter for the `sdpMLineIndex` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMLineIndex)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    pub fn sdp_m_line_index(this: &RtcIceCandidate) -> Option<u16>;
    # [ wasm_bindgen ( structural , method , setter , js_name = sdpMLineIndex ) ]
    #[doc = "Setter for the `sdpMLineIndex` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMLineIndex)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    pub fn set_sdp_m_line_index(this: &RtcIceCandidate, value: Option<u16>);
    #[cfg(feature = "RtcIceCandidateInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new RtcIceCandidate(..)` constructor, creating a new instance of `RtcIceCandidate`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/RTCIceCandidate)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcIceCandidateInit`*"]
    pub fn new(
        this: &RtcIceCandidate,
        candidate_init_dict: &RtcIceCandidateInit,
    ) -> Result<RtcIceCandidate, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = toJSON ) ]
    #[doc = "The `toJSON()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/toJSON)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
    pub fn to_json(this: &RtcIceCandidate) -> ::js_sys::Object;
}
