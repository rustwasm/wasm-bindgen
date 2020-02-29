use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCAnswerOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcAnswerOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RtcAnswerOptions`*
    pub type RtcAnswerOptions;

}

impl RtcAnswerOptions {
    ///Construct a new `RtcAnswerOptions`.
    ///
    ///*This API requires the following crate features to be activated: `RtcAnswerOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }
}
