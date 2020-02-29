use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `SecurityPolicyViolationEventDisposition` enum.
///
///*This API requires the following crate features to be activated: `SecurityPolicyViolationEventDisposition`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum SecurityPolicyViolationEventDisposition {
    Enforce = "enforce",
    Report = "report",
}
