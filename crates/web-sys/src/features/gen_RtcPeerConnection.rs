use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = RTCPeerConnection , typescript_name = RTCPeerConnection ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcPeerConnection` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub type RtcPeerConnection;
    # [ wasm_bindgen ( structural , method , getter , js_name = localDescription ) ]
    #[cfg(feature = "RtcSessionDescription")]
    #[doc = "Getter for the `localDescription` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/localDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    pub fn local_description(this: &RtcPeerConnection) -> Option<RtcSessionDescription>;
    # [ wasm_bindgen ( structural , method , getter , js_name = currentLocalDescription ) ]
    #[cfg(feature = "RtcSessionDescription")]
    #[doc = "Getter for the `currentLocalDescription` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/currentLocalDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    pub fn current_local_description(this: &RtcPeerConnection) -> Option<RtcSessionDescription>;
    # [ wasm_bindgen ( structural , method , getter , js_name = pendingLocalDescription ) ]
    #[cfg(feature = "RtcSessionDescription")]
    #[doc = "Getter for the `pendingLocalDescription` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/pendingLocalDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    pub fn pending_local_description(this: &RtcPeerConnection) -> Option<RtcSessionDescription>;
    # [ wasm_bindgen ( structural , method , getter , js_name = remoteDescription ) ]
    #[cfg(feature = "RtcSessionDescription")]
    #[doc = "Getter for the `remoteDescription` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/remoteDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    pub fn remote_description(this: &RtcPeerConnection) -> Option<RtcSessionDescription>;
    # [ wasm_bindgen ( structural , method , getter , js_name = currentRemoteDescription ) ]
    #[cfg(feature = "RtcSessionDescription")]
    #[doc = "Getter for the `currentRemoteDescription` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/currentRemoteDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    pub fn current_remote_description(this: &RtcPeerConnection) -> Option<RtcSessionDescription>;
    # [ wasm_bindgen ( structural , method , getter , js_name = pendingRemoteDescription ) ]
    #[cfg(feature = "RtcSessionDescription")]
    #[doc = "Getter for the `pendingRemoteDescription` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/pendingRemoteDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
    pub fn pending_remote_description(this: &RtcPeerConnection) -> Option<RtcSessionDescription>;
    # [ wasm_bindgen ( structural , method , getter , js_name = signalingState ) ]
    #[cfg(feature = "RtcSignalingState")]
    #[doc = "Getter for the `signalingState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/signalingState)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSignalingState`*"]
    pub fn signaling_state(this: &RtcPeerConnection) -> RtcSignalingState;
    # [ wasm_bindgen ( structural , method , getter , js_name = canTrickleIceCandidates ) ]
    #[doc = "Getter for the `canTrickleIceCandidates` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/canTrickleIceCandidates)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn can_trickle_ice_candidates(this: &RtcPeerConnection) -> Option<bool>;
    # [ wasm_bindgen ( structural , method , getter , js_name = iceGatheringState ) ]
    #[cfg(feature = "RtcIceGatheringState")]
    #[doc = "Getter for the `iceGatheringState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/iceGatheringState)\n\n*This API requires the following crate features to be activated: `RtcIceGatheringState`, `RtcPeerConnection`*"]
    pub fn ice_gathering_state(this: &RtcPeerConnection) -> RtcIceGatheringState;
    # [ wasm_bindgen ( structural , method , getter , js_name = iceConnectionState ) ]
    #[cfg(feature = "RtcIceConnectionState")]
    #[doc = "Getter for the `iceConnectionState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/iceConnectionState)\n\n*This API requires the following crate features to be activated: `RtcIceConnectionState`, `RtcPeerConnection`*"]
    pub fn ice_connection_state(this: &RtcPeerConnection) -> RtcIceConnectionState;
    # [ wasm_bindgen ( structural , method , getter , js_name = peerIdentity ) ]
    #[doc = "Getter for the `peerIdentity` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/peerIdentity)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn peer_identity(this: &RtcPeerConnection) -> ::js_sys::Promise;
    # [ wasm_bindgen ( structural , method , getter , js_name = idpLoginUrl ) ]
    #[doc = "Getter for the `idpLoginUrl` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/idpLoginUrl)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn idp_login_url(this: &RtcPeerConnection) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = onnegotiationneeded ) ]
    #[doc = "Getter for the `onnegotiationneeded` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onnegotiationneeded)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn onnegotiationneeded(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onnegotiationneeded ) ]
    #[doc = "Setter for the `onnegotiationneeded` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onnegotiationneeded)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_onnegotiationneeded(this: &RtcPeerConnection, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onicecandidate ) ]
    #[doc = "Getter for the `onicecandidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicecandidate)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn onicecandidate(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onicecandidate ) ]
    #[doc = "Setter for the `onicecandidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicecandidate)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_onicecandidate(this: &RtcPeerConnection, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onsignalingstatechange ) ]
    #[doc = "Getter for the `onsignalingstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onsignalingstatechange)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn onsignalingstatechange(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onsignalingstatechange ) ]
    #[doc = "Setter for the `onsignalingstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onsignalingstatechange)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_onsignalingstatechange(this: &RtcPeerConnection, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onaddstream ) ]
    #[doc = "Getter for the `onaddstream` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onaddstream)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn onaddstream(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onaddstream ) ]
    #[doc = "Setter for the `onaddstream` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onaddstream)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_onaddstream(this: &RtcPeerConnection, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onaddtrack ) ]
    #[doc = "Getter for the `onaddtrack` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onaddtrack)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn onaddtrack(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onaddtrack ) ]
    #[doc = "Setter for the `onaddtrack` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onaddtrack)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_onaddtrack(this: &RtcPeerConnection, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ontrack ) ]
    #[doc = "Getter for the `ontrack` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ontrack)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn ontrack(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ontrack ) ]
    #[doc = "Setter for the `ontrack` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ontrack)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_ontrack(this: &RtcPeerConnection, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onremovestream ) ]
    #[doc = "Getter for the `onremovestream` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onremovestream)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn onremovestream(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onremovestream ) ]
    #[doc = "Setter for the `onremovestream` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onremovestream)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_onremovestream(this: &RtcPeerConnection, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = oniceconnectionstatechange ) ]
    #[doc = "Getter for the `oniceconnectionstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/oniceconnectionstatechange)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn oniceconnectionstatechange(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oniceconnectionstatechange ) ]
    #[doc = "Setter for the `oniceconnectionstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/oniceconnectionstatechange)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_oniceconnectionstatechange(
        this: &RtcPeerConnection,
        value: Option<&::js_sys::Function>,
    );
    # [ wasm_bindgen ( structural , method , getter , js_name = onicegatheringstatechange ) ]
    #[doc = "Getter for the `onicegatheringstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicegatheringstatechange)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn onicegatheringstatechange(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onicegatheringstatechange ) ]
    #[doc = "Setter for the `onicegatheringstatechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicegatheringstatechange)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_onicegatheringstatechange(
        this: &RtcPeerConnection,
        value: Option<&::js_sys::Function>,
    );
    # [ wasm_bindgen ( structural , method , getter , js_name = ondatachannel ) ]
    #[doc = "Getter for the `ondatachannel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ondatachannel)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn ondatachannel(this: &RtcPeerConnection) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondatachannel ) ]
    #[doc = "Setter for the `ondatachannel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ondatachannel)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_ondatachannel(this: &RtcPeerConnection, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new RtcPeerConnection(..)` constructor, creating a new instance of `RtcPeerConnection`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/RTCPeerConnection)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn new(this: &RtcPeerConnection) -> Result<RtcPeerConnection, JsValue>;
    #[cfg(feature = "RtcConfiguration")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new RtcPeerConnection(..)` constructor, creating a new instance of `RtcPeerConnection`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/RTCPeerConnection)\n\n*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcPeerConnection`*"]
    pub fn new_with_configuration(
        this: &RtcPeerConnection,
        configuration: &RtcConfiguration,
    ) -> Result<RtcPeerConnection, JsValue>;
    #[cfg(feature = "RtcConfiguration")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new RtcPeerConnection(..)` constructor, creating a new instance of `RtcPeerConnection`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/RTCPeerConnection)\n\n*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcPeerConnection`*"]
    pub fn new_with_configuration_and_constraints(
        this: &RtcPeerConnection,
        configuration: &RtcConfiguration,
        constraints: Option<&::js_sys::Object>,
    ) -> Result<RtcPeerConnection, JsValue>;
    #[cfg(feature = "RtcIceCandidateInit")]
    # [ wasm_bindgen ( method , structural , js_name = addIceCandidate ) ]
    #[doc = "The `addIceCandidate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addIceCandidate)\n\n*This API requires the following crate features to be activated: `RtcIceCandidateInit`, `RtcPeerConnection`*"]
    pub fn add_ice_candidate_with_opt_rtc_ice_candidate_init(
        this: &RtcPeerConnection,
        candidate: Option<&RtcIceCandidateInit>,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "RtcIceCandidate")]
    # [ wasm_bindgen ( method , structural , js_name = addIceCandidate ) ]
    #[doc = "The `addIceCandidate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addIceCandidate)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnection`*"]
    pub fn add_ice_candidate_with_opt_rtc_ice_candidate(
        this: &RtcPeerConnection,
        candidate: Option<&RtcIceCandidate>,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "RtcIceCandidate")]
    # [ wasm_bindgen ( method , structural , js_name = addIceCandidate ) ]
    #[doc = "The `addIceCandidate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addIceCandidate)\n\n*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnection`*"]
    pub fn add_ice_candidate_with_rtc_ice_candidate_and_success_callback_and_failure_callback(
        this: &RtcPeerConnection,
        candidate: &RtcIceCandidate,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "MediaStream")]
    # [ wasm_bindgen ( method , structural , js_name = addStream ) ]
    #[doc = "The `addStream()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addStream)\n\n*This API requires the following crate features to be activated: `MediaStream`, `RtcPeerConnection`*"]
    pub fn add_stream(this: &RtcPeerConnection, stream: &MediaStream);
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    # [ wasm_bindgen ( method , structural , variadic , js_name = addTrack ) ]
    #[doc = "The `addTrack()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    # [ wasm_bindgen ( method , structural , js_name = addTrack ) ]
    #[doc = "The `addTrack()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track_0(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    # [ wasm_bindgen ( method , structural , js_name = addTrack ) ]
    #[doc = "The `addTrack()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track_1(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    # [ wasm_bindgen ( method , structural , js_name = addTrack ) ]
    #[doc = "The `addTrack()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track_2(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    # [ wasm_bindgen ( method , structural , js_name = addTrack ) ]
    #[doc = "The `addTrack()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track_3(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
        more_streams_3: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    # [ wasm_bindgen ( method , structural , js_name = addTrack ) ]
    #[doc = "The `addTrack()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track_4(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
        more_streams_3: &MediaStream,
        more_streams_4: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    # [ wasm_bindgen ( method , structural , js_name = addTrack ) ]
    #[doc = "The `addTrack()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track_5(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
        more_streams_3: &MediaStream,
        more_streams_4: &MediaStream,
        more_streams_5: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    # [ wasm_bindgen ( method , structural , js_name = addTrack ) ]
    #[doc = "The `addTrack()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track_6(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
        more_streams_3: &MediaStream,
        more_streams_4: &MediaStream,
        more_streams_5: &MediaStream,
        more_streams_6: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    # [ wasm_bindgen ( method , structural , js_name = addTrack ) ]
    #[doc = "The `addTrack()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn add_track_7(
        this: &RtcPeerConnection,
        track: &MediaStreamTrack,
        stream: &MediaStream,
        more_streams_1: &MediaStream,
        more_streams_2: &MediaStream,
        more_streams_3: &MediaStream,
        more_streams_4: &MediaStream,
        more_streams_5: &MediaStream,
        more_streams_6: &MediaStream,
        more_streams_7: &MediaStream,
    ) -> RtcRtpSender;
    #[cfg(all(feature = "MediaStreamTrack", feature = "RtcRtpTransceiver",))]
    # [ wasm_bindgen ( method , structural , js_name = addTransceiver ) ]
    #[doc = "The `addTransceiver()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpTransceiver`*"]
    pub fn add_transceiver_with_media_stream_track(
        this: &RtcPeerConnection,
        track_or_kind: &MediaStreamTrack,
    ) -> RtcRtpTransceiver;
    #[cfg(feature = "RtcRtpTransceiver")]
    # [ wasm_bindgen ( method , structural , js_name = addTransceiver ) ]
    #[doc = "The `addTransceiver()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcRtpTransceiver`*"]
    pub fn add_transceiver_with_str(
        this: &RtcPeerConnection,
        track_or_kind: &str,
    ) -> RtcRtpTransceiver;
    #[cfg(all(
        feature = "MediaStreamTrack",
        feature = "RtcRtpTransceiver",
        feature = "RtcRtpTransceiverInit",
    ))]
    # [ wasm_bindgen ( method , structural , js_name = addTransceiver ) ]
    #[doc = "The `addTransceiver()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpTransceiver`, `RtcRtpTransceiverInit`*"]
    pub fn add_transceiver_with_media_stream_track_and_init(
        this: &RtcPeerConnection,
        track_or_kind: &MediaStreamTrack,
        init: &RtcRtpTransceiverInit,
    ) -> RtcRtpTransceiver;
    #[cfg(all(feature = "RtcRtpTransceiver", feature = "RtcRtpTransceiverInit",))]
    # [ wasm_bindgen ( method , structural , js_name = addTransceiver ) ]
    #[doc = "The `addTransceiver()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcRtpTransceiver`, `RtcRtpTransceiverInit`*"]
    pub fn add_transceiver_with_str_and_init(
        this: &RtcPeerConnection,
        track_or_kind: &str,
        init: &RtcRtpTransceiverInit,
    ) -> RtcRtpTransceiver;
    # [ wasm_bindgen ( method , structural , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/close)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn close(this: &RtcPeerConnection);
    # [ wasm_bindgen ( method , structural , js_name = createAnswer ) ]
    #[doc = "The `createAnswer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createAnswer)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn create_answer(this: &RtcPeerConnection) -> ::js_sys::Promise;
    #[cfg(feature = "RtcAnswerOptions")]
    # [ wasm_bindgen ( method , structural , js_name = createAnswer ) ]
    #[doc = "The `createAnswer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createAnswer)\n\n*This API requires the following crate features to be activated: `RtcAnswerOptions`, `RtcPeerConnection`*"]
    pub fn create_answer_with_rtc_answer_options(
        this: &RtcPeerConnection,
        options: &RtcAnswerOptions,
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_name = createAnswer ) ]
    #[doc = "The `createAnswer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createAnswer)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn create_answer_with_success_callback_and_failure_callback(
        this: &RtcPeerConnection,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "RtcDataChannel")]
    # [ wasm_bindgen ( method , structural , js_name = createDataChannel ) ]
    #[doc = "The `createDataChannel()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createDataChannel)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcPeerConnection`*"]
    pub fn create_data_channel(this: &RtcPeerConnection, label: &str) -> RtcDataChannel;
    #[cfg(all(feature = "RtcDataChannel", feature = "RtcDataChannelInit",))]
    # [ wasm_bindgen ( method , structural , js_name = createDataChannel ) ]
    #[doc = "The `createDataChannel()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createDataChannel)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelInit`, `RtcPeerConnection`*"]
    pub fn create_data_channel_with_data_channel_dict(
        this: &RtcPeerConnection,
        label: &str,
        data_channel_dict: &RtcDataChannelInit,
    ) -> RtcDataChannel;
    # [ wasm_bindgen ( method , structural , js_name = createOffer ) ]
    #[doc = "The `createOffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn create_offer(this: &RtcPeerConnection) -> ::js_sys::Promise;
    #[cfg(feature = "RtcOfferOptions")]
    # [ wasm_bindgen ( method , structural , js_name = createOffer ) ]
    #[doc = "The `createOffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)\n\n*This API requires the following crate features to be activated: `RtcOfferOptions`, `RtcPeerConnection`*"]
    pub fn create_offer_with_rtc_offer_options(
        this: &RtcPeerConnection,
        options: &RtcOfferOptions,
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_name = createOffer ) ]
    #[doc = "The `createOffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn create_offer_with_callback_and_failure_callback(
        this: &RtcPeerConnection,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "RtcOfferOptions")]
    # [ wasm_bindgen ( method , structural , js_name = createOffer ) ]
    #[doc = "The `createOffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)\n\n*This API requires the following crate features to be activated: `RtcOfferOptions`, `RtcPeerConnection`*"]
    pub fn create_offer_with_callback_and_failure_callback_and_options(
        this: &RtcPeerConnection,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
        options: &RtcOfferOptions,
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( catch , method , structural , static_method_of = RTCPeerConnection , js_name = generateCertificate ) ]
    #[doc = "The `generateCertificate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/generateCertificate)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn generate_certificate_with_object(
        keygen_algorithm: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , static_method_of = RTCPeerConnection , js_name = generateCertificate ) ]
    #[doc = "The `generateCertificate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/generateCertificate)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn generate_certificate_with_str(
        keygen_algorithm: &str,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "RtcConfiguration")]
    # [ wasm_bindgen ( method , structural , js_name = getConfiguration ) ]
    #[doc = "The `getConfiguration()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getConfiguration)\n\n*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcPeerConnection`*"]
    pub fn get_configuration(this: &RtcPeerConnection) -> RtcConfiguration;
    # [ wasm_bindgen ( method , structural , js_name = getIdentityAssertion ) ]
    #[doc = "The `getIdentityAssertion()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getIdentityAssertion)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn get_identity_assertion(this: &RtcPeerConnection) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_name = getLocalStreams ) ]
    #[doc = "The `getLocalStreams()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getLocalStreams)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn get_local_streams(this: &RtcPeerConnection) -> ::js_sys::Array;
    # [ wasm_bindgen ( method , structural , js_name = getReceivers ) ]
    #[doc = "The `getReceivers()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getReceivers)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn get_receivers(this: &RtcPeerConnection) -> ::js_sys::Array;
    # [ wasm_bindgen ( method , structural , js_name = getRemoteStreams ) ]
    #[doc = "The `getRemoteStreams()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getRemoteStreams)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn get_remote_streams(this: &RtcPeerConnection) -> ::js_sys::Array;
    # [ wasm_bindgen ( method , structural , js_name = getSenders ) ]
    #[doc = "The `getSenders()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getSenders)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn get_senders(this: &RtcPeerConnection) -> ::js_sys::Array;
    # [ wasm_bindgen ( method , structural , js_name = getStats ) ]
    #[doc = "The `getStats()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getStats)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn get_stats(this: &RtcPeerConnection) -> ::js_sys::Promise;
    #[cfg(feature = "MediaStreamTrack")]
    # [ wasm_bindgen ( method , structural , js_name = getStats ) ]
    #[doc = "The `getStats()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getStats)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcPeerConnection`*"]
    pub fn get_stats_with_selector(
        this: &RtcPeerConnection,
        selector: Option<&MediaStreamTrack>,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "MediaStreamTrack")]
    # [ wasm_bindgen ( method , structural , js_name = getStats ) ]
    #[doc = "The `getStats()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getStats)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcPeerConnection`*"]
    pub fn get_stats_with_selector_and_success_callback_and_failure_callback(
        this: &RtcPeerConnection,
        selector: Option<&MediaStreamTrack>,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_name = getTransceivers ) ]
    #[doc = "The `getTransceivers()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getTransceivers)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn get_transceivers(this: &RtcPeerConnection) -> ::js_sys::Array;
    #[cfg(feature = "RtcRtpSender")]
    # [ wasm_bindgen ( method , structural , js_name = removeTrack ) ]
    #[doc = "The `removeTrack()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/removeTrack)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcRtpSender`*"]
    pub fn remove_track(this: &RtcPeerConnection, sender: &RtcRtpSender);
    # [ wasm_bindgen ( method , structural , js_name = setIdentityProvider ) ]
    #[doc = "The `setIdentityProvider()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setIdentityProvider)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
    pub fn set_identity_provider(this: &RtcPeerConnection, provider: &str);
    #[cfg(feature = "RtcIdentityProviderOptions")]
    # [ wasm_bindgen ( method , structural , js_name = setIdentityProvider ) ]
    #[doc = "The `setIdentityProvider()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setIdentityProvider)\n\n*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`, `RtcPeerConnection`*"]
    pub fn set_identity_provider_with_options(
        this: &RtcPeerConnection,
        provider: &str,
        options: &RtcIdentityProviderOptions,
    );
    #[cfg(feature = "RtcSessionDescriptionInit")]
    # [ wasm_bindgen ( method , structural , js_name = setLocalDescription ) ]
    #[doc = "The `setLocalDescription()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setLocalDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescriptionInit`*"]
    pub fn set_local_description(
        this: &RtcPeerConnection,
        description: &RtcSessionDescriptionInit,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "RtcSessionDescriptionInit")]
    # [ wasm_bindgen ( method , structural , js_name = setLocalDescription ) ]
    #[doc = "The `setLocalDescription()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setLocalDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescriptionInit`*"]
    pub fn set_local_description_with_success_callback_and_failure_callback(
        this: &RtcPeerConnection,
        description: &RtcSessionDescriptionInit,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "RtcSessionDescriptionInit")]
    # [ wasm_bindgen ( method , structural , js_name = setRemoteDescription ) ]
    #[doc = "The `setRemoteDescription()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setRemoteDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescriptionInit`*"]
    pub fn set_remote_description(
        this: &RtcPeerConnection,
        description: &RtcSessionDescriptionInit,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "RtcSessionDescriptionInit")]
    # [ wasm_bindgen ( method , structural , js_name = setRemoteDescription ) ]
    #[doc = "The `setRemoteDescription()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setRemoteDescription)\n\n*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescriptionInit`*"]
    pub fn set_remote_description_with_success_callback_and_failure_callback(
        this: &RtcPeerConnection,
        description: &RtcSessionDescriptionInit,
        success_callback: &::js_sys::Function,
        failure_callback: &::js_sys::Function,
    ) -> ::js_sys::Promise;
}
