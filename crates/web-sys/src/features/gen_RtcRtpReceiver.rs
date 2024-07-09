#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtpReceiver , typescript_type = "RTCRtpReceiver")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpReceiver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
    pub type RtcRtpReceiver;
    #[cfg(feature = "MediaStreamTrack")]
    # [wasm_bindgen (structural , method , getter , js_class = "RTCRtpReceiver" , js_name = track)]
    #[doc = "Getter for the `track` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/track)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpReceiver`*"]
    pub fn track(this: &RtcRtpReceiver) -> MediaStreamTrack;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "RTCRtpReceiver" , js_name = jitterBufferTarget)]
    #[doc = "Getter for the `jitterBufferTarget` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/jitterBufferTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn jitter_buffer_target(this: &RtcRtpReceiver) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , catch , method , setter , js_class = "RTCRtpReceiver" , js_name = jitterBufferTarget)]
    #[doc = "Setter for the `jitterBufferTarget` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/jitterBufferTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_jitter_buffer_target(
        this: &RtcRtpReceiver,
        value: Option<f64>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "RtcRtpCapabilities")]
    # [wasm_bindgen (static_method_of = RtcRtpReceiver , js_class = "RTCRtpReceiver" , js_name = getCapabilities)]
    #[doc = "The `getCapabilities()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getCapabilities)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`, `RtcRtpReceiver`*"]
    pub fn get_capabilities(kind: &str) -> Option<RtcRtpCapabilities>;
    # [wasm_bindgen (method , structural , js_class = "RTCRtpReceiver" , js_name = getContributingSources)]
    #[doc = "The `getContributingSources()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getContributingSources)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
    pub fn get_contributing_sources(this: &RtcRtpReceiver) -> ::js_sys::Array;
    # [wasm_bindgen (method , structural , js_class = "RTCRtpReceiver" , js_name = getStats)]
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getStats)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
    pub fn get_stats(this: &RtcRtpReceiver) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "RTCRtpReceiver" , js_name = getSynchronizationSources)]
    #[doc = "The `getSynchronizationSources()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getSynchronizationSources)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
    pub fn get_synchronization_sources(this: &RtcRtpReceiver) -> ::js_sys::Array;
}
