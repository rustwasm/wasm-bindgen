use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCSessionDescription , typescript_name = RTCSessionDescription ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcSessionDescription` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription)\n\n*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
    pub type RtcSessionDescription;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[cfg(feature = "RtcSdpType")]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/type)\n\n*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescription`*"]
    pub fn type_(this: &RtcSessionDescription) -> RtcSdpType;
    # [ wasm_bindgen ( structural , method , setter , js_name = type ) ]
    #[cfg(feature = "RtcSdpType")]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/type)\n\n*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescription`*"]
    pub fn set_type(this: &RtcSessionDescription, value: RtcSdpType);
    # [ wasm_bindgen ( structural , method , getter , js_name = sdp ) ]
    #[doc = "Getter for the `sdp` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/sdp)\n\n*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
    pub fn sdp(this: &RtcSessionDescription) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = sdp ) ]
    #[doc = "Setter for the `sdp` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/sdp)\n\n*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
    pub fn set_sdp(this: &RtcSessionDescription, value: String);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new RtcSessionDescription(..)` constructor, creating a new instance of `RtcSessionDescription`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/RTCSessionDescription)\n\n*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
    pub fn new(this: &RtcSessionDescription) -> Result<RtcSessionDescription, JsValue>;
    #[cfg(feature = "RtcSessionDescriptionInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new RtcSessionDescription(..)` constructor, creating a new instance of `RtcSessionDescription`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/RTCSessionDescription)\n\n*This API requires the following crate features to be activated: `RtcSessionDescription`, `RtcSessionDescriptionInit`*"]
    pub fn new_with_description_init_dict(
        this: &RtcSessionDescription,
        description_init_dict: &RtcSessionDescriptionInit,
    ) -> Result<RtcSessionDescription, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = toJSON ) ]
    #[doc = "The `toJSON()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/toJSON)\n\n*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
    pub fn to_json(this: &RtcSessionDescription) -> ::js_sys::Object;
}
