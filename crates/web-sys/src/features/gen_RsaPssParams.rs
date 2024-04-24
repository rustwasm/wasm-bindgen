#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RsaPssParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RsaPssParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    pub type RsaPssParams;
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &RsaPssParams) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &RsaPssParams, val: &str);
    #[wasm_bindgen(method, getter = "saltLength")]
    fn salt_length_shim(this: &RsaPssParams) -> u32;
    #[wasm_bindgen(method, setter = "saltLength")]
    fn set_salt_length_shim(this: &RsaPssParams, val: u32);
}
#[doc = "The trait to access properties on the `RsaPssParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
pub trait RsaPssParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    fn name(&self) -> String;
    #[doc = "Get the `saltLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    fn salt_length(&self) -> u32;
}
impl RsaPssParamsGetters for RsaPssParams {
    fn name(&self) -> String {
        self.name_shim()
    }
    fn salt_length(&self) -> u32 {
        self.salt_length_shim()
    }
}
impl RsaPssParams {
    #[doc = "Construct a new `RsaPssParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    pub fn new(name: &str, salt_length: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::name(&mut ret, name);
        Self::salt_length(&mut ret, salt_length);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `saltLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    pub fn salt_length(&mut self, val: u32) -> &mut Self {
        self.set_salt_length_shim(val);
        self
    }
}
