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
    #[wasm_bindgen(method, getter = "cmd")]
    fn cmd_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "cmd")]
    fn set_cmd_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "curExternalIfname")]
    fn cur_external_ifname_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "curExternalIfname")]
    fn set_cur_external_ifname_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "curInternalIfname")]
    fn cur_internal_ifname_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "curInternalIfname")]
    fn set_cur_internal_ifname_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "dns1")]
    fn dns1_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "dns1")]
    fn set_dns1_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "dns1_long")]
    fn dns1_long_shim(this: &NetworkCommandOptions) -> i32;
    #[wasm_bindgen(method, setter = "dns1_long")]
    fn set_dns1_long_shim(this: &NetworkCommandOptions, val: i32);
    #[wasm_bindgen(method, getter = "dns2")]
    fn dns2_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "dns2")]
    fn set_dns2_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "dns2_long")]
    fn dns2_long_shim(this: &NetworkCommandOptions) -> i32;
    #[wasm_bindgen(method, setter = "dns2_long")]
    fn set_dns2_long_shim(this: &NetworkCommandOptions, val: i32);
    #[wasm_bindgen(method, getter = "dnses")]
    fn dnses_shim(this: &NetworkCommandOptions) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "dnses")]
    fn set_dnses_shim(this: &NetworkCommandOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "domain")]
    fn domain_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "domain")]
    fn set_domain_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "enable")]
    fn enable_shim(this: &NetworkCommandOptions) -> bool;
    #[wasm_bindgen(method, setter = "enable")]
    fn set_enable_shim(this: &NetworkCommandOptions, val: bool);
    #[wasm_bindgen(method, getter = "enabled")]
    fn enabled_shim(this: &NetworkCommandOptions) -> bool;
    #[wasm_bindgen(method, setter = "enabled")]
    fn set_enabled_shim(this: &NetworkCommandOptions, val: bool);
    #[wasm_bindgen(method, getter = "endIp")]
    fn end_ip_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "endIp")]
    fn set_end_ip_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "externalIfname")]
    fn external_ifname_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "externalIfname")]
    fn set_external_ifname_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "gateway")]
    fn gateway_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "gateway")]
    fn set_gateway_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "gateway_long")]
    fn gateway_long_shim(this: &NetworkCommandOptions) -> i32;
    #[wasm_bindgen(method, setter = "gateway_long")]
    fn set_gateway_long_shim(this: &NetworkCommandOptions, val: i32);
    #[wasm_bindgen(method, getter = "gateways")]
    fn gateways_shim(this: &NetworkCommandOptions) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "gateways")]
    fn set_gateways_shim(this: &NetworkCommandOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &NetworkCommandOptions) -> i32;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &NetworkCommandOptions, val: i32);
    #[wasm_bindgen(method, getter = "ifname")]
    fn ifname_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "ifname")]
    fn set_ifname_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "interfaceList")]
    fn interface_list_shim(this: &NetworkCommandOptions) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "interfaceList")]
    fn set_interface_list_shim(this: &NetworkCommandOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "internalIfname")]
    fn internal_ifname_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "internalIfname")]
    fn set_internal_ifname_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "ip")]
    fn ip_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "ip")]
    fn set_ip_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "ipaddr")]
    fn ipaddr_shim(this: &NetworkCommandOptions) -> i32;
    #[wasm_bindgen(method, setter = "ipaddr")]
    fn set_ipaddr_shim(this: &NetworkCommandOptions, val: i32);
    #[wasm_bindgen(method, getter = "key")]
    fn key_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "key")]
    fn set_key_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "link")]
    fn link_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "link")]
    fn set_link_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "mask")]
    fn mask_shim(this: &NetworkCommandOptions) -> i32;
    #[wasm_bindgen(method, setter = "mask")]
    fn set_mask_shim(this: &NetworkCommandOptions, val: i32);
    #[wasm_bindgen(method, getter = "maskLength")]
    fn mask_length_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "maskLength")]
    fn set_mask_length_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "mode")]
    fn mode_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "mode")]
    fn set_mode_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "mtu")]
    fn mtu_shim(this: &NetworkCommandOptions) -> i32;
    #[wasm_bindgen(method, setter = "mtu")]
    fn set_mtu_shim(this: &NetworkCommandOptions, val: i32);
    #[wasm_bindgen(method, getter = "preExternalIfname")]
    fn pre_external_ifname_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "preExternalIfname")]
    fn set_pre_external_ifname_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "preInternalIfname")]
    fn pre_internal_ifname_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "preInternalIfname")]
    fn set_pre_internal_ifname_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "prefix")]
    fn prefix_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "prefix")]
    fn set_prefix_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "prefixLength")]
    fn prefix_length_shim(this: &NetworkCommandOptions) -> u32;
    #[wasm_bindgen(method, setter = "prefixLength")]
    fn set_prefix_length_shim(this: &NetworkCommandOptions, val: u32);
    #[wasm_bindgen(method, getter = "report")]
    fn report_shim(this: &NetworkCommandOptions) -> bool;
    #[wasm_bindgen(method, setter = "report")]
    fn set_report_shim(this: &NetworkCommandOptions, val: bool);
    #[wasm_bindgen(method, getter = "security")]
    fn security_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "security")]
    fn set_security_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "serverIp")]
    fn server_ip_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "serverIp")]
    fn set_server_ip_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "ssid")]
    fn ssid_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "ssid")]
    fn set_ssid_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "startIp")]
    fn start_ip_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "startIp")]
    fn set_start_ip_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "threshold")]
    fn threshold_shim(this: &NetworkCommandOptions) -> f64;
    #[wasm_bindgen(method, setter = "threshold")]
    fn set_threshold_shim(this: &NetworkCommandOptions, val: f64);
    #[wasm_bindgen(method, getter = "usbEndIp")]
    fn usb_end_ip_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "usbEndIp")]
    fn set_usb_end_ip_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "usbStartIp")]
    fn usb_start_ip_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "usbStartIp")]
    fn set_usb_start_ip_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "wifiEndIp")]
    fn wifi_end_ip_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "wifiEndIp")]
    fn set_wifi_end_ip_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "wifiStartIp")]
    fn wifi_start_ip_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "wifiStartIp")]
    fn set_wifi_start_ip_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, getter = "wifictrlinterfacename")]
    fn wifictrlinterfacename_shim(this: &NetworkCommandOptions) -> String;
    #[wasm_bindgen(method, setter = "wifictrlinterfacename")]
    fn set_wifictrlinterfacename_shim(this: &NetworkCommandOptions, val: &str);
}
#[doc = "The trait to access properties on the `NetworkCommandOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
pub trait NetworkCommandOptionsGetters {
    #[doc = "Get the `cmd` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn cmd(&self) -> String;
    #[doc = "Get the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn cur_external_ifname(&self) -> String;
    #[doc = "Get the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn cur_internal_ifname(&self) -> String;
    #[doc = "Get the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn dns1(&self) -> String;
    #[doc = "Get the `dns1_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn dns1_long(&self) -> i32;
    #[doc = "Get the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn dns2(&self) -> String;
    #[doc = "Get the `dns2_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn dns2_long(&self) -> i32;
    #[doc = "Get the `dnses` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn dnses(&self) -> ::js_sys::Array;
    #[doc = "Get the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn domain(&self) -> String;
    #[doc = "Get the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn enable(&self) -> bool;
    #[doc = "Get the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn enabled(&self) -> bool;
    #[doc = "Get the `endIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn end_ip(&self) -> String;
    #[doc = "Get the `externalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn external_ifname(&self) -> String;
    #[doc = "Get the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn gateway(&self) -> String;
    #[doc = "Get the `gateway_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn gateway_long(&self) -> i32;
    #[doc = "Get the `gateways` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn gateways(&self) -> ::js_sys::Array;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn id(&self) -> i32;
    #[doc = "Get the `ifname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn ifname(&self) -> String;
    #[doc = "Get the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn interface_list(&self) -> ::js_sys::Array;
    #[doc = "Get the `internalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn internal_ifname(&self) -> String;
    #[doc = "Get the `ip` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn ip(&self) -> String;
    #[doc = "Get the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn ipaddr(&self) -> i32;
    #[doc = "Get the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn key(&self) -> String;
    #[doc = "Get the `link` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn link(&self) -> String;
    #[doc = "Get the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn mask(&self) -> i32;
    #[doc = "Get the `maskLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn mask_length(&self) -> String;
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn mode(&self) -> String;
    #[doc = "Get the `mtu` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn mtu(&self) -> i32;
    #[doc = "Get the `preExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn pre_external_ifname(&self) -> String;
    #[doc = "Get the `preInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn pre_internal_ifname(&self) -> String;
    #[doc = "Get the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn prefix(&self) -> String;
    #[doc = "Get the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn prefix_length(&self) -> u32;
    #[doc = "Get the `report` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn report(&self) -> bool;
    #[doc = "Get the `security` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn security(&self) -> String;
    #[doc = "Get the `serverIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn server_ip(&self) -> String;
    #[doc = "Get the `ssid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn ssid(&self) -> String;
    #[doc = "Get the `startIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn start_ip(&self) -> String;
    #[doc = "Get the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn threshold(&self) -> f64;
    #[doc = "Get the `usbEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn usb_end_ip(&self) -> String;
    #[doc = "Get the `usbStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn usb_start_ip(&self) -> String;
    #[doc = "Get the `wifiEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn wifi_end_ip(&self) -> String;
    #[doc = "Get the `wifiStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn wifi_start_ip(&self) -> String;
    #[doc = "Get the `wifictrlinterfacename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn wifictrlinterfacename(&self) -> String;
}
impl NetworkCommandOptionsGetters for NetworkCommandOptions {
    fn cmd(&self) -> String {
        self.cmd_shim()
    }
    fn cur_external_ifname(&self) -> String {
        self.cur_external_ifname_shim()
    }
    fn cur_internal_ifname(&self) -> String {
        self.cur_internal_ifname_shim()
    }
    fn dns1(&self) -> String {
        self.dns1_shim()
    }
    fn dns1_long(&self) -> i32 {
        self.dns1_long_shim()
    }
    fn dns2(&self) -> String {
        self.dns2_shim()
    }
    fn dns2_long(&self) -> i32 {
        self.dns2_long_shim()
    }
    fn dnses(&self) -> ::js_sys::Array {
        self.dnses_shim()
    }
    fn domain(&self) -> String {
        self.domain_shim()
    }
    fn enable(&self) -> bool {
        self.enable_shim()
    }
    fn enabled(&self) -> bool {
        self.enabled_shim()
    }
    fn end_ip(&self) -> String {
        self.end_ip_shim()
    }
    fn external_ifname(&self) -> String {
        self.external_ifname_shim()
    }
    fn gateway(&self) -> String {
        self.gateway_shim()
    }
    fn gateway_long(&self) -> i32 {
        self.gateway_long_shim()
    }
    fn gateways(&self) -> ::js_sys::Array {
        self.gateways_shim()
    }
    fn id(&self) -> i32 {
        self.id_shim()
    }
    fn ifname(&self) -> String {
        self.ifname_shim()
    }
    fn interface_list(&self) -> ::js_sys::Array {
        self.interface_list_shim()
    }
    fn internal_ifname(&self) -> String {
        self.internal_ifname_shim()
    }
    fn ip(&self) -> String {
        self.ip_shim()
    }
    fn ipaddr(&self) -> i32 {
        self.ipaddr_shim()
    }
    fn key(&self) -> String {
        self.key_shim()
    }
    fn link(&self) -> String {
        self.link_shim()
    }
    fn mask(&self) -> i32 {
        self.mask_shim()
    }
    fn mask_length(&self) -> String {
        self.mask_length_shim()
    }
    fn mode(&self) -> String {
        self.mode_shim()
    }
    fn mtu(&self) -> i32 {
        self.mtu_shim()
    }
    fn pre_external_ifname(&self) -> String {
        self.pre_external_ifname_shim()
    }
    fn pre_internal_ifname(&self) -> String {
        self.pre_internal_ifname_shim()
    }
    fn prefix(&self) -> String {
        self.prefix_shim()
    }
    fn prefix_length(&self) -> u32 {
        self.prefix_length_shim()
    }
    fn report(&self) -> bool {
        self.report_shim()
    }
    fn security(&self) -> String {
        self.security_shim()
    }
    fn server_ip(&self) -> String {
        self.server_ip_shim()
    }
    fn ssid(&self) -> String {
        self.ssid_shim()
    }
    fn start_ip(&self) -> String {
        self.start_ip_shim()
    }
    fn threshold(&self) -> f64 {
        self.threshold_shim()
    }
    fn usb_end_ip(&self) -> String {
        self.usb_end_ip_shim()
    }
    fn usb_start_ip(&self) -> String {
        self.usb_start_ip_shim()
    }
    fn wifi_end_ip(&self) -> String {
        self.wifi_end_ip_shim()
    }
    fn wifi_start_ip(&self) -> String {
        self.wifi_start_ip_shim()
    }
    fn wifictrlinterfacename(&self) -> String {
        self.wifictrlinterfacename_shim()
    }
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
        self.set_cmd_shim(val);
        self
    }
    #[doc = "Change the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn cur_external_ifname(&mut self, val: &str) -> &mut Self {
        self.set_cur_external_ifname_shim(val);
        self
    }
    #[doc = "Change the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn cur_internal_ifname(&mut self, val: &str) -> &mut Self {
        self.set_cur_internal_ifname_shim(val);
        self
    }
    #[doc = "Change the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dns1(&mut self, val: &str) -> &mut Self {
        self.set_dns1_shim(val);
        self
    }
    #[doc = "Change the `dns1_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dns1_long(&mut self, val: i32) -> &mut Self {
        self.set_dns1_long_shim(val);
        self
    }
    #[doc = "Change the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dns2(&mut self, val: &str) -> &mut Self {
        self.set_dns2_shim(val);
        self
    }
    #[doc = "Change the `dns2_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dns2_long(&mut self, val: i32) -> &mut Self {
        self.set_dns2_long_shim(val);
        self
    }
    #[doc = "Change the `dnses` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dnses(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_dnses_shim(val);
        self
    }
    #[doc = "Change the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn domain(&mut self, val: &str) -> &mut Self {
        self.set_domain_shim(val);
        self
    }
    #[doc = "Change the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn enable(&mut self, val: bool) -> &mut Self {
        self.set_enable_shim(val);
        self
    }
    #[doc = "Change the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        self.set_enabled_shim(val);
        self
    }
    #[doc = "Change the `endIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn end_ip(&mut self, val: &str) -> &mut Self {
        self.set_end_ip_shim(val);
        self
    }
    #[doc = "Change the `externalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn external_ifname(&mut self, val: &str) -> &mut Self {
        self.set_external_ifname_shim(val);
        self
    }
    #[doc = "Change the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn gateway(&mut self, val: &str) -> &mut Self {
        self.set_gateway_shim(val);
        self
    }
    #[doc = "Change the `gateway_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn gateway_long(&mut self, val: i32) -> &mut Self {
        self.set_gateway_long_shim(val);
        self
    }
    #[doc = "Change the `gateways` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn gateways(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_gateways_shim(val);
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id_shim(val);
        self
    }
    #[doc = "Change the `ifname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn ifname(&mut self, val: &str) -> &mut Self {
        self.set_ifname_shim(val);
        self
    }
    #[doc = "Change the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn interface_list(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_interface_list_shim(val);
        self
    }
    #[doc = "Change the `internalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn internal_ifname(&mut self, val: &str) -> &mut Self {
        self.set_internal_ifname_shim(val);
        self
    }
    #[doc = "Change the `ip` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn ip(&mut self, val: &str) -> &mut Self {
        self.set_ip_shim(val);
        self
    }
    #[doc = "Change the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn ipaddr(&mut self, val: i32) -> &mut Self {
        self.set_ipaddr_shim(val);
        self
    }
    #[doc = "Change the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn key(&mut self, val: &str) -> &mut Self {
        self.set_key_shim(val);
        self
    }
    #[doc = "Change the `link` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn link(&mut self, val: &str) -> &mut Self {
        self.set_link_shim(val);
        self
    }
    #[doc = "Change the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn mask(&mut self, val: i32) -> &mut Self {
        self.set_mask_shim(val);
        self
    }
    #[doc = "Change the `maskLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn mask_length(&mut self, val: &str) -> &mut Self {
        self.set_mask_length_shim(val);
        self
    }
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn mode(&mut self, val: &str) -> &mut Self {
        self.set_mode_shim(val);
        self
    }
    #[doc = "Change the `mtu` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn mtu(&mut self, val: i32) -> &mut Self {
        self.set_mtu_shim(val);
        self
    }
    #[doc = "Change the `preExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn pre_external_ifname(&mut self, val: &str) -> &mut Self {
        self.set_pre_external_ifname_shim(val);
        self
    }
    #[doc = "Change the `preInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn pre_internal_ifname(&mut self, val: &str) -> &mut Self {
        self.set_pre_internal_ifname_shim(val);
        self
    }
    #[doc = "Change the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn prefix(&mut self, val: &str) -> &mut Self {
        self.set_prefix_shim(val);
        self
    }
    #[doc = "Change the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn prefix_length(&mut self, val: u32) -> &mut Self {
        self.set_prefix_length_shim(val);
        self
    }
    #[doc = "Change the `report` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn report(&mut self, val: bool) -> &mut Self {
        self.set_report_shim(val);
        self
    }
    #[doc = "Change the `security` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn security(&mut self, val: &str) -> &mut Self {
        self.set_security_shim(val);
        self
    }
    #[doc = "Change the `serverIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn server_ip(&mut self, val: &str) -> &mut Self {
        self.set_server_ip_shim(val);
        self
    }
    #[doc = "Change the `ssid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn ssid(&mut self, val: &str) -> &mut Self {
        self.set_ssid_shim(val);
        self
    }
    #[doc = "Change the `startIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn start_ip(&mut self, val: &str) -> &mut Self {
        self.set_start_ip_shim(val);
        self
    }
    #[doc = "Change the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn threshold(&mut self, val: f64) -> &mut Self {
        self.set_threshold_shim(val);
        self
    }
    #[doc = "Change the `usbEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn usb_end_ip(&mut self, val: &str) -> &mut Self {
        self.set_usb_end_ip_shim(val);
        self
    }
    #[doc = "Change the `usbStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn usb_start_ip(&mut self, val: &str) -> &mut Self {
        self.set_usb_start_ip_shim(val);
        self
    }
    #[doc = "Change the `wifiEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn wifi_end_ip(&mut self, val: &str) -> &mut Self {
        self.set_wifi_end_ip_shim(val);
        self
    }
    #[doc = "Change the `wifiStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn wifi_start_ip(&mut self, val: &str) -> &mut Self {
        self.set_wifi_start_ip_shim(val);
        self
    }
    #[doc = "Change the `wifictrlinterfacename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn wifictrlinterfacename(&mut self, val: &str) -> &mut Self {
        self.set_wifictrlinterfacename_shim(val);
        self
    }
}
impl Default for NetworkCommandOptions {
    fn default() -> Self {
        Self::new()
    }
}
