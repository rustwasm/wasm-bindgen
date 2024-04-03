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
    #[wasm_bindgen(method, setter = "broadcast")]
    fn broadcast_shim(this: &NetworkResultOptions, val: bool);
    #[wasm_bindgen(method, setter = "curExternalIfname")]
    fn cur_external_ifname_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "curInternalIfname")]
    fn cur_internal_ifname_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "dns1")]
    fn dns1_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, setter = "dns1_str")]
    fn dns1_str_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "dns2")]
    fn dns2_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, setter = "dns2_str")]
    fn dns2_str_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "enable")]
    fn enable_shim(this: &NetworkResultOptions, val: bool);
    #[wasm_bindgen(method, setter = "error")]
    fn error_shim(this: &NetworkResultOptions, val: bool);
    #[wasm_bindgen(method, setter = "flag")]
    fn flag_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "gateway")]
    fn gateway_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, setter = "gateway_str")]
    fn gateway_str_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, setter = "interfaceList")]
    fn interface_list_shim(this: &NetworkResultOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "ipAddr")]
    fn ip_addr_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "ipaddr")]
    fn ipaddr_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, setter = "ipaddr_str")]
    fn ipaddr_str_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "lease")]
    fn lease_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, setter = "macAddr")]
    fn mac_addr_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "mask")]
    fn mask_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, setter = "mask_str")]
    fn mask_str_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "netId")]
    fn net_id_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "prefixLength")]
    fn prefix_length_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, setter = "reason")]
    fn reason_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "reply")]
    fn reply_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "result")]
    fn result_shim(this: &NetworkResultOptions, val: bool);
    #[wasm_bindgen(method, setter = "resultCode")]
    fn result_code_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, setter = "resultReason")]
    fn result_reason_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "ret")]
    fn ret_shim(this: &NetworkResultOptions, val: bool);
    #[wasm_bindgen(method, setter = "route")]
    fn route_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "server")]
    fn server_shim(this: &NetworkResultOptions, val: i32);
    #[wasm_bindgen(method, setter = "server_str")]
    fn server_str_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "success")]
    fn success_shim(this: &NetworkResultOptions, val: bool);
    #[wasm_bindgen(method, setter = "topic")]
    fn topic_shim(this: &NetworkResultOptions, val: &str);
    #[wasm_bindgen(method, setter = "vendor_str")]
    fn vendor_str_shim(this: &NetworkResultOptions, val: &str);
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
        self.broadcast_shim(val);
        self
    }
    #[doc = "Change the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn cur_external_ifname(&mut self, val: &str) -> &mut Self {
        self.cur_external_ifname_shim(val);
        self
    }
    #[doc = "Change the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn cur_internal_ifname(&mut self, val: &str) -> &mut Self {
        self.cur_internal_ifname_shim(val);
        self
    }
    #[doc = "Change the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn dns1(&mut self, val: i32) -> &mut Self {
        self.dns1_shim(val);
        self
    }
    #[doc = "Change the `dns1_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn dns1_str(&mut self, val: &str) -> &mut Self {
        self.dns1_str_shim(val);
        self
    }
    #[doc = "Change the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn dns2(&mut self, val: i32) -> &mut Self {
        self.dns2_shim(val);
        self
    }
    #[doc = "Change the `dns2_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn dns2_str(&mut self, val: &str) -> &mut Self {
        self.dns2_str_shim(val);
        self
    }
    #[doc = "Change the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn enable(&mut self, val: bool) -> &mut Self {
        self.enable_shim(val);
        self
    }
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn error(&mut self, val: bool) -> &mut Self {
        self.error_shim(val);
        self
    }
    #[doc = "Change the `flag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn flag(&mut self, val: &str) -> &mut Self {
        self.flag_shim(val);
        self
    }
    #[doc = "Change the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn gateway(&mut self, val: i32) -> &mut Self {
        self.gateway_shim(val);
        self
    }
    #[doc = "Change the `gateway_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn gateway_str(&mut self, val: &str) -> &mut Self {
        self.gateway_str_shim(val);
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.id_shim(val);
        self
    }
    #[doc = "Change the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn interface_list(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.interface_list_shim(val);
        self
    }
    #[doc = "Change the `ipAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn ip_addr(&mut self, val: &str) -> &mut Self {
        self.ip_addr_shim(val);
        self
    }
    #[doc = "Change the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn ipaddr(&mut self, val: i32) -> &mut Self {
        self.ipaddr_shim(val);
        self
    }
    #[doc = "Change the `ipaddr_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn ipaddr_str(&mut self, val: &str) -> &mut Self {
        self.ipaddr_str_shim(val);
        self
    }
    #[doc = "Change the `lease` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn lease(&mut self, val: i32) -> &mut Self {
        self.lease_shim(val);
        self
    }
    #[doc = "Change the `macAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn mac_addr(&mut self, val: &str) -> &mut Self {
        self.mac_addr_shim(val);
        self
    }
    #[doc = "Change the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn mask(&mut self, val: i32) -> &mut Self {
        self.mask_shim(val);
        self
    }
    #[doc = "Change the `mask_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn mask_str(&mut self, val: &str) -> &mut Self {
        self.mask_str_shim(val);
        self
    }
    #[doc = "Change the `netId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn net_id(&mut self, val: &str) -> &mut Self {
        self.net_id_shim(val);
        self
    }
    #[doc = "Change the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn prefix_length(&mut self, val: i32) -> &mut Self {
        self.prefix_length_shim(val);
        self
    }
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn reason(&mut self, val: &str) -> &mut Self {
        self.reason_shim(val);
        self
    }
    #[doc = "Change the `reply` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn reply(&mut self, val: &str) -> &mut Self {
        self.reply_shim(val);
        self
    }
    #[doc = "Change the `result` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn result(&mut self, val: bool) -> &mut Self {
        self.result_shim(val);
        self
    }
    #[doc = "Change the `resultCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn result_code(&mut self, val: i32) -> &mut Self {
        self.result_code_shim(val);
        self
    }
    #[doc = "Change the `resultReason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn result_reason(&mut self, val: &str) -> &mut Self {
        self.result_reason_shim(val);
        self
    }
    #[doc = "Change the `ret` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn ret(&mut self, val: bool) -> &mut Self {
        self.ret_shim(val);
        self
    }
    #[doc = "Change the `route` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn route(&mut self, val: &str) -> &mut Self {
        self.route_shim(val);
        self
    }
    #[doc = "Change the `server` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn server(&mut self, val: i32) -> &mut Self {
        self.server_shim(val);
        self
    }
    #[doc = "Change the `server_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn server_str(&mut self, val: &str) -> &mut Self {
        self.server_str_shim(val);
        self
    }
    #[doc = "Change the `success` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn success(&mut self, val: bool) -> &mut Self {
        self.success_shim(val);
        self
    }
    #[doc = "Change the `topic` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn topic(&mut self, val: &str) -> &mut Self {
        self.topic_shim(val);
        self
    }
    #[doc = "Change the `vendor_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn vendor_str(&mut self, val: &str) -> &mut Self {
        self.vendor_str_shim(val);
        self
    }
}
impl Default for NetworkResultOptions {
    fn default() -> Self {
        Self::new()
    }
}
