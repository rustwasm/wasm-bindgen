#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = NetworkResultOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NetworkResultOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub type NetworkResultOptions;
    #[doc = "Get the `broadcast` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "broadcast")]
    pub fn get_broadcast(this: &NetworkResultOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "broadcast")]
    fn set_broadcast(this: &NetworkResultOptions, val: bool);
    #[doc = "Get the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "curExternalIfname")]
    pub fn get_cur_external_ifname(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "curExternalIfname")]
    fn set_cur_external_ifname(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "curInternalIfname")]
    pub fn get_cur_internal_ifname(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "curInternalIfname")]
    fn set_cur_internal_ifname(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "dns1")]
    pub fn get_dns1(this: &NetworkResultOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "dns1")]
    fn set_dns1(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `dns1_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "dns1_str")]
    pub fn get_dns1_str(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "dns1_str")]
    fn set_dns1_str(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "dns2")]
    pub fn get_dns2(this: &NetworkResultOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "dns2")]
    fn set_dns2(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `dns2_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "dns2_str")]
    pub fn get_dns2_str(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "dns2_str")]
    fn set_dns2_str(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "enable")]
    pub fn get_enable(this: &NetworkResultOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "enable")]
    fn set_enable(this: &NetworkResultOptions, val: bool);
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &NetworkResultOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "error")]
    fn set_error(this: &NetworkResultOptions, val: bool);
    #[doc = "Get the `flag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "flag")]
    pub fn get_flag(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "flag")]
    fn set_flag(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "gateway")]
    pub fn get_gateway(this: &NetworkResultOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "gateway")]
    fn set_gateway(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `gateway_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "gateway_str")]
    pub fn get_gateway_str(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "gateway_str")]
    fn set_gateway_str(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &NetworkResultOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "interfaceList")]
    pub fn get_interface_list(this: &NetworkResultOptions) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "interfaceList")]
    fn set_interface_list(this: &NetworkResultOptions, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `ipAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "ipAddr")]
    pub fn get_ip_addr(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "ipAddr")]
    fn set_ip_addr(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "ipaddr")]
    pub fn get_ipaddr(this: &NetworkResultOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "ipaddr")]
    fn set_ipaddr(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `ipaddr_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "ipaddr_str")]
    pub fn get_ipaddr_str(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "ipaddr_str")]
    fn set_ipaddr_str(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `lease` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "lease")]
    pub fn get_lease(this: &NetworkResultOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "lease")]
    fn set_lease(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `macAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "macAddr")]
    pub fn get_mac_addr(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "macAddr")]
    fn set_mac_addr(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "mask")]
    pub fn get_mask(this: &NetworkResultOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "mask")]
    fn set_mask(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `mask_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "mask_str")]
    pub fn get_mask_str(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "mask_str")]
    fn set_mask_str(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `netId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "netId")]
    pub fn get_net_id(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "netId")]
    fn set_net_id(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "prefixLength")]
    pub fn get_prefix_length(this: &NetworkResultOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "prefixLength")]
    fn set_prefix_length(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "reason")]
    pub fn get_reason(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "reason")]
    fn set_reason(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `reply` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "reply")]
    pub fn get_reply(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "reply")]
    fn set_reply(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `result` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "result")]
    pub fn get_result(this: &NetworkResultOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "result")]
    fn set_result(this: &NetworkResultOptions, val: bool);
    #[doc = "Get the `resultCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "resultCode")]
    pub fn get_result_code(this: &NetworkResultOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "resultCode")]
    fn set_result_code(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `resultReason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "resultReason")]
    pub fn get_result_reason(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "resultReason")]
    fn set_result_reason(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `ret` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "ret")]
    pub fn get_ret(this: &NetworkResultOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "ret")]
    fn set_ret(this: &NetworkResultOptions, val: bool);
    #[doc = "Get the `route` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "route")]
    pub fn get_route(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "route")]
    fn set_route(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `server` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "server")]
    pub fn get_server(this: &NetworkResultOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "server")]
    fn set_server(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `server_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "server_str")]
    pub fn get_server_str(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "server_str")]
    fn set_server_str(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `success` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "success")]
    pub fn get_success(this: &NetworkResultOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "success")]
    fn set_success(this: &NetworkResultOptions, val: bool);
    #[doc = "Get the `topic` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "topic")]
    pub fn get_topic(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "topic")]
    fn set_topic(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `vendor_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "vendor_str")]
    pub fn get_vendor_str(this: &NetworkResultOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "vendor_str")]
    fn set_vendor_str(this: &NetworkResultOptions, val: &str);
}
impl NetworkResultOptions {
    #[doc = "Construct a new `NetworkResultOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `broadcast` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn broadcast(&mut self, val: bool) -> &mut Self {
        self.set_broadcast(val);
        self
    }
    #[doc = "Change the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn cur_external_ifname(&mut self, val: &str) -> &mut Self {
        self.set_cur_external_ifname(val);
        self
    }
    #[doc = "Change the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn cur_internal_ifname(&mut self, val: &str) -> &mut Self {
        self.set_cur_internal_ifname(val);
        self
    }
    #[doc = "Change the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn dns1(&mut self, val: i32) -> &mut Self {
        self.set_dns1(val);
        self
    }
    #[doc = "Change the `dns1_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn dns1_str(&mut self, val: &str) -> &mut Self {
        self.set_dns1_str(val);
        self
    }
    #[doc = "Change the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn dns2(&mut self, val: i32) -> &mut Self {
        self.set_dns2(val);
        self
    }
    #[doc = "Change the `dns2_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn dns2_str(&mut self, val: &str) -> &mut Self {
        self.set_dns2_str(val);
        self
    }
    #[doc = "Change the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn enable(&mut self, val: bool) -> &mut Self {
        self.set_enable(val);
        self
    }
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn error(&mut self, val: bool) -> &mut Self {
        self.set_error(val);
        self
    }
    #[doc = "Change the `flag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn flag(&mut self, val: &str) -> &mut Self {
        self.set_flag(val);
        self
    }
    #[doc = "Change the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn gateway(&mut self, val: i32) -> &mut Self {
        self.set_gateway(val);
        self
    }
    #[doc = "Change the `gateway_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn gateway_str(&mut self, val: &str) -> &mut Self {
        self.set_gateway_str(val);
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id(val);
        self
    }
    #[doc = "Change the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn interface_list(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_interface_list(val);
        self
    }
    #[doc = "Change the `ipAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn ip_addr(&mut self, val: &str) -> &mut Self {
        self.set_ip_addr(val);
        self
    }
    #[doc = "Change the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn ipaddr(&mut self, val: i32) -> &mut Self {
        self.set_ipaddr(val);
        self
    }
    #[doc = "Change the `ipaddr_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn ipaddr_str(&mut self, val: &str) -> &mut Self {
        self.set_ipaddr_str(val);
        self
    }
    #[doc = "Change the `lease` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn lease(&mut self, val: i32) -> &mut Self {
        self.set_lease(val);
        self
    }
    #[doc = "Change the `macAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn mac_addr(&mut self, val: &str) -> &mut Self {
        self.set_mac_addr(val);
        self
    }
    #[doc = "Change the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn mask(&mut self, val: i32) -> &mut Self {
        self.set_mask(val);
        self
    }
    #[doc = "Change the `mask_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn mask_str(&mut self, val: &str) -> &mut Self {
        self.set_mask_str(val);
        self
    }
    #[doc = "Change the `netId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn net_id(&mut self, val: &str) -> &mut Self {
        self.set_net_id(val);
        self
    }
    #[doc = "Change the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn prefix_length(&mut self, val: i32) -> &mut Self {
        self.set_prefix_length(val);
        self
    }
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn reason(&mut self, val: &str) -> &mut Self {
        self.set_reason(val);
        self
    }
    #[doc = "Change the `reply` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn reply(&mut self, val: &str) -> &mut Self {
        self.set_reply(val);
        self
    }
    #[doc = "Change the `result` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn result(&mut self, val: bool) -> &mut Self {
        self.set_result(val);
        self
    }
    #[doc = "Change the `resultCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn result_code(&mut self, val: i32) -> &mut Self {
        self.set_result_code(val);
        self
    }
    #[doc = "Change the `resultReason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn result_reason(&mut self, val: &str) -> &mut Self {
        self.set_result_reason(val);
        self
    }
    #[doc = "Change the `ret` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn ret(&mut self, val: bool) -> &mut Self {
        self.set_ret(val);
        self
    }
    #[doc = "Change the `route` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn route(&mut self, val: &str) -> &mut Self {
        self.set_route(val);
        self
    }
    #[doc = "Change the `server` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn server(&mut self, val: i32) -> &mut Self {
        self.set_server(val);
        self
    }
    #[doc = "Change the `server_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn server_str(&mut self, val: &str) -> &mut Self {
        self.set_server_str(val);
        self
    }
    #[doc = "Change the `success` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn success(&mut self, val: bool) -> &mut Self {
        self.set_success(val);
        self
    }
    #[doc = "Change the `topic` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn topic(&mut self, val: &str) -> &mut Self {
        self.set_topic(val);
        self
    }
    #[doc = "Change the `vendor_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn vendor_str(&mut self, val: &str) -> &mut Self {
        self.set_vendor_str(val);
        self
    }
}
impl Default for NetworkResultOptions {
    fn default() -> Self {
        Self::new()
    }
}
