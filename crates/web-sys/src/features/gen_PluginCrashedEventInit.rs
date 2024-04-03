#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PluginCrashedEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PluginCrashedEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub type PluginCrashedEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &PluginCrashedEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &PluginCrashedEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &PluginCrashedEventInit, val: bool);
    #[wasm_bindgen(method, setter = "browserDumpID")]
    fn browser_dump_id_shim(this: &PluginCrashedEventInit, val: Option<&str>);
    #[wasm_bindgen(method, setter = "gmpPlugin")]
    fn gmp_plugin_shim(this: &PluginCrashedEventInit, val: bool);
    #[wasm_bindgen(method, setter = "pluginDumpID")]
    fn plugin_dump_id_shim(this: &PluginCrashedEventInit, val: &str);
    #[wasm_bindgen(method, setter = "pluginFilename")]
    fn plugin_filename_shim(this: &PluginCrashedEventInit, val: Option<&str>);
    #[wasm_bindgen(method, setter = "pluginID")]
    fn plugin_id_shim(this: &PluginCrashedEventInit, val: u32);
    #[wasm_bindgen(method, setter = "pluginName")]
    fn plugin_name_shim(this: &PluginCrashedEventInit, val: &str);
    #[wasm_bindgen(method, setter = "submittedCrashReport")]
    fn submitted_crash_report_shim(this: &PluginCrashedEventInit, val: bool);
}
impl PluginCrashedEventInit {
    #[doc = "Construct a new `PluginCrashedEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `browserDumpID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn browser_dump_id(&mut self, val: Option<&str>) -> &mut Self {
        self.browser_dump_id_shim(val);
        self
    }
    #[doc = "Change the `gmpPlugin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn gmp_plugin(&mut self, val: bool) -> &mut Self {
        self.gmp_plugin_shim(val);
        self
    }
    #[doc = "Change the `pluginDumpID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn plugin_dump_id(&mut self, val: &str) -> &mut Self {
        self.plugin_dump_id_shim(val);
        self
    }
    #[doc = "Change the `pluginFilename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn plugin_filename(&mut self, val: Option<&str>) -> &mut Self {
        self.plugin_filename_shim(val);
        self
    }
    #[doc = "Change the `pluginID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn plugin_id(&mut self, val: u32) -> &mut Self {
        self.plugin_id_shim(val);
        self
    }
    #[doc = "Change the `pluginName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn plugin_name(&mut self, val: &str) -> &mut Self {
        self.plugin_name_shim(val);
        self
    }
    #[doc = "Change the `submittedCrashReport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn submitted_crash_report(&mut self, val: bool) -> &mut Self {
        self.submitted_crash_report_shim(val);
        self
    }
}
impl Default for PluginCrashedEventInit {
    fn default() -> Self {
        Self::new()
    }
}
