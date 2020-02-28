use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = SecurityPolicyViolationEvent , typescript_name = SecurityPolicyViolationEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SecurityPolicyViolationEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub type SecurityPolicyViolationEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = documentURI ) ]
    #[doc = "Getter for the `documentURI` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/documentURI)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn document_uri(this: &SecurityPolicyViolationEvent) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = referrer ) ]
    #[doc = "Getter for the `referrer` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/referrer)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn referrer(this: &SecurityPolicyViolationEvent) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = blockedURI ) ]
    #[doc = "Getter for the `blockedURI` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/blockedURI)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn blocked_uri(this: &SecurityPolicyViolationEvent) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = violatedDirective ) ]
    #[doc = "Getter for the `violatedDirective` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/violatedDirective)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn violated_directive(this: &SecurityPolicyViolationEvent) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = effectiveDirective ) ]
    #[doc = "Getter for the `effectiveDirective` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/effectiveDirective)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn effective_directive(this: &SecurityPolicyViolationEvent) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = originalPolicy ) ]
    #[doc = "Getter for the `originalPolicy` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/originalPolicy)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn original_policy(this: &SecurityPolicyViolationEvent) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = sourceFile ) ]
    #[doc = "Getter for the `sourceFile` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/sourceFile)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn source_file(this: &SecurityPolicyViolationEvent) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = sample ) ]
    #[doc = "Getter for the `sample` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/sample)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn sample(this: &SecurityPolicyViolationEvent) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = disposition ) ]
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[doc = "Getter for the `disposition` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/disposition)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`, `SecurityPolicyViolationEventDisposition`*"]
    pub fn disposition(
        this: &SecurityPolicyViolationEvent,
    ) -> SecurityPolicyViolationEventDisposition;
    # [ wasm_bindgen ( structural , method , getter , js_name = statusCode ) ]
    #[doc = "Getter for the `statusCode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/statusCode)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn status_code(this: &SecurityPolicyViolationEvent) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_name = lineNumber ) ]
    #[doc = "Getter for the `lineNumber` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/lineNumber)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn line_number(this: &SecurityPolicyViolationEvent) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_name = columnNumber ) ]
    #[doc = "Getter for the `columnNumber` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/columnNumber)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn column_number(this: &SecurityPolicyViolationEvent) -> i32;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new SecurityPolicyViolationEvent(..)` constructor, creating a new instance of `SecurityPolicyViolationEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/SecurityPolicyViolationEvent)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
    pub fn new(
        this: &SecurityPolicyViolationEvent,
        type_: &str,
    ) -> Result<SecurityPolicyViolationEvent, JsValue>;
    #[cfg(feature = "SecurityPolicyViolationEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new SecurityPolicyViolationEvent(..)` constructor, creating a new instance of `SecurityPolicyViolationEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/SecurityPolicyViolationEvent)\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`, `SecurityPolicyViolationEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &SecurityPolicyViolationEvent,
        type_: &str,
        event_init_dict: &SecurityPolicyViolationEventInit,
    ) -> Result<SecurityPolicyViolationEvent, JsValue>;
}
