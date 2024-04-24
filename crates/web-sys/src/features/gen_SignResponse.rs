#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SignResponse)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SignResponse` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    pub type SignResponse;
    #[wasm_bindgen(method, getter = "clientData")]
    fn client_data_shim(this: &SignResponse) -> &str;
    #[wasm_bindgen(method, setter = "clientData")]
    fn set_client_data_shim(this: &SignResponse, val: &str);
    #[wasm_bindgen(method, getter = "errorCode")]
    fn error_code_shim(this: &SignResponse) -> Option<u16>;
    #[wasm_bindgen(method, setter = "errorCode")]
    fn set_error_code_shim(this: &SignResponse, val: Option<u16>);
    #[wasm_bindgen(method, getter = "errorMessage")]
    fn error_message_shim(this: &SignResponse) -> Option<&str>;
    #[wasm_bindgen(method, setter = "errorMessage")]
    fn set_error_message_shim(this: &SignResponse, val: Option<&str>);
    #[wasm_bindgen(method, getter = "keyHandle")]
    fn key_handle_shim(this: &SignResponse) -> &str;
    #[wasm_bindgen(method, setter = "keyHandle")]
    fn set_key_handle_shim(this: &SignResponse, val: &str);
    #[wasm_bindgen(method, getter = "signatureData")]
    fn signature_data_shim(this: &SignResponse) -> &str;
    #[wasm_bindgen(method, setter = "signatureData")]
    fn set_signature_data_shim(this: &SignResponse, val: &str);
}
#[doc = "The trait to access properties on the `SignResponse` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
pub trait SignResponseGetters {
    #[doc = "Get the `clientData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    fn client_data(&self) -> &str;
    #[doc = "Get the `errorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    fn error_code(&self) -> Option<u16>;
    #[doc = "Get the `errorMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    fn error_message(&self) -> Option<&str>;
    #[doc = "Get the `keyHandle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    fn key_handle(&self) -> &str;
    #[doc = "Get the `signatureData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    fn signature_data(&self) -> &str;
}
impl SignResponseGetters for SignResponse {
    fn client_data(&self) -> &str {
        self.client_data_shim()
    }
    fn error_code(&self) -> Option<u16> {
        self.error_code_shim()
    }
    fn error_message(&self) -> Option<&str> {
        self.error_message_shim()
    }
    fn key_handle(&self) -> &str {
        self.key_handle_shim()
    }
    fn signature_data(&self) -> &str {
        self.signature_data_shim()
    }
}
impl SignResponse {
    #[doc = "Construct a new `SignResponse`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `clientData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    pub fn client_data(&mut self, val: &str) -> &mut Self {
        self.set_client_data_shim(val);
        self
    }
    #[doc = "Change the `errorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    pub fn error_code(&mut self, val: Option<u16>) -> &mut Self {
        self.set_error_code_shim(val);
        self
    }
    #[doc = "Change the `errorMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    pub fn error_message(&mut self, val: Option<&str>) -> &mut Self {
        self.set_error_message_shim(val);
        self
    }
    #[doc = "Change the `keyHandle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    pub fn key_handle(&mut self, val: &str) -> &mut Self {
        self.set_key_handle_shim(val);
        self
    }
    #[doc = "Change the `signatureData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    pub fn signature_data(&mut self, val: &str) -> &mut Self {
        self.set_signature_data_shim(val);
        self
    }
}
impl Default for SignResponse {
    fn default() -> Self {
        Self::new()
    }
}
