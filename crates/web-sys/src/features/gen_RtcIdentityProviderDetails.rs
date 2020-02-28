use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCIdentityProviderDetails ) ]
    #[doc = "The `RtcIdentityProviderDetails` dictionary.\n\n*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
    pub type RtcIdentityProviderDetails;
}
impl RtcIdentityProviderDetails {
    #[doc = "Construct a new `RtcIdentityProviderDetails`.\n\n*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
    pub fn new(domain: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.domain(domain);
        ret
    }
    #[doc = "Change the `domain` field of this object.\n\n*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
    pub fn domain(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("domain"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `protocol` field of this object.\n\n*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
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
}
