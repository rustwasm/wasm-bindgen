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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &SecurityPolicyViolationEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &SecurityPolicyViolationEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &SecurityPolicyViolationEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &SecurityPolicyViolationEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &SecurityPolicyViolationEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &SecurityPolicyViolationEventInit, val: bool);
    #[wasm_bindgen(method, getter = "blockedURI")]
    fn blocked_uri_shim(this: &SecurityPolicyViolationEventInit) -> String;
    #[wasm_bindgen(method, setter = "blockedURI")]
    fn set_blocked_uri_shim(this: &SecurityPolicyViolationEventInit, val: &str);
    #[wasm_bindgen(method, getter = "columnNumber")]
    fn column_number_shim(this: &SecurityPolicyViolationEventInit) -> i32;
    #[wasm_bindgen(method, setter = "columnNumber")]
    fn set_column_number_shim(this: &SecurityPolicyViolationEventInit, val: i32);
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[wasm_bindgen(method, getter = "disposition")]
    fn disposition_shim(
        this: &SecurityPolicyViolationEventInit,
    ) -> SecurityPolicyViolationEventDisposition;
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[wasm_bindgen(method, setter = "disposition")]
    fn set_disposition_shim(
        this: &SecurityPolicyViolationEventInit,
        val: SecurityPolicyViolationEventDisposition,
    );
    #[wasm_bindgen(method, getter = "documentURI")]
    fn document_uri_shim(this: &SecurityPolicyViolationEventInit) -> String;
    #[wasm_bindgen(method, setter = "documentURI")]
    fn set_document_uri_shim(this: &SecurityPolicyViolationEventInit, val: &str);
    #[wasm_bindgen(method, getter = "effectiveDirective")]
    fn effective_directive_shim(this: &SecurityPolicyViolationEventInit) -> String;
    #[wasm_bindgen(method, setter = "effectiveDirective")]
    fn set_effective_directive_shim(this: &SecurityPolicyViolationEventInit, val: &str);
    #[wasm_bindgen(method, getter = "lineNumber")]
    fn line_number_shim(this: &SecurityPolicyViolationEventInit) -> i32;
    #[wasm_bindgen(method, setter = "lineNumber")]
    fn set_line_number_shim(this: &SecurityPolicyViolationEventInit, val: i32);
    #[wasm_bindgen(method, getter = "originalPolicy")]
    fn original_policy_shim(this: &SecurityPolicyViolationEventInit) -> String;
    #[wasm_bindgen(method, setter = "originalPolicy")]
    fn set_original_policy_shim(this: &SecurityPolicyViolationEventInit, val: &str);
    #[wasm_bindgen(method, getter = "referrer")]
    fn referrer_shim(this: &SecurityPolicyViolationEventInit) -> String;
    #[wasm_bindgen(method, setter = "referrer")]
    fn set_referrer_shim(this: &SecurityPolicyViolationEventInit, val: &str);
    #[wasm_bindgen(method, getter = "sample")]
    fn sample_shim(this: &SecurityPolicyViolationEventInit) -> String;
    #[wasm_bindgen(method, setter = "sample")]
    fn set_sample_shim(this: &SecurityPolicyViolationEventInit, val: &str);
    #[wasm_bindgen(method, getter = "sourceFile")]
    fn source_file_shim(this: &SecurityPolicyViolationEventInit) -> String;
    #[wasm_bindgen(method, setter = "sourceFile")]
    fn set_source_file_shim(this: &SecurityPolicyViolationEventInit, val: &str);
    #[wasm_bindgen(method, getter = "statusCode")]
    fn status_code_shim(this: &SecurityPolicyViolationEventInit) -> u16;
    #[wasm_bindgen(method, setter = "statusCode")]
    fn set_status_code_shim(this: &SecurityPolicyViolationEventInit, val: u16);
    #[wasm_bindgen(method, getter = "violatedDirective")]
    fn violated_directive_shim(this: &SecurityPolicyViolationEventInit) -> String;
    #[wasm_bindgen(method, setter = "violatedDirective")]
    fn set_violated_directive_shim(this: &SecurityPolicyViolationEventInit, val: &str);
}
#[doc = "The trait to access properties on the `SecurityPolicyViolationEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
pub trait SecurityPolicyViolationEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `blockedURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn blocked_uri(&self) -> String;
    #[doc = "Get the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn column_number(&self) -> i32;
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[doc = "Get the `disposition` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventDisposition`, `SecurityPolicyViolationEventInit`*"]
    fn disposition(&self) -> SecurityPolicyViolationEventDisposition;
    #[doc = "Get the `documentURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn document_uri(&self) -> String;
    #[doc = "Get the `effectiveDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn effective_directive(&self) -> String;
    #[doc = "Get the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn line_number(&self) -> i32;
    #[doc = "Get the `originalPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn original_policy(&self) -> String;
    #[doc = "Get the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn referrer(&self) -> String;
    #[doc = "Get the `sample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn sample(&self) -> String;
    #[doc = "Get the `sourceFile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn source_file(&self) -> String;
    #[doc = "Get the `statusCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn status_code(&self) -> u16;
    #[doc = "Get the `violatedDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn violated_directive(&self) -> String;
}
impl SecurityPolicyViolationEventInitGetters for SecurityPolicyViolationEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn blocked_uri(&self) -> String {
        self.blocked_uri_shim()
    }
    fn column_number(&self) -> i32 {
        self.column_number_shim()
    }
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    fn disposition(&self) -> SecurityPolicyViolationEventDisposition {
        self.disposition_shim()
    }
    fn document_uri(&self) -> String {
        self.document_uri_shim()
    }
    fn effective_directive(&self) -> String {
        self.effective_directive_shim()
    }
    fn line_number(&self) -> i32 {
        self.line_number_shim()
    }
    fn original_policy(&self) -> String {
        self.original_policy_shim()
    }
    fn referrer(&self) -> String {
        self.referrer_shim()
    }
    fn sample(&self) -> String {
        self.sample_shim()
    }
    fn source_file(&self) -> String {
        self.source_file_shim()
    }
    fn status_code(&self) -> u16 {
        self.status_code_shim()
    }
    fn violated_directive(&self) -> String {
        self.violated_directive_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `blockedURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn blocked_uri(&mut self, val: &str) -> &mut Self {
        self.set_blocked_uri_shim(val);
        self
    }
    #[doc = "Change the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn column_number(&mut self, val: i32) -> &mut Self {
        self.set_column_number_shim(val);
        self
    }
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[doc = "Change the `disposition` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventDisposition`, `SecurityPolicyViolationEventInit`*"]
    pub fn disposition(&mut self, val: SecurityPolicyViolationEventDisposition) -> &mut Self {
        self.set_disposition_shim(val);
        self
    }
    #[doc = "Change the `documentURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn document_uri(&mut self, val: &str) -> &mut Self {
        self.set_document_uri_shim(val);
        self
    }
    #[doc = "Change the `effectiveDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn effective_directive(&mut self, val: &str) -> &mut Self {
        self.set_effective_directive_shim(val);
        self
    }
    #[doc = "Change the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn line_number(&mut self, val: i32) -> &mut Self {
        self.set_line_number_shim(val);
        self
    }
    #[doc = "Change the `originalPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn original_policy(&mut self, val: &str) -> &mut Self {
        self.set_original_policy_shim(val);
        self
    }
    #[doc = "Change the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn referrer(&mut self, val: &str) -> &mut Self {
        self.set_referrer_shim(val);
        self
    }
    #[doc = "Change the `sample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn sample(&mut self, val: &str) -> &mut Self {
        self.set_sample_shim(val);
        self
    }
    #[doc = "Change the `sourceFile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn source_file(&mut self, val: &str) -> &mut Self {
        self.set_source_file_shim(val);
        self
    }
    #[doc = "Change the `statusCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn status_code(&mut self, val: u16) -> &mut Self {
        self.set_status_code_shim(val);
        self
    }
    #[doc = "Change the `violatedDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn violated_directive(&mut self, val: &str) -> &mut Self {
        self.set_violated_directive_shim(val);
        self
    }
}
impl Default for SecurityPolicyViolationEventInit {
    fn default() -> Self {
        Self::new()
    }
}
