use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCIceCandidate , typescript_type = "RTCIceCandidate" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcIceCandidate` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate)
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidate`*
    pub type RtcIceCandidate;

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCIceCandidate" , js_name = candidate ) ]
    ///Getter for the `candidate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/candidate)
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidate`*
    pub fn candidate(this: &RtcIceCandidate) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "RTCIceCandidate" , js_name = candidate ) ]
    ///Setter for the `candidate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/candidate)
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidate`*
    pub fn set_candidate(this: &RtcIceCandidate, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCIceCandidate" , js_name = sdpMid ) ]
    ///Getter for the `sdpMid` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMid)
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidate`*
    pub fn sdp_mid(this: &RtcIceCandidate) -> Option<String>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "RTCIceCandidate" , js_name = sdpMid ) ]
    ///Setter for the `sdpMid` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMid)
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidate`*
    pub fn set_sdp_mid(this: &RtcIceCandidate, value: Option<&str>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCIceCandidate" , js_name = sdpMLineIndex ) ]
    ///Getter for the `sdpMLineIndex` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMLineIndex)
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidate`*
    pub fn sdp_m_line_index(this: &RtcIceCandidate) -> Option<u16>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "RTCIceCandidate" , js_name = sdpMLineIndex ) ]
    ///Setter for the `sdpMLineIndex` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMLineIndex)
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidate`*
    pub fn set_sdp_m_line_index(this: &RtcIceCandidate, value: Option<u16>);

    #[cfg(feature = "RtcIceCandidateInit")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCIceCandidate")]
    ///The `new RtcIceCandidate(..)` constructor, creating a new instance of `RtcIceCandidate`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/RTCIceCandidate)
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcIceCandidateInit`*
    pub fn new(candidate_init_dict: &RtcIceCandidateInit) -> Result<RtcIceCandidate, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "RTCIceCandidate" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidate`*
    pub fn to_json(this: &RtcIceCandidate) -> ::js_sys::Object;

}
