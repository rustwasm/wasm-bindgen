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
    #[wasm_bindgen(method, getter = "cacheNotSlowCount")]
    fn cache_not_slow_count_shim(this: &RcwnStatus) -> u32;
    #[wasm_bindgen(method, setter = "cacheNotSlowCount")]
    fn set_cache_not_slow_count_shim(this: &RcwnStatus, val: u32);
    #[wasm_bindgen(method, getter = "cacheSlowCount")]
    fn cache_slow_count_shim(this: &RcwnStatus) -> u32;
    #[wasm_bindgen(method, setter = "cacheSlowCount")]
    fn set_cache_slow_count_shim(this: &RcwnStatus, val: u32);
    #[wasm_bindgen(method, getter = "perfStats")]
    fn perf_stats_shim(this: &RcwnStatus) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "perfStats")]
    fn set_perf_stats_shim(this: &RcwnStatus, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "rcwnCacheWonCount")]
    fn rcwn_cache_won_count_shim(this: &RcwnStatus) -> u32;
    #[wasm_bindgen(method, setter = "rcwnCacheWonCount")]
    fn set_rcwn_cache_won_count_shim(this: &RcwnStatus, val: u32);
    #[wasm_bindgen(method, getter = "rcwnNetWonCount")]
    fn rcwn_net_won_count_shim(this: &RcwnStatus) -> u32;
    #[wasm_bindgen(method, setter = "rcwnNetWonCount")]
    fn set_rcwn_net_won_count_shim(this: &RcwnStatus, val: u32);
    #[wasm_bindgen(method, getter = "totalNetworkRequests")]
    fn total_network_requests_shim(this: &RcwnStatus) -> u32;
    #[wasm_bindgen(method, setter = "totalNetworkRequests")]
    fn set_total_network_requests_shim(this: &RcwnStatus, val: u32);
}
#[doc = "The trait to access properties on the `RcwnStatus` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
pub trait RcwnStatusGetters {
    #[doc = "Get the `cacheNotSlowCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    fn cache_not_slow_count(&self) -> u32;
    #[doc = "Get the `cacheSlowCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    fn cache_slow_count(&self) -> u32;
    #[doc = "Get the `perfStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    fn perf_stats(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `rcwnCacheWonCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    fn rcwn_cache_won_count(&self) -> u32;
    #[doc = "Get the `rcwnNetWonCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    fn rcwn_net_won_count(&self) -> u32;
    #[doc = "Get the `totalNetworkRequests` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    fn total_network_requests(&self) -> u32;
}
impl RcwnStatusGetters for RcwnStatus {
    fn cache_not_slow_count(&self) -> u32 {
        self.cache_not_slow_count_shim()
    }
    fn cache_slow_count(&self) -> u32 {
        self.cache_slow_count_shim()
    }
    fn perf_stats(&self) -> &::wasm_bindgen::JsValue {
        self.perf_stats_shim()
    }
    fn rcwn_cache_won_count(&self) -> u32 {
        self.rcwn_cache_won_count_shim()
    }
    fn rcwn_net_won_count(&self) -> u32 {
        self.rcwn_net_won_count_shim()
    }
    fn total_network_requests(&self) -> u32 {
        self.total_network_requests_shim()
    }
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
        self.set_cache_not_slow_count_shim(val);
        self
    }
    #[doc = "Change the `cacheSlowCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    pub fn cache_slow_count(&mut self, val: u32) -> &mut Self {
        self.set_cache_slow_count_shim(val);
        self
    }
    #[doc = "Change the `perfStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    pub fn perf_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_perf_stats_shim(val);
        self
    }
    #[doc = "Change the `rcwnCacheWonCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    pub fn rcwn_cache_won_count(&mut self, val: u32) -> &mut Self {
        self.set_rcwn_cache_won_count_shim(val);
        self
    }
    #[doc = "Change the `rcwnNetWonCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    pub fn rcwn_net_won_count(&mut self, val: u32) -> &mut Self {
        self.set_rcwn_net_won_count_shim(val);
        self
    }
    #[doc = "Change the `totalNetworkRequests` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    pub fn total_network_requests(&mut self, val: u32) -> &mut Self {
        self.set_total_network_requests_shim(val);
        self
    }
}
impl Default for RcwnStatus {
    fn default() -> Self {
        Self::new()
    }
}
