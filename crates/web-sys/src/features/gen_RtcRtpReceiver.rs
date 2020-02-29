use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCRtpReceiver , typescript_name = RTCRtpReceiver ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcRtpReceiver` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpReceiver`*
    pub type RtcRtpReceiver;

    #[cfg(feature = "MediaStreamTrack")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCRtpReceiver" , js_name = track ) ]
    ///Getter for the `track` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/track)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpReceiver`*
    pub fn track(this: &RtcRtpReceiver) -> MediaStreamTrack;

    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpReceiver" , js_name = getContributingSources ) ]
    ///The `getContributingSources()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getContributingSources)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpReceiver`*
    pub fn get_contributing_sources(this: &RtcRtpReceiver) -> ::js_sys::Array;

    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpReceiver" , js_name = getStats ) ]
    ///The `getStats()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getStats)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpReceiver`*
    pub fn get_stats(this: &RtcRtpReceiver) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "RTCRtpReceiver" , js_name = getSynchronizationSources ) ]
    ///The `getSynchronizationSources()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getSynchronizationSources)
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpReceiver`*
    pub fn get_synchronization_sources(this: &RtcRtpReceiver) -> ::js_sys::Array;

}
