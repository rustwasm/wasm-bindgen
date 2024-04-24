#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RegisterResponse)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RegisterResponse` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    pub type RegisterResponse;
    #[wasm_bindgen(method, getter = "clientData")]
    fn client_data_shim(this: &RegisterResponse) -> &str;
    #[wasm_bindgen(method, setter = "clientData")]
    fn set_client_data_shim(this: &RegisterResponse, val: &str);
    #[wasm_bindgen(method, getter = "errorCode")]
    fn error_code_shim(this: &RegisterResponse) -> Option<u16>;
    #[wasm_bindgen(method, setter = "errorCode")]
    fn set_error_code_shim(this: &RegisterResponse, val: Option<u16>);
    #[wasm_bindgen(method, getter = "errorMessage")]
    fn error_message_shim(this: &RegisterResponse) -> Option<&str>;
    #[wasm_bindgen(method, setter = "errorMessage")]
    fn set_error_message_shim(this: &RegisterResponse, val: Option<&str>);
    #[wasm_bindgen(method, getter = "registrationData")]
    fn registration_data_shim(this: &RegisterResponse) -> &str;
    #[wasm_bindgen(method, setter = "registrationData")]
    fn set_registration_data_shim(this: &RegisterResponse, val: &str);
    #[wasm_bindgen(method, getter = "version")]
    fn version_shim(this: &RegisterResponse) -> &str;
    #[wasm_bindgen(method, setter = "version")]
    fn set_version_shim(this: &RegisterResponse, val: &str);
}
#[doc = "The trait to access properties on the `RegisterResponse` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
pub trait RegisterResponseGetters {
    #[doc = "Get the `clientData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    fn client_data(&self) -> &str;
    #[doc = "Get the `errorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    fn error_code(&self) -> Option<u16>;
    #[doc = "Get the `errorMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    fn error_message(&self) -> Option<&str>;
    #[doc = "Get the `registrationData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    fn registration_data(&self) -> &str;
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    fn version(&self) -> &str;
}
impl RegisterResponseGetters for RegisterResponse {
    fn client_data(&self) -> &str {
        self.client_data_shim()
    }
    fn error_code(&self) -> Option<u16> {
        self.error_code_shim()
    }
    fn error_message(&self) -> Option<&str> {
        self.error_message_shim()
    }
    fn registration_data(&self) -> &str {
        self.registration_data_shim()
    }
    fn version(&self) -> &str {
        self.version_shim()
    }
}
impl RegisterResponse {
    #[doc = "Construct a new `RegisterResponse`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `clientData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    pub fn client_data(&mut self, val: &str) -> &mut Self {
        self.set_client_data_shim(val);
        self
    }
    #[doc = "Change the `errorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    pub fn error_code(&mut self, val: Option<u16>) -> &mut Self {
        self.set_error_code_shim(val);
        self
    }
    #[doc = "Change the `errorMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    pub fn error_message(&mut self, val: Option<&str>) -> &mut Self {
        self.set_error_message_shim(val);
        self
    }
    #[doc = "Change the `registrationData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    pub fn registration_data(&mut self, val: &str) -> &mut Self {
        self.set_registration_data_shim(val);
        self
    }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    pub fn version(&mut self, val: &str) -> &mut Self {
        self.set_version_shim(val);
        self
    }
}
impl Default for RegisterResponse {
    fn default() -> Self {
        Self::new()
    }
}
