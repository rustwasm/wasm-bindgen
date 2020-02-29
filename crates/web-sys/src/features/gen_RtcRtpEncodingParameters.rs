use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCRtpEncodingParameters ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcRtpEncodingParameters` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*
    pub type RtcRtpEncodingParameters;

}

impl RtcRtpEncodingParameters {
    ///Construct a new `RtcRtpEncodingParameters`.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `active` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*

    pub fn active(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("active"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "RtcDegradationPreference")]
    ///Change the `degradationPreference` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcDegradationPreference`, `RtcRtpEncodingParameters`*

    pub fn degradation_preference(&mut self, val: RtcDegradationPreference) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("degradationPreference"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "RtcFecParameters")]
    ///Change the `fec` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcFecParameters`, `RtcRtpEncodingParameters`*

    pub fn fec(&mut self, val: &RtcFecParameters) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("fec"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `maxBitrate` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*

    pub fn max_bitrate(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("maxBitrate"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "RtcPriorityType")]
    ///Change the `priority` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcPriorityType`, `RtcRtpEncodingParameters`*

    pub fn priority(&mut self, val: RtcPriorityType) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("priority"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `rid` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*

    pub fn rid(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("rid"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "RtcRtxParameters")]
    ///Change the `rtx` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`, `RtcRtxParameters`*

    pub fn rtx(&mut self, val: &RtcRtxParameters) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("rtx"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `scaleResolutionDownBy` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*

    pub fn scale_resolution_down_by(&mut self, val: f32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("scaleResolutionDownBy"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `ssrc` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*

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
