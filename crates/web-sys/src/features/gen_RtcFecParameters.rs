use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCFecParameters ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcFecParameters` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RtcFecParameters`*
    pub type RtcFecParameters;

}

impl RtcFecParameters {
    ///Construct a new `RtcFecParameters`.
    ///
    ///*This API requires the following crate features to be activated: `RtcFecParameters`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `ssrc` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcFecParameters`*

    pub fn ssrc(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("ssrc"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
