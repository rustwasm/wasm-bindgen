#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(no_export)]
#[doc = "The `SecurityPolicyViolationEventDisposition` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventDisposition`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityPolicyViolationEventDisposition {
    Enforce = "enforce",
    Report = "report",
}
