use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCIdentityProvider ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcIdentityProvider` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RtcIdentityProvider`*
    pub type RtcIdentityProvider;

}

impl RtcIdentityProvider {
    ///Construct a new `RtcIdentityProvider`.
    ///
    ///*This API requires the following crate features to be activated: `RtcIdentityProvider`*

    pub fn new(
        generate_assertion: &::js_sys::Function,
        validate_assertion: &::js_sys::Function,
    ) -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret.generate_assertion(generate_assertion);

        ret.validate_assertion(validate_assertion);

        ret
    }

    ///Change the `generateAssertion` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIdentityProvider`*

    pub fn generate_assertion(&mut self, val: &::js_sys::Function) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("generateAssertion"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `validateAssertion` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIdentityProvider`*

    pub fn validate_assertion(&mut self, val: &::js_sys::Function) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("validateAssertion"),
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
