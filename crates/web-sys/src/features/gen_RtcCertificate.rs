use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCCertificate , typescript_name = RTCCertificate ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcCertificate` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCCertificate)\n\n*This API requires the following crate features to be activated: `RtcCertificate`*"]
    pub type RtcCertificate;
    # [ wasm_bindgen ( structural , method , getter , js_name = expires ) ]
    #[doc = "Getter for the `expires` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCCertificate/expires)\n\n*This API requires the following crate features to be activated: `RtcCertificate`*"]
    pub fn expires(this: &RtcCertificate) -> f64;
}
