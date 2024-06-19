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
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &SecurityPolicyViolationEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &SecurityPolicyViolationEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &SecurityPolicyViolationEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &SecurityPolicyViolationEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &SecurityPolicyViolationEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &SecurityPolicyViolationEventInit, val: bool);
    #[doc = "Get the `blockedURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "blockedURI")]
    pub fn get_blocked_uri(this: &SecurityPolicyViolationEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "blockedURI")]
    fn set_blocked_uri(this: &SecurityPolicyViolationEventInit, val: &str);
    #[doc = "Get the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "columnNumber")]
    pub fn get_column_number(this: &SecurityPolicyViolationEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "columnNumber")]
    fn set_column_number(this: &SecurityPolicyViolationEventInit, val: i32);
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[doc = "Get the `disposition` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventDisposition`, `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "disposition")]
    pub fn get_disposition(
        this: &SecurityPolicyViolationEventInit,
    ) -> Option<SecurityPolicyViolationEventDisposition>;
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[wasm_bindgen(method, setter = "disposition")]
    fn set_disposition(
        this: &SecurityPolicyViolationEventInit,
        val: SecurityPolicyViolationEventDisposition,
    );
    #[doc = "Get the `documentURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "documentURI")]
    pub fn get_document_uri(this: &SecurityPolicyViolationEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "documentURI")]
    fn set_document_uri(this: &SecurityPolicyViolationEventInit, val: &str);
    #[doc = "Get the `effectiveDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "effectiveDirective")]
    pub fn get_effective_directive(this: &SecurityPolicyViolationEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "effectiveDirective")]
    fn set_effective_directive(this: &SecurityPolicyViolationEventInit, val: &str);
    #[doc = "Get the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "lineNumber")]
    pub fn get_line_number(this: &SecurityPolicyViolationEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "lineNumber")]
    fn set_line_number(this: &SecurityPolicyViolationEventInit, val: i32);
    #[doc = "Get the `originalPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "originalPolicy")]
    pub fn get_original_policy(this: &SecurityPolicyViolationEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "originalPolicy")]
    fn set_original_policy(this: &SecurityPolicyViolationEventInit, val: &str);
    #[doc = "Get the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "referrer")]
    pub fn get_referrer(this: &SecurityPolicyViolationEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "referrer")]
    fn set_referrer(this: &SecurityPolicyViolationEventInit, val: &str);
    #[doc = "Get the `sample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "sample")]
    pub fn get_sample(this: &SecurityPolicyViolationEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "sample")]
    fn set_sample(this: &SecurityPolicyViolationEventInit, val: &str);
    #[doc = "Get the `sourceFile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "sourceFile")]
    pub fn get_source_file(this: &SecurityPolicyViolationEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "sourceFile")]
    fn set_source_file(this: &SecurityPolicyViolationEventInit, val: &str);
    #[doc = "Get the `statusCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "statusCode")]
    pub fn get_status_code(this: &SecurityPolicyViolationEventInit) -> Option<u16>;
    #[wasm_bindgen(method, setter = "statusCode")]
    fn set_status_code(this: &SecurityPolicyViolationEventInit, val: u16);
    #[doc = "Get the `violatedDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    #[wasm_bindgen(method, getter = "violatedDirective")]
    pub fn get_violated_directive(this: &SecurityPolicyViolationEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "violatedDirective")]
    fn set_violated_directive(this: &SecurityPolicyViolationEventInit, val: &str);
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
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `blockedURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn blocked_uri(&mut self, val: &str) -> &mut Self {
        self.set_blocked_uri(val);
        self
    }
    #[doc = "Change the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn column_number(&mut self, val: i32) -> &mut Self {
        self.set_column_number(val);
        self
    }
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[doc = "Change the `disposition` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventDisposition`, `SecurityPolicyViolationEventInit`*"]
    pub fn disposition(&mut self, val: SecurityPolicyViolationEventDisposition) -> &mut Self {
        self.set_disposition(val);
        self
    }
    #[doc = "Change the `documentURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn document_uri(&mut self, val: &str) -> &mut Self {
        self.set_document_uri(val);
        self
    }
    #[doc = "Change the `effectiveDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn effective_directive(&mut self, val: &str) -> &mut Self {
        self.set_effective_directive(val);
        self
    }
    #[doc = "Change the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn line_number(&mut self, val: i32) -> &mut Self {
        self.set_line_number(val);
        self
    }
    #[doc = "Change the `originalPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn original_policy(&mut self, val: &str) -> &mut Self {
        self.set_original_policy(val);
        self
    }
    #[doc = "Change the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn referrer(&mut self, val: &str) -> &mut Self {
        self.set_referrer(val);
        self
    }
    #[doc = "Change the `sample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn sample(&mut self, val: &str) -> &mut Self {
        self.set_sample(val);
        self
    }
    #[doc = "Change the `sourceFile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn source_file(&mut self, val: &str) -> &mut Self {
        self.set_source_file(val);
        self
    }
    #[doc = "Change the `statusCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn status_code(&mut self, val: u16) -> &mut Self {
        self.set_status_code(val);
        self
    }
    #[doc = "Change the `violatedDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn violated_directive(&mut self, val: &str) -> &mut Self {
        self.set_violated_directive(val);
        self
    }
}
impl Default for SecurityPolicyViolationEventInit {
    fn default() -> Self {
        Self::new()
    }
}
