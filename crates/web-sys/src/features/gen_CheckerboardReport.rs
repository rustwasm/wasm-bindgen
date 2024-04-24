#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CheckerboardReport)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CheckerboardReport` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    pub type CheckerboardReport;
    #[wasm_bindgen(method, getter = "log")]
    fn log_shim(this: &CheckerboardReport) -> String;
    #[wasm_bindgen(method, setter = "log")]
    fn set_log_shim(this: &CheckerboardReport, val: &str);
    #[cfg(feature = "CheckerboardReason")]
    #[wasm_bindgen(method, getter = "reason")]
    fn reason_shim(this: &CheckerboardReport) -> CheckerboardReason;
    #[cfg(feature = "CheckerboardReason")]
    #[wasm_bindgen(method, setter = "reason")]
    fn set_reason_shim(this: &CheckerboardReport, val: CheckerboardReason);
    #[wasm_bindgen(method, getter = "severity")]
    fn severity_shim(this: &CheckerboardReport) -> u32;
    #[wasm_bindgen(method, setter = "severity")]
    fn set_severity_shim(this: &CheckerboardReport, val: u32);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &CheckerboardReport) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &CheckerboardReport, val: f64);
}
#[doc = "The trait to access properties on the `CheckerboardReport` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
pub trait CheckerboardReportGetters {
    #[doc = "Get the `log` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    fn log(&self) -> String;
    #[cfg(feature = "CheckerboardReason")]
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReason`, `CheckerboardReport`*"]
    fn reason(&self) -> CheckerboardReason;
    #[doc = "Get the `severity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    fn severity(&self) -> u32;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    fn timestamp(&self) -> f64;
}
impl CheckerboardReportGetters for CheckerboardReport {
    fn log(&self) -> String {
        self.log_shim()
    }
    #[cfg(feature = "CheckerboardReason")]
    fn reason(&self) -> CheckerboardReason {
        self.reason_shim()
    }
    fn severity(&self) -> u32 {
        self.severity_shim()
    }
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
}
impl CheckerboardReport {
    #[doc = "Construct a new `CheckerboardReport`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `log` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    pub fn log(&mut self, val: &str) -> &mut Self {
        self.set_log_shim(val);
        self
    }
    #[cfg(feature = "CheckerboardReason")]
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReason`, `CheckerboardReport`*"]
    pub fn reason(&mut self, val: CheckerboardReason) -> &mut Self {
        self.set_reason_shim(val);
        self
    }
    #[doc = "Change the `severity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    pub fn severity(&mut self, val: u32) -> &mut Self {
        self.set_severity_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
}
impl Default for CheckerboardReport {
    fn default() -> Self {
        Self::new()
    }
}
