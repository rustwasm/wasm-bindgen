#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RegisteredKey)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RegisteredKey` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
    pub type RegisteredKey;
    #[wasm_bindgen(method, setter = "appId")]
    fn app_id_shim(this: &RegisteredKey, val: Option<&str>);
    #[wasm_bindgen(method, setter = "keyHandle")]
    fn key_handle_shim(this: &RegisteredKey, val: &str);
    #[wasm_bindgen(method, setter = "transports")]
    fn transports_shim(this: &RegisteredKey, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "version")]
    fn version_shim(this: &RegisteredKey, val: &str);
}
impl RegisteredKey {
    #[doc = "Construct a new `RegisteredKey`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `appId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
    pub fn app_id(&mut self, val: Option<&str>) -> &mut Self {
        self.app_id_shim(val);
        self
    }
    #[doc = "Change the `keyHandle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
    pub fn key_handle(&mut self, val: &str) -> &mut Self {
        self.key_handle_shim(val);
        self
    }
    #[doc = "Change the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
    pub fn transports(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        self.transports_shim(val.unwrap_or(&::wasm_bindgen::JsValue::NULL));
        self
    }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
    pub fn version(&mut self, val: &str) -> &mut Self {
        self.version_shim(val);
        self
    }
}
impl Default for RegisteredKey {
    fn default() -> Self {
        Self::new()
    }
}
