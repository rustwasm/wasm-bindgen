use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCAnswerOptions ) ]
    #[doc = "The `RtcAnswerOptions` dictionary.\n\n*This API requires the following crate features to be activated: `RtcAnswerOptions`*"]
    pub type RtcAnswerOptions;
}
impl RtcAnswerOptions {
    #[doc = "Construct a new `RtcAnswerOptions`.\n\n*This API requires the following crate features to be activated: `RtcAnswerOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
