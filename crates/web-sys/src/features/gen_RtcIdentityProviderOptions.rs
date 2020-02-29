use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCIdentityProviderOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `RtcIdentityProviderOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*
    pub type RtcIdentityProviderOptions;

}

impl RtcIdentityProviderOptions {
    ///Construct a new `RtcIdentityProviderOptions`.
    ///
    ///*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `peerIdentity` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*

    pub fn peer_identity(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("peerIdentity"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `protocol` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*

    pub fn protocol(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("protocol"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `usernameHint` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*

    pub fn username_hint(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("usernameHint"),
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
