use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCRtpSynchronizationSource ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcRtpSynchronizationSource` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*
    pub type RtcRtpSynchronizationSource;

}

impl RtcRtpSynchronizationSource {
    ///Construct a new `RtcRtpSynchronizationSource`.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*

    pub fn new(source: u32, timestamp: f64) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.source(source);

        ret.timestamp(timestamp);

        ret
    }

    ///Change the `audioLevel` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*

    pub fn audio_level(&mut self, val: f64) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("audioLevel"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `source` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*

    pub fn source(&mut self, val: u32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("source"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `timestamp` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*

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

    ///Change the `voiceActivityFlag` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*

    pub fn voice_activity_flag(&mut self, val: Option<bool>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("voiceActivityFlag"),
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
