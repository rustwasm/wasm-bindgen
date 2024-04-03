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
    #[wasm_bindgen(method, setter = "avgLong")]
    fn avg_long_shim(this: &RcwnPerfStats, val: u32);
    #[wasm_bindgen(method, setter = "avgShort")]
    fn avg_short_shim(this: &RcwnPerfStats, val: u32);
    #[wasm_bindgen(method, setter = "stddevLong")]
    fn stddev_long_shim(this: &RcwnPerfStats, val: u32);
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
        self.avg_long_shim(val);
        self
    }
    #[doc = "Change the `avgShort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    pub fn avg_short(&mut self, val: u32) -> &mut Self {
        self.avg_short_shim(val);
        self
    }
    #[doc = "Change the `stddevLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    pub fn stddev_long(&mut self, val: u32) -> &mut Self {
        self.stddev_long_shim(val);
        self
    }
}
impl Default for RcwnPerfStats {
    fn default() -> Self {
        Self::new()
    }
}
