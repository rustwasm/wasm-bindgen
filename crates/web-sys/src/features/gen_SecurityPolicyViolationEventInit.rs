#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SecurityPolicyViolationEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SecurityPolicyViolationEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub type SecurityPolicyViolationEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &SecurityPolicyViolationEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &SecurityPolicyViolationEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &SecurityPolicyViolationEventInit, val: bool);
    #[wasm_bindgen(method, setter = "blockedURI")]
    fn blocked_uri_shim(this: &SecurityPolicyViolationEventInit, val: &str);
    #[wasm_bindgen(method, setter = "columnNumber")]
    fn column_number_shim(this: &SecurityPolicyViolationEventInit, val: i32);
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[wasm_bindgen(method, setter = "disposition")]
    fn disposition_shim(
        this: &SecurityPolicyViolationEventInit,
        val: SecurityPolicyViolationEventDisposition,
    );
    #[wasm_bindgen(method, setter = "documentURI")]
    fn document_uri_shim(this: &SecurityPolicyViolationEventInit, val: &str);
    #[wasm_bindgen(method, setter = "effectiveDirective")]
    fn effective_directive_shim(this: &SecurityPolicyViolationEventInit, val: &str);
    #[wasm_bindgen(method, setter = "lineNumber")]
    fn line_number_shim(this: &SecurityPolicyViolationEventInit, val: i32);
    #[wasm_bindgen(method, setter = "originalPolicy")]
    fn original_policy_shim(this: &SecurityPolicyViolationEventInit, val: &str);
    #[wasm_bindgen(method, setter = "referrer")]
    fn referrer_shim(this: &SecurityPolicyViolationEventInit, val: &str);
    #[wasm_bindgen(method, setter = "sample")]
    fn sample_shim(this: &SecurityPolicyViolationEventInit, val: &str);
    #[wasm_bindgen(method, setter = "sourceFile")]
    fn source_file_shim(this: &SecurityPolicyViolationEventInit, val: &str);
    #[wasm_bindgen(method, setter = "statusCode")]
    fn status_code_shim(this: &SecurityPolicyViolationEventInit, val: u16);
    #[wasm_bindgen(method, setter = "violatedDirective")]
    fn violated_directive_shim(this: &SecurityPolicyViolationEventInit, val: &str);
}
impl SecurityPolicyViolationEventInit {
    #[doc = "Construct a new `SecurityPolicyViolationEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `blockedURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn blocked_uri(&mut self, val: &str) -> &mut Self {
        self.blocked_uri_shim(val);
        self
    }
    #[doc = "Change the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn column_number(&mut self, val: i32) -> &mut Self {
        self.column_number_shim(val);
        self
    }
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[doc = "Change the `disposition` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventDisposition`, `SecurityPolicyViolationEventInit`*"]
    pub fn disposition(&mut self, val: SecurityPolicyViolationEventDisposition) -> &mut Self {
        self.disposition_shim(val);
        self
    }
    #[doc = "Change the `documentURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn document_uri(&mut self, val: &str) -> &mut Self {
        self.document_uri_shim(val);
        self
    }
    #[doc = "Change the `effectiveDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn effective_directive(&mut self, val: &str) -> &mut Self {
        self.effective_directive_shim(val);
        self
    }
    #[doc = "Change the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn line_number(&mut self, val: i32) -> &mut Self {
        self.line_number_shim(val);
        self
    }
    #[doc = "Change the `originalPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn original_policy(&mut self, val: &str) -> &mut Self {
        self.original_policy_shim(val);
        self
    }
    #[doc = "Change the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn referrer(&mut self, val: &str) -> &mut Self {
        self.referrer_shim(val);
        self
    }
    #[doc = "Change the `sample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn sample(&mut self, val: &str) -> &mut Self {
        self.sample_shim(val);
        self
    }
    #[doc = "Change the `sourceFile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn source_file(&mut self, val: &str) -> &mut Self {
        self.source_file_shim(val);
        self
    }
    #[doc = "Change the `statusCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn status_code(&mut self, val: u16) -> &mut Self {
        self.status_code_shim(val);
        self
    }
    #[doc = "Change the `violatedDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn violated_directive(&mut self, val: &str) -> &mut Self {
        self.violated_directive_shim(val);
        self
    }
}
impl Default for SecurityPolicyViolationEventInit {
    fn default() -> Self {
        Self::new()
    }
}
