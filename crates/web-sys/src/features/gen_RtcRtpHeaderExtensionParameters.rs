use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCRtpHeaderExtensionParameters ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcRtpHeaderExtensionParameters` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*
    pub type RtcRtpHeaderExtensionParameters;

}

impl RtcRtpHeaderExtensionParameters {
    ///Construct a new `RtcRtpHeaderExtensionParameters`.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `encrypted` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*

    pub fn encrypted(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("encrypted"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `id` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*

    pub fn id(&mut self, val: u16) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("id"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `uri` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*

    pub fn uri(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("uri"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
