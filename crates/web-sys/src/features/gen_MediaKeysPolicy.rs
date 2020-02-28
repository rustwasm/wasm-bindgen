use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaKeysPolicy ) ]
    #[doc = "The `MediaKeysPolicy` dictionary.\n\n*This API requires the following crate features to be activated: `MediaKeysPolicy`*"]
    pub type MediaKeysPolicy;
}
impl MediaKeysPolicy {
    #[doc = "Construct a new `MediaKeysPolicy`.\n\n*This API requires the following crate features to be activated: `MediaKeysPolicy`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `minHdcpVersion` field of this object.\n\n*This API requires the following crate features to be activated: `MediaKeysPolicy`*"]
    pub fn min_hdcp_version(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("minHdcpVersion"),
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
