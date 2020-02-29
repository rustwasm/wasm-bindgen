use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCRtpSender , typescript_name = RTCRtpSender ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcRtpSender` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpSender`*
    pub type RtcRtpSender;

    #[cfg(feature = "MediaStreamTrack")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpSender" , js_name = track ) ]
    ///Getter for the `track` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/track)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpSender`*
    pub fn track(this: &RtcRtpSender) -> Option<MediaStreamTrack>;

    #[cfg(feature = "RtcdtmfSender")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpSender" , js_name = dtmf ) ]
    ///Getter for the `dtmf` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/dtmf)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpSender`, `RtcdtmfSender`*
    pub fn dtmf(this: &RtcRtpSender) -> Option<RtcdtmfSender>;

    #[cfg(feature = "RtcRtpParameters")]
    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpSender" , js_name = getParameters ) ]
    ///The `getParameters()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getParameters)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpParameters`, `RtcRtpSender`*
    pub fn get_parameters(this: &RtcRtpSender) -> RtcRtpParameters;

    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpSender" , js_name = getStats ) ]
    ///The `getStats()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getStats)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpSender`*
    pub fn get_stats(this: &RtcRtpSender) -> ::js_sys::Promise;

    #[cfg(feature = "MediaStreamTrack")]
    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpSender" , js_name = replaceTrack ) ]
    ///The `replaceTrack()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/replaceTrack)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpSender`*
    pub fn replace_track(
        this: &RtcRtpSender,
        with_track: Option<&MediaStreamTrack>,
    ) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpSender" , js_name = setParameters ) ]
    ///The `setParameters()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/setParameters)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpSender`*
    pub fn set_parameters(this: &RtcRtpSender) -> ::js_sys::Promise;

    #[cfg(feature = "RtcRtpParameters")]
    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpSender" , js_name = setParameters ) ]
    ///The `setParameters()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/setParameters)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpParameters`, `RtcRtpSender`*
    pub fn set_parameters_with_parameters(
        this: &RtcRtpSender,
        parameters: &RtcRtpParameters,
    ) -> ::js_sys::Promise;

}
