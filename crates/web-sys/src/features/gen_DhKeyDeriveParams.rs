use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DhKeyDeriveParams ) ]
    #[doc = "The `DhKeyDeriveParams` dictionary.\n\n*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"]
    pub type DhKeyDeriveParams;
}
impl DhKeyDeriveParams {
    #[cfg(feature = "CryptoKey")]
    #[doc = "Construct a new `DhKeyDeriveParams`.\n\n*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"]
    pub fn new(name: &str, public: &CryptoKey) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.public(public);
        ret
    }
    #[doc = "Change the `name` field of this object.\n\n*This API requires the following crate features to be activated: `DhKeyDeriveParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("name"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "CryptoKey")]
    #[doc = "Change the `public` field of this object.\n\n*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"]
    pub fn public(&mut self, val: &CryptoKey) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("public"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
