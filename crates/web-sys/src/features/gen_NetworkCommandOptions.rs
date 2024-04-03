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
    #[wasm_bindgen(method, setter = "cmd")]
    fn cmd_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "curExternalIfname")]
    fn cur_external_ifname_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "curInternalIfname")]
    fn cur_internal_ifname_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "dns1")]
    fn dns1_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "dns1_long")]
    fn dns1_long_shim(this: &NetworkCommandOptions, val: i32);
    #[wasm_bindgen(method, setter = "dns2")]
    fn dns2_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "dns2_long")]
    fn dns2_long_shim(this: &NetworkCommandOptions, val: i32);
    #[wasm_bindgen(method, setter = "dnses")]
    fn dnses_shim(this: &NetworkCommandOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "domain")]
    fn domain_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "enable")]
    fn enable_shim(this: &NetworkCommandOptions, val: bool);
    #[wasm_bindgen(method, setter = "enabled")]
    fn enabled_shim(this: &NetworkCommandOptions, val: bool);
    #[wasm_bindgen(method, setter = "endIp")]
    fn end_ip_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "externalIfname")]
    fn external_ifname_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "gateway")]
    fn gateway_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "gateway_long")]
    fn gateway_long_shim(this: &NetworkCommandOptions, val: i32);
    #[wasm_bindgen(method, setter = "gateways")]
    fn gateways_shim(this: &NetworkCommandOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &NetworkCommandOptions, val: i32);
    #[wasm_bindgen(method, setter = "ifname")]
    fn ifname_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "interfaceList")]
    fn interface_list_shim(this: &NetworkCommandOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "internalIfname")]
    fn internal_ifname_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "ip")]
    fn ip_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "ipaddr")]
    fn ipaddr_shim(this: &NetworkCommandOptions, val: i32);
    #[wasm_bindgen(method, setter = "key")]
    fn key_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "link")]
    fn link_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "mask")]
    fn mask_shim(this: &NetworkCommandOptions, val: i32);
    #[wasm_bindgen(method, setter = "maskLength")]
    fn mask_length_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "mode")]
    fn mode_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "mtu")]
    fn mtu_shim(this: &NetworkCommandOptions, val: i32);
    #[wasm_bindgen(method, setter = "preExternalIfname")]
    fn pre_external_ifname_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "preInternalIfname")]
    fn pre_internal_ifname_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "prefix")]
    fn prefix_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "prefixLength")]
    fn prefix_length_shim(this: &NetworkCommandOptions, val: u32);
    #[wasm_bindgen(method, setter = "report")]
    fn report_shim(this: &NetworkCommandOptions, val: bool);
    #[wasm_bindgen(method, setter = "security")]
    fn security_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "serverIp")]
    fn server_ip_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "ssid")]
    fn ssid_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "startIp")]
    fn start_ip_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "threshold")]
    fn threshold_shim(this: &NetworkCommandOptions, val: f64);
    #[wasm_bindgen(method, setter = "usbEndIp")]
    fn usb_end_ip_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "usbStartIp")]
    fn usb_start_ip_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "wifiEndIp")]
    fn wifi_end_ip_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "wifiStartIp")]
    fn wifi_start_ip_shim(this: &NetworkCommandOptions, val: &str);
    #[wasm_bindgen(method, setter = "wifictrlinterfacename")]
    fn wifictrlinterfacename_shim(this: &NetworkCommandOptions, val: &str);
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
        self.cmd_shim(val);
        self
    }
    #[doc = "Change the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn cur_external_ifname(&mut self, val: &str) -> &mut Self {
        self.cur_external_ifname_shim(val);
        self
    }
    #[doc = "Change the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn cur_internal_ifname(&mut self, val: &str) -> &mut Self {
        self.cur_internal_ifname_shim(val);
        self
    }
    #[doc = "Change the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dns1(&mut self, val: &str) -> &mut Self {
        self.dns1_shim(val);
        self
    }
    #[doc = "Change the `dns1_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dns1_long(&mut self, val: i32) -> &mut Self {
        self.dns1_long_shim(val);
        self
    }
    #[doc = "Change the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dns2(&mut self, val: &str) -> &mut Self {
        self.dns2_shim(val);
        self
    }
    #[doc = "Change the `dns2_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dns2_long(&mut self, val: i32) -> &mut Self {
        self.dns2_long_shim(val);
        self
    }
    #[doc = "Change the `dnses` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dnses(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.dnses_shim(val);
        self
    }
    #[doc = "Change the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn domain(&mut self, val: &str) -> &mut Self {
        self.domain_shim(val);
        self
    }
    #[doc = "Change the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn enable(&mut self, val: bool) -> &mut Self {
        self.enable_shim(val);
        self
    }
    #[doc = "Change the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        self.enabled_shim(val);
        self
    }
    #[doc = "Change the `endIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn end_ip(&mut self, val: &str) -> &mut Self {
        self.end_ip_shim(val);
        self
    }
    #[doc = "Change the `externalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn external_ifname(&mut self, val: &str) -> &mut Self {
        self.external_ifname_shim(val);
        self
    }
    #[doc = "Change the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn gateway(&mut self, val: &str) -> &mut Self {
        self.gateway_shim(val);
        self
    }
    #[doc = "Change the `gateway_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn gateway_long(&mut self, val: i32) -> &mut Self {
        self.gateway_long_shim(val);
        self
    }
    #[doc = "Change the `gateways` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn gateways(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.gateways_shim(val);
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.id_shim(val);
        self
    }
    #[doc = "Change the `ifname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn ifname(&mut self, val: &str) -> &mut Self {
        self.ifname_shim(val);
        self
    }
    #[doc = "Change the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn interface_list(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.interface_list_shim(val);
        self
    }
    #[doc = "Change the `internalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn internal_ifname(&mut self, val: &str) -> &mut Self {
        self.internal_ifname_shim(val);
        self
    }
    #[doc = "Change the `ip` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn ip(&mut self, val: &str) -> &mut Self {
        self.ip_shim(val);
        self
    }
    #[doc = "Change the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn ipaddr(&mut self, val: i32) -> &mut Self {
        self.ipaddr_shim(val);
        self
    }
    #[doc = "Change the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn key(&mut self, val: &str) -> &mut Self {
        self.key_shim(val);
        self
    }
    #[doc = "Change the `link` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn link(&mut self, val: &str) -> &mut Self {
        self.link_shim(val);
        self
    }
    #[doc = "Change the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn mask(&mut self, val: i32) -> &mut Self {
        self.mask_shim(val);
        self
    }
    #[doc = "Change the `maskLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn mask_length(&mut self, val: &str) -> &mut Self {
        self.mask_length_shim(val);
        self
    }
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn mode(&mut self, val: &str) -> &mut Self {
        self.mode_shim(val);
        self
    }
    #[doc = "Change the `mtu` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn mtu(&mut self, val: i32) -> &mut Self {
        self.mtu_shim(val);
        self
    }
    #[doc = "Change the `preExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn pre_external_ifname(&mut self, val: &str) -> &mut Self {
        self.pre_external_ifname_shim(val);
        self
    }
    #[doc = "Change the `preInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn pre_internal_ifname(&mut self, val: &str) -> &mut Self {
        self.pre_internal_ifname_shim(val);
        self
    }
    #[doc = "Change the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn prefix(&mut self, val: &str) -> &mut Self {
        self.prefix_shim(val);
        self
    }
    #[doc = "Change the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn prefix_length(&mut self, val: u32) -> &mut Self {
        self.prefix_length_shim(val);
        self
    }
    #[doc = "Change the `report` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn report(&mut self, val: bool) -> &mut Self {
        self.report_shim(val);
        self
    }
    #[doc = "Change the `security` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn security(&mut self, val: &str) -> &mut Self {
        self.security_shim(val);
        self
    }
    #[doc = "Change the `serverIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn server_ip(&mut self, val: &str) -> &mut Self {
        self.server_ip_shim(val);
        self
    }
    #[doc = "Change the `ssid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn ssid(&mut self, val: &str) -> &mut Self {
        self.ssid_shim(val);
        self
    }
    #[doc = "Change the `startIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn start_ip(&mut self, val: &str) -> &mut Self {
        self.start_ip_shim(val);
        self
    }
    #[doc = "Change the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn threshold(&mut self, val: f64) -> &mut Self {
        self.threshold_shim(val);
        self
    }
    #[doc = "Change the `usbEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn usb_end_ip(&mut self, val: &str) -> &mut Self {
        self.usb_end_ip_shim(val);
        self
    }
    #[doc = "Change the `usbStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn usb_start_ip(&mut self, val: &str) -> &mut Self {
        self.usb_start_ip_shim(val);
        self
    }
    #[doc = "Change the `wifiEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn wifi_end_ip(&mut self, val: &str) -> &mut Self {
        self.wifi_end_ip_shim(val);
        self
    }
    #[doc = "Change the `wifiStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn wifi_start_ip(&mut self, val: &str) -> &mut Self {
        self.wifi_start_ip_shim(val);
        self
    }
    #[doc = "Change the `wifictrlinterfacename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn wifictrlinterfacename(&mut self, val: &str) -> &mut Self {
        self.wifictrlinterfacename_shim(val);
        self
    }
}
impl Default for NetworkCommandOptions {
    fn default() -> Self {
        Self::new()
    }
}
