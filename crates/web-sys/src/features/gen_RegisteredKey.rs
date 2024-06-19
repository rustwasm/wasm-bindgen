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
    #[doc = "Get the `appId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
    #[wasm_bindgen(method, getter = "appId")]
    pub fn get_app_id(this: &RegisteredKey) -> Option<String>;
    #[wasm_bindgen(method, setter = "appId")]
    fn set_app_id(this: &RegisteredKey, val: Option<&str>);
    #[doc = "Get the `keyHandle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
    #[wasm_bindgen(method, getter = "keyHandle")]
    pub fn get_key_handle(this: &RegisteredKey) -> Option<String>;
    #[wasm_bindgen(method, setter = "keyHandle")]
    fn set_key_handle(this: &RegisteredKey, val: &str);
    #[doc = "Get the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
    #[wasm_bindgen(method, getter = "transports")]
    pub fn get_transports(this: &RegisteredKey) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "transports")]
    fn set_transports(this: &RegisteredKey, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
    #[wasm_bindgen(method, getter = "version")]
    pub fn get_version(this: &RegisteredKey) -> Option<String>;
    #[wasm_bindgen(method, setter = "version")]
    fn set_version(this: &RegisteredKey, val: &str);
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
        self.set_app_id(val);
        self
    }
    #[doc = "Change the `keyHandle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
    pub fn key_handle(&mut self, val: &str) -> &mut Self {
        self.set_key_handle(val);
        self
    }
    #[doc = "Change the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
    pub fn transports(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        self.set_transports(val.unwrap_or(&::wasm_bindgen::JsValue::NULL));
        self
    }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
    pub fn version(&mut self, val: &str) -> &mut Self {
        self.set_version(val);
        self
    }
}
impl Default for RegisteredKey {
    fn default() -> Self {
        Self::new()
    }
}
