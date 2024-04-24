#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RcwnPerfStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RcwnPerfStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    pub type RcwnPerfStats;
    #[wasm_bindgen(method, getter = "avgLong")]
    fn avg_long_shim(this: &RcwnPerfStats) -> u32;
    #[wasm_bindgen(method, setter = "avgLong")]
    fn set_avg_long_shim(this: &RcwnPerfStats, val: u32);
    #[wasm_bindgen(method, getter = "avgShort")]
    fn avg_short_shim(this: &RcwnPerfStats) -> u32;
    #[wasm_bindgen(method, setter = "avgShort")]
    fn set_avg_short_shim(this: &RcwnPerfStats, val: u32);
    #[wasm_bindgen(method, getter = "stddevLong")]
    fn stddev_long_shim(this: &RcwnPerfStats) -> u32;
    #[wasm_bindgen(method, setter = "stddevLong")]
    fn set_stddev_long_shim(this: &RcwnPerfStats, val: u32);
}
#[doc = "The trait to access properties on the `RcwnPerfStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
pub trait RcwnPerfStatsGetters {
    #[doc = "Get the `avgLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    fn avg_long(&self) -> u32;
    #[doc = "Get the `avgShort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    fn avg_short(&self) -> u32;
    #[doc = "Get the `stddevLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    fn stddev_long(&self) -> u32;
}
impl RcwnPerfStatsGetters for RcwnPerfStats {
    fn avg_long(&self) -> u32 {
        self.avg_long_shim()
    }
    fn avg_short(&self) -> u32 {
        self.avg_short_shim()
    }
    fn stddev_long(&self) -> u32 {
        self.stddev_long_shim()
    }
}
impl RcwnPerfStats {
    #[doc = "Construct a new `RcwnPerfStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `avgLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    pub fn avg_long(&mut self, val: u32) -> &mut Self {
        self.set_avg_long_shim(val);
        self
    }
    #[doc = "Change the `avgShort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    pub fn avg_short(&mut self, val: u32) -> &mut Self {
        self.set_avg_short_shim(val);
        self
    }
    #[doc = "Change the `stddevLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    pub fn stddev_long(&mut self, val: u32) -> &mut Self {
        self.set_stddev_long_shim(val);
        self
    }
}
impl Default for RcwnPerfStats {
    fn default() -> Self {
        Self::new()
    }
}
