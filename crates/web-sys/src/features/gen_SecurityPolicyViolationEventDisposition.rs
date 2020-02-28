use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `SecurityPolicyViolationEventDisposition` enum.\n\n*This API requires the following crate features to be activated: `SecurityPolicyViolationEventDisposition`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SecurityPolicyViolationEventDisposition {
    Enforce = "enforce",
    Report = "report",
}
