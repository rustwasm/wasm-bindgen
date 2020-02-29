use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCRtpParameters ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcRtpParameters` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpParameters`*
    pub type RtcRtpParameters;

}

impl RtcRtpParameters {
    ///Construct a new `RtcRtpParameters`.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpParameters`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `codecs` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpParameters`*

    pub fn codecs(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("codecs"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `encodings` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpParameters`*

    pub fn encodings(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("encodings"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `headerExtensions` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpParameters`*

    pub fn header_extensions(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("headerExtensions"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "RtcRtcpParameters")]
    ///Change the `rtcp` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtcpParameters`, `RtcRtpParameters`*

    pub fn rtcp(&mut self, val: &RtcRtcpParameters) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("rtcp"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
