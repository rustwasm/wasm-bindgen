use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = RTCDTMFToneChangeEvent , typescript_type = "RTCDTMFToneChangeEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcdtmfToneChangeEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent)
    ///
    ///*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`*
    pub type RtcdtmfToneChangeEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCDTMFToneChangeEvent" , js_name = tone ) ]
    ///Getter for the `tone` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/tone)
    ///
    ///*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`*
    pub fn tone(this: &RtcdtmfToneChangeEvent) -> String;

    #[wasm_bindgen(catch, constructor, js_class = "RTCDTMFToneChangeEvent")]
    ///The `new RtcdtmfToneChangeEvent(..)` constructor, creating a new instance of `RtcdtmfToneChangeEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/RTCDTMFToneChangeEvent)
    ///
    ///*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`*
    pub fn new(type_: &str) -> Result<RtcdtmfToneChangeEvent, JsValue>;

    #[cfg(feature = "RtcdtmfToneChangeEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCDTMFToneChangeEvent")]
    ///The `new RtcdtmfToneChangeEvent(..)` constructor, creating a new instance of `RtcdtmfToneChangeEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/RTCDTMFToneChangeEvent)
    ///
    ///*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`, `RtcdtmfToneChangeEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &RtcdtmfToneChangeEventInit,
    ) -> Result<RtcdtmfToneChangeEvent, JsValue>;

}
