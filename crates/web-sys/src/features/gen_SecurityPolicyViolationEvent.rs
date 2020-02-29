use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = SecurityPolicyViolationEvent , typescript_name = SecurityPolicyViolationEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SecurityPolicyViolationEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent)
    ///
    ///*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*
    pub type SecurityPolicyViolationEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SecurityPolicyViolationEvent" , js_name = documentURI ) ]
    ///Getter for the `documentURI` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/documentURI)
    ///
    ///*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*
    pub fn document_uri(this: &SecurityPolicyViolationEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SecurityPolicyViolationEvent" , js_name = referrer ) ]
    ///Getter for the `referrer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/referrer)
    ///
    ///*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*
    pub fn referrer(this: &SecurityPolicyViolationEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SecurityPolicyViolationEvent" , js_name = blockedURI ) ]
    ///Getter for the `blockedURI` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/blockedURI)
    ///
    ///*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*
    pub fn blocked_uri(this: &SecurityPolicyViolationEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SecurityPolicyViolationEvent" , js_name = violatedDirective ) ]
    ///Getter for the `violatedDirective` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/violatedDirective)
    ///
    ///*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*
    pub fn violated_directive(this: &SecurityPolicyViolationEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SecurityPolicyViolationEvent" , js_name = effectiveDirective ) ]
    ///Getter for the `effectiveDirective` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/effectiveDirective)
    ///
    ///*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*
    pub fn effective_directive(this: &SecurityPolicyViolationEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SecurityPolicyViolationEvent" , js_name = originalPolicy ) ]
    ///Getter for the `originalPolicy` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/originalPolicy)
    ///
    ///*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*
    pub fn original_policy(this: &SecurityPolicyViolationEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SecurityPolicyViolationEvent" , js_name = sourceFile ) ]
    ///Getter for the `sourceFile` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/sourceFile)
    ///
    ///*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*
    pub fn source_file(this: &SecurityPolicyViolationEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SecurityPolicyViolationEvent" , js_name = sample ) ]
    ///Getter for the `sample` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/sample)
    ///
    ///*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*
    pub fn sample(this: &SecurityPolicyViolationEvent) -> String;

    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SecurityPolicyViolationEvent" , js_name = disposition ) ]
    ///Getter for the `disposition` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/disposition)
    ///
    ///*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`, `SecurityPolicyViolationEventDisposition`*
    pub fn disposition(
        this: &SecurityPolicyViolationEvent,
    ) -> SecurityPolicyViolationEventDisposition;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SecurityPolicyViolationEvent" , js_name = statusCode ) ]
    ///Getter for the `statusCode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/statusCode)
    ///
    ///*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*
    pub fn status_code(this: &SecurityPolicyViolationEvent) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SecurityPolicyViolationEvent" , js_name = lineNumber ) ]
    ///Getter for the `lineNumber` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/lineNumber)
    ///
    ///*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*
    pub fn line_number(this: &SecurityPolicyViolationEvent) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SecurityPolicyViolationEvent" , js_name = columnNumber ) ]
    ///Getter for the `columnNumber` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/columnNumber)
    ///
    ///*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*
    pub fn column_number(this: &SecurityPolicyViolationEvent) -> i32;

    #[wasm_bindgen(catch, constructor, js_class = "SecurityPolicyViolationEvent")]
    ///The `new SecurityPolicyViolationEvent(..)` constructor, creating a new instance of `SecurityPolicyViolationEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/SecurityPolicyViolationEvent)
    ///
    ///*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*
    pub fn new(type_: &str) -> Result<SecurityPolicyViolationEvent, JsValue>;

    #[cfg(feature = "SecurityPolicyViolationEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "SecurityPolicyViolationEvent")]
    ///The `new SecurityPolicyViolationEvent(..)` constructor, creating a new instance of `SecurityPolicyViolationEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/SecurityPolicyViolationEvent)
    ///
    ///*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`, `SecurityPolicyViolationEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &SecurityPolicyViolationEventInit,
    ) -> Result<SecurityPolicyViolationEvent, JsValue>;

}
