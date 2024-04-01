#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RcwnStatus)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RcwnStatus` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    pub type RcwnStatus;
    #[wasm_bindgen(method, setter = "cacheNotSlowCount")]
    fn cache_not_slow_count_shim(this: &RcwnStatus, val: u32);
    #[wasm_bindgen(method, setter = "cacheSlowCount")]
    fn cache_slow_count_shim(this: &RcwnStatus, val: u32);
    #[wasm_bindgen(method, setter = "perfStats")]
    fn perf_stats_shim(this: &RcwnStatus, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "rcwnCacheWonCount")]
    fn rcwn_cache_won_count_shim(this: &RcwnStatus, val: u32);
    #[wasm_bindgen(method, setter = "rcwnNetWonCount")]
    fn rcwn_net_won_count_shim(this: &RcwnStatus, val: u32);
    #[wasm_bindgen(method, setter = "totalNetworkRequests")]
    fn total_network_requests_shim(this: &RcwnStatus, val: u32);
}
impl RcwnStatus {
    #[doc = "Construct a new `RcwnStatus`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `cacheNotSlowCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    pub fn cache_not_slow_count(&mut self, val: u32) -> &mut Self {
        self.cache_not_slow_count_shim(val);
        self
    }
    #[doc = "Change the `cacheSlowCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    pub fn cache_slow_count(&mut self, val: u32) -> &mut Self {
        self.cache_slow_count_shim(val);
        self
    }
    #[doc = "Change the `perfStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    pub fn perf_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.perf_stats_shim(val);
        self
    }
    #[doc = "Change the `rcwnCacheWonCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    pub fn rcwn_cache_won_count(&mut self, val: u32) -> &mut Self {
        self.rcwn_cache_won_count_shim(val);
        self
    }
    #[doc = "Change the `rcwnNetWonCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    pub fn rcwn_net_won_count(&mut self, val: u32) -> &mut Self {
        self.rcwn_net_won_count_shim(val);
        self
    }
    #[doc = "Change the `totalNetworkRequests` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    pub fn total_network_requests(&mut self, val: u32) -> &mut Self {
        self.total_network_requests_shim(val);
        self
    }
}
impl Default for RcwnStatus {
    fn default() -> Self {
        Self::new()
    }
}
