#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EcdhKeyDeriveParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EcdhKeyDeriveParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdhKeyDeriveParams`*"]
    pub type EcdhKeyDeriveParams;
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &EcdhKeyDeriveParams) -> &str;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &EcdhKeyDeriveParams, val: &str);
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(method, getter = "public")]
    fn public_shim(this: &EcdhKeyDeriveParams) -> &CryptoKey;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(method, setter = "public")]
    fn set_public_shim(this: &EcdhKeyDeriveParams, val: &CryptoKey);
}
#[doc = "The trait to access properties on the `EcdhKeyDeriveParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `EcdhKeyDeriveParams`*"]
pub trait EcdhKeyDeriveParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdhKeyDeriveParams`*"]
    fn name(&self) -> &str;
    #[cfg(feature = "CryptoKey")]
    #[doc = "Get the `public` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `EcdhKeyDeriveParams`*"]
    fn public(&self) -> &CryptoKey;
}
impl EcdhKeyDeriveParamsGetters for EcdhKeyDeriveParams {
    fn name(&self) -> &str {
        self.name_shim()
    }
    #[cfg(feature = "CryptoKey")]
    fn public(&self) -> &CryptoKey {
        self.public_shim()
    }
}
impl EcdhKeyDeriveParams {
    #[cfg(feature = "CryptoKey")]
    #[doc = "Construct a new `EcdhKeyDeriveParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `EcdhKeyDeriveParams`*"]
    pub fn new(name: &str, public: &CryptoKey) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.public(public);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdhKeyDeriveParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[cfg(feature = "CryptoKey")]
    #[doc = "Change the `public` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `EcdhKeyDeriveParams`*"]
    pub fn public(&mut self, val: &CryptoKey) -> &mut Self {
        self.set_public_shim(val);
        self
    }
}
