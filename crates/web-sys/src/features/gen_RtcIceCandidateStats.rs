use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCIceCandidateStats ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcIceCandidateStats` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidateStats`*
    pub type RtcIceCandidateStats;

}

impl RtcIceCandidateStats {
    ///Construct a new `RtcIceCandidateStats`.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidateStats`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `id` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidateStats`*

    pub fn id(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("id"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `timestamp` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidateStats`*

    pub fn timestamp(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("timestamp"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "RtcStatsType")]
    ///Change the `type` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidateStats`, `RtcStatsType`*

    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("type"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `candidateId` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidateStats`*

    pub fn candidate_id(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("candidateId"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "RtcStatsIceCandidateType")]
    ///Change the `candidateType` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidateStats`, `RtcStatsIceCandidateType`*

    pub fn candidate_type(&mut self, val: RtcStatsIceCandidateType) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("candidateType"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `componentId` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidateStats`*

    pub fn component_id(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("componentId"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `ipAddress` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidateStats`*

    pub fn ip_address(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("ipAddress"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `portNumber` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidateStats`*

    pub fn port_number(&mut self, val: i32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("portNumber"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `transport` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceCandidateStats`*

    pub fn transport(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("transport"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
