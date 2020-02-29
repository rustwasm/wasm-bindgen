use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCTransportStats ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcTransportStats` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RtcTransportStats`*
    pub type RtcTransportStats;

}

impl RtcTransportStats {
    ///Construct a new `RtcTransportStats`.
    ///
    ///*This API requires the following crate features to be activated: `RtcTransportStats`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `id` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcTransportStats`*

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
    ///*This API requires the following crate features to be activated: `RtcTransportStats`*

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
    ///*This API requires the following crate features to be activated: `RtcStatsType`, `RtcTransportStats`*

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

    ///Change the `bytesReceived` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcTransportStats`*

    pub fn bytes_received(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("bytesReceived"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `bytesSent` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcTransportStats`*

    pub fn bytes_sent(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("bytesSent"),
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
