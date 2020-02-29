use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCSessionDescription , typescript_name = RTCSessionDescription ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcSessionDescription` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription)
    ///
    ///*This API requires the following crate features to be activated: `RtcSessionDescription`*
    pub type RtcSessionDescription;

    #[cfg(feature = "RtcSdpType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCSessionDescription" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/type)
    ///
    ///*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescription`*
    pub fn type_(this: &RtcSessionDescription) -> RtcSdpType;

    #[cfg(feature = "RtcSdpType")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "RTCSessionDescription" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/type)
    ///
    ///*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescription`*
    pub fn set_type(this: &RtcSessionDescription, value: RtcSdpType);

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCSessionDescription" , js_name = sdp ) ]
    ///Getter for the `sdp` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/sdp)
    ///
    ///*This API requires the following crate features to be activated: `RtcSessionDescription`*
    pub fn sdp(this: &RtcSessionDescription) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "RTCSessionDescription" , js_name = sdp ) ]
    ///Setter for the `sdp` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/sdp)
    ///
    ///*This API requires the following crate features to be activated: `RtcSessionDescription`*
    pub fn set_sdp(this: &RtcSessionDescription, value: &str);

    #[wasm_bindgen(catch, constructor, js_class = "RTCSessionDescription")]
    ///The `new RtcSessionDescription(..)` constructor, creating a new instance of `RtcSessionDescription`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/RTCSessionDescription)
    ///
    ///*This API requires the following crate features to be activated: `RtcSessionDescription`*
    pub fn new() -> Result<RtcSessionDescription, JsValue>;

    #[cfg(feature = "RtcSessionDescriptionInit")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCSessionDescription")]
    ///The `new RtcSessionDescription(..)` constructor, creating a new instance of `RtcSessionDescription`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/RTCSessionDescription)
    ///
    ///*This API requires the following crate features to be activated: `RtcSessionDescription`, `RtcSessionDescriptionInit`*
    pub fn new_with_description_init_dict(
        description_init_dict: &RtcSessionDescriptionInit,
    ) -> Result<RtcSessionDescription, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "RTCSessionDescription" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `RtcSessionDescription`*
    pub fn to_json(this: &RtcSessionDescription) -> ::js_sys::Object;

}
