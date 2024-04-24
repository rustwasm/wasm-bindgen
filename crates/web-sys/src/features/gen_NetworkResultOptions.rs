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
    #[wasm_bindgen(method, getter = "broadcast")]
    fn broadcast_shim(this: &NetworkResultOptions) -> bool;
    #[wasm_bindgen(method, setter = "broadcast")]
    fn set_broadcast_shim(this: &NetworkResultOptions, val: bool);
    #[wasm_bindgen(method, getter = "curExternalIfname")]
    fn cur_external_ifname_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "curExternalIfname")]
    fn set_cur_external_ifname_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "curInternalIfname")]
    fn cur_internal_ifname_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "curInternalIfname")]
    fn set_cur_internal_ifname_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "dns1")]
    fn dns1_shim(this: &NetworkResultOptions) -> i32;
    #[wasm_bindgen(method, setter = "dns1")]
    fn set_dns1_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, getter = "dns1_str")]
    fn dns1_str_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "dns1_str")]
    fn set_dns1_str_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "dns2")]
    fn dns2_shim(this: &NetworkResultOptions) -> i32;
    #[wasm_bindgen(method, setter = "dns2")]
    fn set_dns2_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, getter = "dns2_str")]
    fn dns2_str_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "dns2_str")]
    fn set_dns2_str_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "enable")]
    fn enable_shim(this: &NetworkResultOptions) -> bool;
    #[wasm_bindgen(method, setter = "enable")]
    fn set_enable_shim(this: &NetworkResultOptions, val: bool);
    #[wasm_bindgen(method, getter = "error")]
    fn error_shim(this: &NetworkResultOptions) -> bool;
    #[wasm_bindgen(method, setter = "error")]
    fn set_error_shim(this: &NetworkResultOptions, val: bool);
    #[wasm_bindgen(method, getter = "flag")]
    fn flag_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "flag")]
    fn set_flag_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "gateway")]
    fn gateway_shim(this: &NetworkResultOptions) -> i32;
    #[wasm_bindgen(method, setter = "gateway")]
    fn set_gateway_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, getter = "gateway_str")]
    fn gateway_str_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "gateway_str")]
    fn set_gateway_str_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &NetworkResultOptions) -> i32;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, getter = "interfaceList")]
    fn interface_list_shim(this: &NetworkResultOptions) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "interfaceList")]
    fn set_interface_list_shim(this: &NetworkResultOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "ipAddr")]
    fn ip_addr_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "ipAddr")]
    fn set_ip_addr_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "ipaddr")]
    fn ipaddr_shim(this: &NetworkResultOptions) -> i32;
    #[wasm_bindgen(method, setter = "ipaddr")]
    fn set_ipaddr_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, getter = "ipaddr_str")]
    fn ipaddr_str_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "ipaddr_str")]
    fn set_ipaddr_str_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "lease")]
    fn lease_shim(this: &NetworkResultOptions) -> i32;
    #[wasm_bindgen(method, setter = "lease")]
    fn set_lease_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, getter = "macAddr")]
    fn mac_addr_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "macAddr")]
    fn set_mac_addr_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "mask")]
    fn mask_shim(this: &NetworkResultOptions) -> i32;
    #[wasm_bindgen(method, setter = "mask")]
    fn set_mask_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, getter = "mask_str")]
    fn mask_str_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "mask_str")]
    fn set_mask_str_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "netId")]
    fn net_id_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "netId")]
    fn set_net_id_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "prefixLength")]
    fn prefix_length_shim(this: &NetworkResultOptions) -> i32;
    #[wasm_bindgen(method, setter = "prefixLength")]
    fn set_prefix_length_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, getter = "reason")]
    fn reason_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "reason")]
    fn set_reason_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "reply")]
    fn reply_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "reply")]
    fn set_reply_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "result")]
    fn result_shim(this: &NetworkResultOptions) -> bool;
    #[wasm_bindgen(method, setter = "result")]
    fn set_result_shim(this: &NetworkResultOptions, val: bool);
    #[wasm_bindgen(method, getter = "resultCode")]
    fn result_code_shim(this: &NetworkResultOptions) -> i32;
    #[wasm_bindgen(method, setter = "resultCode")]
    fn set_result_code_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, getter = "resultReason")]
    fn result_reason_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "resultReason")]
    fn set_result_reason_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "ret")]
    fn ret_shim(this: &NetworkResultOptions) -> bool;
    #[wasm_bindgen(method, setter = "ret")]
    fn set_ret_shim(this: &NetworkResultOptions, val: bool);
    #[wasm_bindgen(method, getter = "route")]
    fn route_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "route")]
    fn set_route_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "server")]
    fn server_shim(this: &NetworkResultOptions) -> i32;
    #[wasm_bindgen(method, setter = "server")]
    fn set_server_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, getter = "server_str")]
    fn server_str_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "server_str")]
    fn set_server_str_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "success")]
    fn success_shim(this: &NetworkResultOptions) -> bool;
    #[wasm_bindgen(method, setter = "success")]
    fn set_success_shim(this: &NetworkResultOptions, val: bool);
    #[wasm_bindgen(method, getter = "topic")]
    fn topic_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "topic")]
    fn set_topic_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, getter = "vendor_str")]
    fn vendor_str_shim(this: &NetworkResultOptions) -> String;
    #[wasm_bindgen(method, setter = "vendor_str")]
    fn set_vendor_str_shim(this: &NetworkResultOptions, val: &str);
}
#[doc = "The trait to access properties on the `NetworkResultOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
pub trait NetworkResultOptionsGetters {
    #[doc = "Get the `broadcast` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn broadcast(&self) -> bool;
    #[doc = "Get the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn cur_external_ifname(&self) -> String;
    #[doc = "Get the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn cur_internal_ifname(&self) -> String;
    #[doc = "Get the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn dns1(&self) -> i32;
    #[doc = "Get the `dns1_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn dns1_str(&self) -> String;
    #[doc = "Get the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn dns2(&self) -> i32;
    #[doc = "Get the `dns2_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn dns2_str(&self) -> String;
    #[doc = "Get the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn enable(&self) -> bool;
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn error(&self) -> bool;
    #[doc = "Get the `flag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn flag(&self) -> String;
    #[doc = "Get the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn gateway(&self) -> i32;
    #[doc = "Get the `gateway_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn gateway_str(&self) -> String;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn id(&self) -> i32;
    #[doc = "Get the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn interface_list(&self) -> ::js_sys::Array;
    #[doc = "Get the `ipAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn ip_addr(&self) -> String;
    #[doc = "Get the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn ipaddr(&self) -> i32;
    #[doc = "Get the `ipaddr_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn ipaddr_str(&self) -> String;
    #[doc = "Get the `lease` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn lease(&self) -> i32;
    #[doc = "Get the `macAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn mac_addr(&self) -> String;
    #[doc = "Get the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn mask(&self) -> i32;
    #[doc = "Get the `mask_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn mask_str(&self) -> String;
    #[doc = "Get the `netId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn net_id(&self) -> String;
    #[doc = "Get the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn prefix_length(&self) -> i32;
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn reason(&self) -> String;
    #[doc = "Get the `reply` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn reply(&self) -> String;
    #[doc = "Get the `result` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn result(&self) -> bool;
    #[doc = "Get the `resultCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn result_code(&self) -> i32;
    #[doc = "Get the `resultReason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn result_reason(&self) -> String;
    #[doc = "Get the `ret` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn ret(&self) -> bool;
    #[doc = "Get the `route` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn route(&self) -> String;
    #[doc = "Get the `server` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn server(&self) -> i32;
    #[doc = "Get the `server_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn server_str(&self) -> String;
    #[doc = "Get the `success` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn success(&self) -> bool;
    #[doc = "Get the `topic` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn topic(&self) -> String;
    #[doc = "Get the `vendor_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn vendor_str(&self) -> String;
}
impl NetworkResultOptionsGetters for NetworkResultOptions {
    fn broadcast(&self) -> bool {
        self.broadcast_shim()
    }
    fn cur_external_ifname(&self) -> String {
        self.cur_external_ifname_shim()
    }
    fn cur_internal_ifname(&self) -> String {
        self.cur_internal_ifname_shim()
    }
    fn dns1(&self) -> i32 {
        self.dns1_shim()
    }
    fn dns1_str(&self) -> String {
        self.dns1_str_shim()
    }
    fn dns2(&self) -> i32 {
        self.dns2_shim()
    }
    fn dns2_str(&self) -> String {
        self.dns2_str_shim()
    }
    fn enable(&self) -> bool {
        self.enable_shim()
    }
    fn error(&self) -> bool {
        self.error_shim()
    }
    fn flag(&self) -> String {
        self.flag_shim()
    }
    fn gateway(&self) -> i32 {
        self.gateway_shim()
    }
    fn gateway_str(&self) -> String {
        self.gateway_str_shim()
    }
    fn id(&self) -> i32 {
        self.id_shim()
    }
    fn interface_list(&self) -> ::js_sys::Array {
        self.interface_list_shim()
    }
    fn ip_addr(&self) -> String {
        self.ip_addr_shim()
    }
    fn ipaddr(&self) -> i32 {
        self.ipaddr_shim()
    }
    fn ipaddr_str(&self) -> String {
        self.ipaddr_str_shim()
    }
    fn lease(&self) -> i32 {
        self.lease_shim()
    }
    fn mac_addr(&self) -> String {
        self.mac_addr_shim()
    }
    fn mask(&self) -> i32 {
        self.mask_shim()
    }
    fn mask_str(&self) -> String {
        self.mask_str_shim()
    }
    fn net_id(&self) -> String {
        self.net_id_shim()
    }
    fn prefix_length(&self) -> i32 {
        self.prefix_length_shim()
    }
    fn reason(&self) -> String {
        self.reason_shim()
    }
    fn reply(&self) -> String {
        self.reply_shim()
    }
    fn result(&self) -> bool {
        self.result_shim()
    }
    fn result_code(&self) -> i32 {
        self.result_code_shim()
    }
    fn result_reason(&self) -> String {
        self.result_reason_shim()
    }
    fn ret(&self) -> bool {
        self.ret_shim()
    }
    fn route(&self) -> String {
        self.route_shim()
    }
    fn server(&self) -> i32 {
        self.server_shim()
    }
    fn server_str(&self) -> String {
        self.server_str_shim()
    }
    fn success(&self) -> bool {
        self.success_shim()
    }
    fn topic(&self) -> String {
        self.topic_shim()
    }
    fn vendor_str(&self) -> String {
        self.vendor_str_shim()
    }
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
        self.set_broadcast_shim(val);
        self
    }
    #[doc = "Change the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn cur_external_ifname(&mut self, val: &str) -> &mut Self {
        self.set_cur_external_ifname_shim(val);
        self
    }
    #[doc = "Change the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn cur_internal_ifname(&mut self, val: &str) -> &mut Self {
        self.set_cur_internal_ifname_shim(val);
        self
    }
    #[doc = "Change the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn dns1(&mut self, val: i32) -> &mut Self {
        self.set_dns1_shim(val);
        self
    }
    #[doc = "Change the `dns1_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn dns1_str(&mut self, val: &str) -> &mut Self {
        self.set_dns1_str_shim(val);
        self
    }
    #[doc = "Change the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn dns2(&mut self, val: i32) -> &mut Self {
        self.set_dns2_shim(val);
        self
    }
    #[doc = "Change the `dns2_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn dns2_str(&mut self, val: &str) -> &mut Self {
        self.set_dns2_str_shim(val);
        self
    }
    #[doc = "Change the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn enable(&mut self, val: bool) -> &mut Self {
        self.set_enable_shim(val);
        self
    }
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn error(&mut self, val: bool) -> &mut Self {
        self.set_error_shim(val);
        self
    }
    #[doc = "Change the `flag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn flag(&mut self, val: &str) -> &mut Self {
        self.set_flag_shim(val);
        self
    }
    #[doc = "Change the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn gateway(&mut self, val: i32) -> &mut Self {
        self.set_gateway_shim(val);
        self
    }
    #[doc = "Change the `gateway_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn gateway_str(&mut self, val: &str) -> &mut Self {
        self.set_gateway_str_shim(val);
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id_shim(val);
        self
    }
    #[doc = "Change the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn interface_list(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_interface_list_shim(val);
        self
    }
    #[doc = "Change the `ipAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn ip_addr(&mut self, val: &str) -> &mut Self {
        self.set_ip_addr_shim(val);
        self
    }
    #[doc = "Change the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn ipaddr(&mut self, val: i32) -> &mut Self {
        self.set_ipaddr_shim(val);
        self
    }
    #[doc = "Change the `ipaddr_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn ipaddr_str(&mut self, val: &str) -> &mut Self {
        self.set_ipaddr_str_shim(val);
        self
    }
    #[doc = "Change the `lease` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn lease(&mut self, val: i32) -> &mut Self {
        self.set_lease_shim(val);
        self
    }
    #[doc = "Change the `macAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn mac_addr(&mut self, val: &str) -> &mut Self {
        self.set_mac_addr_shim(val);
        self
    }
    #[doc = "Change the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn mask(&mut self, val: i32) -> &mut Self {
        self.set_mask_shim(val);
        self
    }
    #[doc = "Change the `mask_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn mask_str(&mut self, val: &str) -> &mut Self {
        self.set_mask_str_shim(val);
        self
    }
    #[doc = "Change the `netId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn net_id(&mut self, val: &str) -> &mut Self {
        self.set_net_id_shim(val);
        self
    }
    #[doc = "Change the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn prefix_length(&mut self, val: i32) -> &mut Self {
        self.set_prefix_length_shim(val);
        self
    }
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn reason(&mut self, val: &str) -> &mut Self {
        self.set_reason_shim(val);
        self
    }
    #[doc = "Change the `reply` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn reply(&mut self, val: &str) -> &mut Self {
        self.set_reply_shim(val);
        self
    }
    #[doc = "Change the `result` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn result(&mut self, val: bool) -> &mut Self {
        self.set_result_shim(val);
        self
    }
    #[doc = "Change the `resultCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn result_code(&mut self, val: i32) -> &mut Self {
        self.set_result_code_shim(val);
        self
    }
    #[doc = "Change the `resultReason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn result_reason(&mut self, val: &str) -> &mut Self {
        self.set_result_reason_shim(val);
        self
    }
    #[doc = "Change the `ret` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn ret(&mut self, val: bool) -> &mut Self {
        self.set_ret_shim(val);
        self
    }
    #[doc = "Change the `route` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn route(&mut self, val: &str) -> &mut Self {
        self.set_route_shim(val);
        self
    }
    #[doc = "Change the `server` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn server(&mut self, val: i32) -> &mut Self {
        self.set_server_shim(val);
        self
    }
    #[doc = "Change the `server_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn server_str(&mut self, val: &str) -> &mut Self {
        self.set_server_str_shim(val);
        self
    }
    #[doc = "Change the `success` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn success(&mut self, val: bool) -> &mut Self {
        self.set_success_shim(val);
        self
    }
    #[doc = "Change the `topic` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn topic(&mut self, val: &str) -> &mut Self {
        self.set_topic_shim(val);
        self
    }
    #[doc = "Change the `vendor_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn vendor_str(&mut self, val: &str) -> &mut Self {
        self.set_vendor_str_shim(val);
        self
    }
}
impl Default for NetworkResultOptions {
    fn default() -> Self {
        Self::new()
    }
}
