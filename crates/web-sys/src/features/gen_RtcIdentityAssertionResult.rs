use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCIdentityAssertionResult ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcIdentityAssertionResult` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`*
    pub type RtcIdentityAssertionResult;

}

impl RtcIdentityAssertionResult {
    #[cfg(feature = "RtcIdentityProviderDetails")]
    ///Construct a new `RtcIdentityAssertionResult`.
    ///
    ///*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`, `RtcIdentityProviderDetails`*

    pub fn new(assertion: &str, idp: &RtcIdentityProviderDetails) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.assertion(assertion);

        ret.idp(idp);

        ret
    }

    ///Change the `assertion` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`*

    pub fn assertion(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("assertion"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    #[cfg(feature = "RtcIdentityProviderDetails")]
    ///Change the `idp` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`, `RtcIdentityProviderDetails`*

    pub fn idp(&mut self, val: &RtcIdentityProviderDetails) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("idp"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
