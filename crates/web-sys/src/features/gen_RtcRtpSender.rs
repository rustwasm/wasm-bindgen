use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCRtpSender , typescript_name = RTCRtpSender ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpSender` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender)\n\n*This API requires the following crate features to be activated: `RtcRtpSender`*"]
    pub type RtcRtpSender;
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpSender" , js_name = track ) ]
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Getter for the `track` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/track)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpSender`*"]
    pub fn track(this: &RtcRtpSender) -> Option<MediaStreamTrack>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpSender" , js_name = dtmf ) ]
    #[cfg(feature = "RtcdtmfSender")]
    #[doc = "Getter for the `dtmf` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/dtmf)\n\n*This API requires the following crate features to be activated: `RtcRtpSender`, `RtcdtmfSender`*"]
    pub fn dtmf(this: &RtcRtpSender) -> Option<RtcdtmfSender>;
    #[cfg(feature = "RtcRtpParameters")]
    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpSender" , js_name = getParameters ) ]
    #[doc = "The `getParameters()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getParameters)\n\n*This API requires the following crate features to be activated: `RtcRtpParameters`, `RtcRtpSender`*"]
    pub fn get_parameters(this: &RtcRtpSender) -> RtcRtpParameters;
    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpSender" , js_name = getStats ) ]
    #[doc = "The `getStats()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getStats)\n\n*This API requires the following crate features to be activated: `RtcRtpSender`*"]
    pub fn get_stats(this: &RtcRtpSender) -> ::js_sys::Promise;
    #[cfg(feature = "MediaStreamTrack")]
    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpSender" , js_name = replaceTrack ) ]
    #[doc = "The `replaceTrack()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/replaceTrack)\n\n*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpSender`*"]
    pub fn replace_track(
        this: &RtcRtpSender,
        with_track: Option<&MediaStreamTrack>,
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpSender" , js_name = setParameters ) ]
    #[doc = "The `setParameters()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/setParameters)\n\n*This API requires the following crate features to be activated: `RtcRtpSender`*"]
    pub fn set_parameters(this: &RtcRtpSender) -> ::js_sys::Promise;
    #[cfg(feature = "RtcRtpParameters")]
    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpSender" , js_name = setParameters ) ]
    #[doc = "The `setParameters()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/setParameters)\n\n*This API requires the following crate features to be activated: `RtcRtpParameters`, `RtcRtpSender`*"]
    pub fn set_parameters_with_parameters(
        this: &RtcRtpSender,
        parameters: &RtcRtpParameters,
    ) -> ::js_sys::Promise;
}
