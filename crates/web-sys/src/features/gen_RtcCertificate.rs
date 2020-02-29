use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCCertificate , typescript_name = RTCCertificate ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcCertificate` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCCertificate)
    ///
    ///*This API requires the following crate features to be activated: `RtcCertificate`*
    pub type RtcCertificate;

    # [ wasm_bindgen ( structural , method , getter , js_class = "RTCCertificate" , js_name = expires ) ]
    ///Getter for the `expires` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCCertificate/expires)
    ///
    ///*This API requires the following crate features to be activated: `RtcCertificate`*
    pub fn expires(this: &RtcCertificate) -> f64;

}
