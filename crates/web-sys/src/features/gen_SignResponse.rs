use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SignResponse ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SignResponse` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `SignResponse`*
    pub type SignResponse;

}

impl SignResponse {
    ///Construct a new `SignResponse`.
    ///
    ///*This API requires the following crate features to be activated: `SignResponse`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `clientData` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SignResponse`*

    pub fn client_data(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("clientData"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `errorCode` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SignResponse`*

    pub fn error_code(&mut self, val: Option<u16>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("errorCode"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `errorMessage` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SignResponse`*

    pub fn error_message(&mut self, val: Option<&str>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("errorMessage"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `keyHandle` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SignResponse`*

    pub fn key_handle(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("keyHandle"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `signatureData` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `SignResponse`*

    pub fn signature_data(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("signatureData"),
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
