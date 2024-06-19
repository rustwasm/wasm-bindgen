#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = NetworkCommandOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NetworkCommandOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub type NetworkCommandOptions;
    #[doc = "Get the `cmd` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "cmd")]
    pub fn get_cmd(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "cmd")]
    fn set_cmd(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "curExternalIfname")]
    pub fn get_cur_external_ifname(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "curExternalIfname")]
    fn set_cur_external_ifname(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "curInternalIfname")]
    pub fn get_cur_internal_ifname(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "curInternalIfname")]
    fn set_cur_internal_ifname(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "dns1")]
    pub fn get_dns1(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "dns1")]
    fn set_dns1(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `dns1_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "dns1_long")]
    pub fn get_dns1_long(this: &NetworkCommandOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "dns1_long")]
    fn set_dns1_long(this: &NetworkCommandOptions, val: i32);
    #[doc = "Get the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "dns2")]
    pub fn get_dns2(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "dns2")]
    fn set_dns2(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `dns2_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "dns2_long")]
    pub fn get_dns2_long(this: &NetworkCommandOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "dns2_long")]
    fn set_dns2_long(this: &NetworkCommandOptions, val: i32);
    #[doc = "Get the `dnses` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "dnses")]
    pub fn get_dnses(this: &NetworkCommandOptions) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "dnses")]
    fn set_dnses(this: &NetworkCommandOptions, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "domain")]
    pub fn get_domain(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "domain")]
    fn set_domain(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "enable")]
    pub fn get_enable(this: &NetworkCommandOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "enable")]
    fn set_enable(this: &NetworkCommandOptions, val: bool);
    #[doc = "Get the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "enabled")]
    pub fn get_enabled(this: &NetworkCommandOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "enabled")]
    fn set_enabled(this: &NetworkCommandOptions, val: bool);
    #[doc = "Get the `endIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "endIp")]
    pub fn get_end_ip(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "endIp")]
    fn set_end_ip(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `externalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "externalIfname")]
    pub fn get_external_ifname(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "externalIfname")]
    fn set_external_ifname(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "gateway")]
    pub fn get_gateway(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "gateway")]
    fn set_gateway(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `gateway_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "gateway_long")]
    pub fn get_gateway_long(this: &NetworkCommandOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "gateway_long")]
    fn set_gateway_long(this: &NetworkCommandOptions, val: i32);
    #[doc = "Get the `gateways` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "gateways")]
    pub fn get_gateways(this: &NetworkCommandOptions) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "gateways")]
    fn set_gateways(this: &NetworkCommandOptions, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &NetworkCommandOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id(this: &NetworkCommandOptions, val: i32);
    #[doc = "Get the `ifname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "ifname")]
    pub fn get_ifname(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "ifname")]
    fn set_ifname(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "interfaceList")]
    pub fn get_interface_list(this: &NetworkCommandOptions) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "interfaceList")]
    fn set_interface_list(this: &NetworkCommandOptions, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `internalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "internalIfname")]
    pub fn get_internal_ifname(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "internalIfname")]
    fn set_internal_ifname(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `ip` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "ip")]
    pub fn get_ip(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "ip")]
    fn set_ip(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "ipaddr")]
    pub fn get_ipaddr(this: &NetworkCommandOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "ipaddr")]
    fn set_ipaddr(this: &NetworkCommandOptions, val: i32);
    #[doc = "Get the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "key")]
    pub fn get_key(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "key")]
    fn set_key(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `link` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "link")]
    pub fn get_link(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "link")]
    fn set_link(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "mask")]
    pub fn get_mask(this: &NetworkCommandOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "mask")]
    fn set_mask(this: &NetworkCommandOptions, val: i32);
    #[doc = "Get the `maskLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "maskLength")]
    pub fn get_mask_length(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "maskLength")]
    fn set_mask_length(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "mode")]
    pub fn get_mode(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "mode")]
    fn set_mode(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `mtu` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "mtu")]
    pub fn get_mtu(this: &NetworkCommandOptions) -> Option<i32>;
    #[wasm_bindgen(method, setter = "mtu")]
    fn set_mtu(this: &NetworkCommandOptions, val: i32);
    #[doc = "Get the `preExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "preExternalIfname")]
    pub fn get_pre_external_ifname(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "preExternalIfname")]
    fn set_pre_external_ifname(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `preInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "preInternalIfname")]
    pub fn get_pre_internal_ifname(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "preInternalIfname")]
    fn set_pre_internal_ifname(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "prefix")]
    pub fn get_prefix(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "prefix")]
    fn set_prefix(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "prefixLength")]
    pub fn get_prefix_length(this: &NetworkCommandOptions) -> Option<u32>;
    #[wasm_bindgen(method, setter = "prefixLength")]
    fn set_prefix_length(this: &NetworkCommandOptions, val: u32);
    #[doc = "Get the `report` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "report")]
    pub fn get_report(this: &NetworkCommandOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "report")]
    fn set_report(this: &NetworkCommandOptions, val: bool);
    #[doc = "Get the `security` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "security")]
    pub fn get_security(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "security")]
    fn set_security(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `serverIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "serverIp")]
    pub fn get_server_ip(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "serverIp")]
    fn set_server_ip(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `ssid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "ssid")]
    pub fn get_ssid(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "ssid")]
    fn set_ssid(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `startIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "startIp")]
    pub fn get_start_ip(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "startIp")]
    fn set_start_ip(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "threshold")]
    pub fn get_threshold(this: &NetworkCommandOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter = "threshold")]
    fn set_threshold(this: &NetworkCommandOptions, val: f64);
    #[doc = "Get the `usbEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "usbEndIp")]
    pub fn get_usb_end_ip(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "usbEndIp")]
    fn set_usb_end_ip(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `usbStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "usbStartIp")]
    pub fn get_usb_start_ip(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "usbStartIp")]
    fn set_usb_start_ip(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `wifiEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "wifiEndIp")]
    pub fn get_wifi_end_ip(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "wifiEndIp")]
    fn set_wifi_end_ip(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `wifiStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "wifiStartIp")]
    pub fn get_wifi_start_ip(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "wifiStartIp")]
    fn set_wifi_start_ip(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `wifictrlinterfacename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "wifictrlinterfacename")]
    pub fn get_wifictrlinterfacename(this: &NetworkCommandOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "wifictrlinterfacename")]
    fn set_wifictrlinterfacename(this: &NetworkCommandOptions, val: &str);
}
impl NetworkCommandOptions {
    #[doc = "Construct a new `NetworkCommandOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `cmd` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn cmd(&mut self, val: &str) -> &mut Self {
        self.set_cmd(val);
        self
    }
    #[doc = "Change the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn cur_external_ifname(&mut self, val: &str) -> &mut Self {
        self.set_cur_external_ifname(val);
        self
    }
    #[doc = "Change the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn cur_internal_ifname(&mut self, val: &str) -> &mut Self {
        self.set_cur_internal_ifname(val);
        self
    }
    #[doc = "Change the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dns1(&mut self, val: &str) -> &mut Self {
        self.set_dns1(val);
        self
    }
    #[doc = "Change the `dns1_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dns1_long(&mut self, val: i32) -> &mut Self {
        self.set_dns1_long(val);
        self
    }
    #[doc = "Change the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dns2(&mut self, val: &str) -> &mut Self {
        self.set_dns2(val);
        self
    }
    #[doc = "Change the `dns2_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dns2_long(&mut self, val: i32) -> &mut Self {
        self.set_dns2_long(val);
        self
    }
    #[doc = "Change the `dnses` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dnses(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_dnses(val);
        self
    }
    #[doc = "Change the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn domain(&mut self, val: &str) -> &mut Self {
        self.set_domain(val);
        self
    }
    #[doc = "Change the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn enable(&mut self, val: bool) -> &mut Self {
        self.set_enable(val);
        self
    }
    #[doc = "Change the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        self.set_enabled(val);
        self
    }
    #[doc = "Change the `endIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn end_ip(&mut self, val: &str) -> &mut Self {
        self.set_end_ip(val);
        self
    }
    #[doc = "Change the `externalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn external_ifname(&mut self, val: &str) -> &mut Self {
        self.set_external_ifname(val);
        self
    }
    #[doc = "Change the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn gateway(&mut self, val: &str) -> &mut Self {
        self.set_gateway(val);
        self
    }
    #[doc = "Change the `gateway_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn gateway_long(&mut self, val: i32) -> &mut Self {
        self.set_gateway_long(val);
        self
    }
    #[doc = "Change the `gateways` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn gateways(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_gateways(val);
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id(val);
        self
    }
    #[doc = "Change the `ifname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn ifname(&mut self, val: &str) -> &mut Self {
        self.set_ifname(val);
        self
    }
    #[doc = "Change the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn interface_list(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_interface_list(val);
        self
    }
    #[doc = "Change the `internalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn internal_ifname(&mut self, val: &str) -> &mut Self {
        self.set_internal_ifname(val);
        self
    }
    #[doc = "Change the `ip` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn ip(&mut self, val: &str) -> &mut Self {
        self.set_ip(val);
        self
    }
    #[doc = "Change the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn ipaddr(&mut self, val: i32) -> &mut Self {
        self.set_ipaddr(val);
        self
    }
    #[doc = "Change the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn key(&mut self, val: &str) -> &mut Self {
        self.set_key(val);
        self
    }
    #[doc = "Change the `link` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn link(&mut self, val: &str) -> &mut Self {
        self.set_link(val);
        self
    }
    #[doc = "Change the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn mask(&mut self, val: i32) -> &mut Self {
        self.set_mask(val);
        self
    }
    #[doc = "Change the `maskLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn mask_length(&mut self, val: &str) -> &mut Self {
        self.set_mask_length(val);
        self
    }
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn mode(&mut self, val: &str) -> &mut Self {
        self.set_mode(val);
        self
    }
    #[doc = "Change the `mtu` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn mtu(&mut self, val: i32) -> &mut Self {
        self.set_mtu(val);
        self
    }
    #[doc = "Change the `preExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn pre_external_ifname(&mut self, val: &str) -> &mut Self {
        self.set_pre_external_ifname(val);
        self
    }
    #[doc = "Change the `preInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn pre_internal_ifname(&mut self, val: &str) -> &mut Self {
        self.set_pre_internal_ifname(val);
        self
    }
    #[doc = "Change the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn prefix(&mut self, val: &str) -> &mut Self {
        self.set_prefix(val);
        self
    }
    #[doc = "Change the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn prefix_length(&mut self, val: u32) -> &mut Self {
        self.set_prefix_length(val);
        self
    }
    #[doc = "Change the `report` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn report(&mut self, val: bool) -> &mut Self {
        self.set_report(val);
        self
    }
    #[doc = "Change the `security` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn security(&mut self, val: &str) -> &mut Self {
        self.set_security(val);
        self
    }
    #[doc = "Change the `serverIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn server_ip(&mut self, val: &str) -> &mut Self {
        self.set_server_ip(val);
        self
    }
    #[doc = "Change the `ssid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn ssid(&mut self, val: &str) -> &mut Self {
        self.set_ssid(val);
        self
    }
    #[doc = "Change the `startIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn start_ip(&mut self, val: &str) -> &mut Self {
        self.set_start_ip(val);
        self
    }
    #[doc = "Change the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn threshold(&mut self, val: f64) -> &mut Self {
        self.set_threshold(val);
        self
    }
    #[doc = "Change the `usbEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn usb_end_ip(&mut self, val: &str) -> &mut Self {
        self.set_usb_end_ip(val);
        self
    }
    #[doc = "Change the `usbStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn usb_start_ip(&mut self, val: &str) -> &mut Self {
        self.set_usb_start_ip(val);
        self
    }
    #[doc = "Change the `wifiEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn wifi_end_ip(&mut self, val: &str) -> &mut Self {
        self.set_wifi_end_ip(val);
        self
    }
    #[doc = "Change the `wifiStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn wifi_start_ip(&mut self, val: &str) -> &mut Self {
        self.set_wifi_start_ip(val);
        self
    }
    #[doc = "Change the `wifictrlinterfacename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn wifictrlinterfacename(&mut self, val: &str) -> &mut Self {
        self.set_wifictrlinterfacename(val);
        self
    }
}
impl Default for NetworkCommandOptions {
    fn default() -> Self {
        Self::new()
    }
}
