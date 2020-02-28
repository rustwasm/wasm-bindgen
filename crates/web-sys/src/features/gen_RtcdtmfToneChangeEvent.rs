use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = RTCDTMFToneChangeEvent , typescript_name = RTCDTMFToneChangeEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcdtmfToneChangeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`*"]
    pub type RtcdtmfToneChangeEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDTMFToneChangeEvent" , js_name = tone ) ]
    #[doc = "Getter for the `tone` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/tone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`*"]
    pub fn tone(this: &RtcdtmfToneChangeEvent) -> String;
    #[wasm_bindgen(catch, js_class = "RTCDTMFToneChangeEvent", constructor)]
    #[doc = "The `new RtcdtmfToneChangeEvent(..)` constructor, creating a new instance of `RtcdtmfToneChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/RTCDTMFToneChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`*"]
    pub fn new(
        this: &RtcdtmfToneChangeEvent,
        type_: &str,
    ) -> Result<RtcdtmfToneChangeEvent, JsValue>;
    #[cfg(feature = "RtcdtmfToneChangeEventInit")]
    #[wasm_bindgen(catch, js_class = "RTCDTMFToneChangeEvent", constructor)]
    #[doc = "The `new RtcdtmfToneChangeEvent(..)` constructor, creating a new instance of `RtcdtmfToneChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/RTCDTMFToneChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`, `RtcdtmfToneChangeEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &RtcdtmfToneChangeEvent,
        type_: &str,
        event_init_dict: &RtcdtmfToneChangeEventInit,
    ) -> Result<RtcdtmfToneChangeEvent, JsValue>;
}
