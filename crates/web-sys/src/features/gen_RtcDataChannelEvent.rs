use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = RTCDataChannelEvent , typescript_name = RTCDataChannelEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcDataChannelEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannelEvent`*
    pub type RtcDataChannelEvent;

    #[cfg(feature = "RtcDataChannel")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDataChannelEvent" , js_name = channel ) ]
    ///Getter for the `channel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent/channel)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelEvent`*
    pub fn channel(this: &RtcDataChannelEvent) -> RtcDataChannel;

    #[cfg(feature = "RtcDataChannelEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCDataChannelEvent")]
    ///The `new RtcDataChannelEvent(..)` constructor, creating a new instance of `RtcDataChannelEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent/RTCDataChannelEvent)
    ///
    ///*This API requires the following crate features to be activated: `RtcDataChannelEvent`, `RtcDataChannelEventInit`*
    pub fn new(
        type_: &str,
        event_init_dict: &RtcDataChannelEventInit,
    ) -> Result<RtcDataChannelEvent, JsValue>;

}
