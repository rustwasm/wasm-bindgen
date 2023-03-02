#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCSessionDescription , typescript_type = "RTCSessionDescription")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcSessionDescription` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
    pub type RtcSessionDescription;
    #[cfg(feature = "RtcSdpType")]
    # [wasm_bindgen (structural , method , getter , js_class = "RTCSessionDescription" , js_name = type)]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescription`*"]
    pub fn type_(this: &RtcSessionDescription) -> RtcSdpType;
    # [wasm_bindgen (structural , method , getter , js_class = "RTCSessionDescription" , js_name = sdp)]
    #[doc = "Getter for the `sdp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/sdp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
    pub fn sdp(this: &RtcSessionDescription) -> String;
    #[cfg(feature = "RtcSessionDescriptionInit")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCSessionDescription")]
    #[doc = "The `new RtcSessionDescription(..)` constructor, creating a new instance of `RtcSessionDescription`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/RTCSessionDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescription`, `RtcSessionDescriptionInit`*"]
    pub fn new(
        description_init_dict: &RtcSessionDescriptionInit,
    ) -> Result<RtcSessionDescription, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "RTCSessionDescription" , js_name = toJSON)]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
    pub fn to_json(this: &RtcSessionDescription) -> ::js_sys::Object;
}
