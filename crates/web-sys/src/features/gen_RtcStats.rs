use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCStats ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcStats` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RtcStats`*
    pub type RtcStats;

}

impl RtcStats {
    ///Construct a new `RtcStats`.
    ///
    ///*This API requires the following crate features to be activated: `RtcStats`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `id` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcStats`*

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
    ///*This API requires the following crate features to be activated: `RtcStats`*

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
    ///*This API requires the following crate features to be activated: `RtcStats`, `RtcStatsType`*

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
}
