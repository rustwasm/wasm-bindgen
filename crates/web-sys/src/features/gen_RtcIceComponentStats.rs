use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCIceComponentStats ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcIceComponentStats` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceComponentStats`*
    pub type RtcIceComponentStats;

}

impl RtcIceComponentStats {
    ///Construct a new `RtcIceComponentStats`.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceComponentStats`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `id` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceComponentStats`*

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
    ///*This API requires the following crate features to be activated: `RtcIceComponentStats`*

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
    ///*This API requires the following crate features to be activated: `RtcIceComponentStats`, `RtcStatsType`*

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

    ///Change the `activeConnection` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceComponentStats`*

    pub fn active_connection(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("activeConnection"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `bytesReceived` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceComponentStats`*

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
    ///*This API requires the following crate features to be activated: `RtcIceComponentStats`*

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

    ///Change the `component` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceComponentStats`*

    pub fn component(&mut self, val: i32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("component"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `transportId` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIceComponentStats`*

    pub fn transport_id(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("transportId"),
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
