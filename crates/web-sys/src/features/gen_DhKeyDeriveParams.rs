#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DhKeyDeriveParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DhKeyDeriveParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyDeriveParams`*"]
    pub type DhKeyDeriveParams;
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &DhKeyDeriveParams) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &DhKeyDeriveParams, val: &str);
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(method, getter = "public")]
    fn public_shim(this: &DhKeyDeriveParams) -> CryptoKey;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(method, setter = "public")]
    fn set_public_shim(this: &DhKeyDeriveParams, val: &CryptoKey);
}
#[doc = "The trait to access properties on the `DhKeyDeriveParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DhKeyDeriveParams`*"]
pub trait DhKeyDeriveParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyDeriveParams`*"]
    fn name(&self) -> String;
    #[cfg(feature = "CryptoKey")]
    #[doc = "Get the `public` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"]
    fn public(&self) -> CryptoKey;
}
impl DhKeyDeriveParamsGetters for DhKeyDeriveParams {
    fn name(&self) -> String {
        self.name_shim()
    }
    #[cfg(feature = "CryptoKey")]
    fn public(&self) -> CryptoKey {
        self.public_shim()
    }
}
impl DhKeyDeriveParams {
    #[cfg(feature = "CryptoKey")]
    #[doc = "Construct a new `DhKeyDeriveParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"]
    pub fn new(name: &str, public: &CryptoKey) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::name(&mut ret, name);
        Self::public(&mut ret, public);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyDeriveParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[cfg(feature = "CryptoKey")]
    #[doc = "Change the `public` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"]
    pub fn public(&mut self, val: &CryptoKey) -> &mut Self {
        self.set_public_shim(val);
        self
    }
}
