use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCIceCandidateInit ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcIceCandidateInit` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidateInit`*
    pub type RtcIceCandidateInit;

}

impl RtcIceCandidateInit {
    ///Construct a new `RtcIceCandidateInit`.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidateInit`*

    pub fn new(candidate: &str) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.candidate(candidate);

        ret
    }

    ///Change the `candidate` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidateInit`*

    pub fn candidate(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("candidate"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `sdpMLineIndex` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidateInit`*

    pub fn sdp_m_line_index(&mut self, val: Option<u16>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("sdpMLineIndex"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `sdpMid` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidateInit`*

    pub fn sdp_mid(&mut self, val: Option<&str>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("sdpMid"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
