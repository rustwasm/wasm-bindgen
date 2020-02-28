use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = RTCIdentityValidationResult ) ]
    #[doc = "The `RtcIdentityValidationResult` dictionary.\n\n*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
    pub type RtcIdentityValidationResult;
}
impl RtcIdentityValidationResult {
    #[doc = "Construct a new `RtcIdentityValidationResult`.\n\n*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
    pub fn new(contents: &str, identity: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.contents(contents);
        ret.identity(identity);
        ret
    }
    #[doc = "Change the `contents` field of this object.\n\n*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
    pub fn contents(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("contents"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `identity` field of this object.\n\n*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
    pub fn identity(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("identity"),
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
