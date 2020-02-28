use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = RTCDataChannelEvent , typescript_name = RTCDataChannelEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcDataChannelEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent)\n\n*This API requires the following crate features to be activated: `RtcDataChannelEvent`*"]
    pub type RtcDataChannelEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = channel ) ]
    #[cfg(feature = "RtcDataChannel")]
    #[doc = "Getter for the `channel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent/channel)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelEvent`*"]
    pub fn channel(this: &RtcDataChannelEvent) -> RtcDataChannel;
    #[cfg(feature = "RtcDataChannelEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new RtcDataChannelEvent(..)` constructor, creating a new instance of `RtcDataChannelEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent/RTCDataChannelEvent)\n\n*This API requires the following crate features to be activated: `RtcDataChannelEvent`, `RtcDataChannelEventInit`*"]
    pub fn new(
        this: &RtcDataChannelEvent,
        type_: &str,
        event_init_dict: &RtcDataChannelEventInit,
    ) -> Result<RtcDataChannelEvent, JsValue>;
}
