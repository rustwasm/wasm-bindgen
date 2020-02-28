use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCOfferAnswerOptions ) ]
    #[doc = "The `RtcOfferAnswerOptions` dictionary.\n\n*This API requires the following crate features to be activated: `RtcOfferAnswerOptions`*"]
    pub type RtcOfferAnswerOptions;
}
impl RtcOfferAnswerOptions {
    #[doc = "Construct a new `RtcOfferAnswerOptions`.\n\n*This API requires the following crate features to be activated: `RtcOfferAnswerOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
