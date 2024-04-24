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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &PluginCrashedEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &PluginCrashedEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &PluginCrashedEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &PluginCrashedEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &PluginCrashedEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &PluginCrashedEventInit, val: bool);
    #[wasm_bindgen(method, getter = "browserDumpID")]
    fn browser_dump_id_shim(this: &PluginCrashedEventInit) -> Option<&str>;
    #[wasm_bindgen(method, setter = "browserDumpID")]
    fn set_browser_dump_id_shim(this: &PluginCrashedEventInit, val: Option<&str>);
    #[wasm_bindgen(method, getter = "gmpPlugin")]
    fn gmp_plugin_shim(this: &PluginCrashedEventInit) -> bool;
    #[wasm_bindgen(method, setter = "gmpPlugin")]
    fn set_gmp_plugin_shim(this: &PluginCrashedEventInit, val: bool);
    #[wasm_bindgen(method, getter = "pluginDumpID")]
    fn plugin_dump_id_shim(this: &PluginCrashedEventInit) -> &str;
    #[wasm_bindgen(method, setter = "pluginDumpID")]
    fn set_plugin_dump_id_shim(this: &PluginCrashedEventInit, val: &str);
    #[wasm_bindgen(method, getter = "pluginFilename")]
    fn plugin_filename_shim(this: &PluginCrashedEventInit) -> Option<&str>;
    #[wasm_bindgen(method, setter = "pluginFilename")]
    fn set_plugin_filename_shim(this: &PluginCrashedEventInit, val: Option<&str>);
    #[wasm_bindgen(method, getter = "pluginID")]
    fn plugin_id_shim(this: &PluginCrashedEventInit) -> u32;
    #[wasm_bindgen(method, setter = "pluginID")]
    fn set_plugin_id_shim(this: &PluginCrashedEventInit, val: u32);
    #[wasm_bindgen(method, getter = "pluginName")]
    fn plugin_name_shim(this: &PluginCrashedEventInit) -> &str;
    #[wasm_bindgen(method, setter = "pluginName")]
    fn set_plugin_name_shim(this: &PluginCrashedEventInit, val: &str);
    #[wasm_bindgen(method, getter = "submittedCrashReport")]
    fn submitted_crash_report_shim(this: &PluginCrashedEventInit) -> bool;
    #[wasm_bindgen(method, setter = "submittedCrashReport")]
    fn set_submitted_crash_report_shim(this: &PluginCrashedEventInit, val: bool);
}
#[doc = "The trait to access properties on the `PluginCrashedEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
pub trait PluginCrashedEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `browserDumpID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn browser_dump_id(&self) -> Option<&str>;
    #[doc = "Get the `gmpPlugin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn gmp_plugin(&self) -> bool;
    #[doc = "Get the `pluginDumpID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn plugin_dump_id(&self) -> &str;
    #[doc = "Get the `pluginFilename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn plugin_filename(&self) -> Option<&str>;
    #[doc = "Get the `pluginID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn plugin_id(&self) -> u32;
    #[doc = "Get the `pluginName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn plugin_name(&self) -> &str;
    #[doc = "Get the `submittedCrashReport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn submitted_crash_report(&self) -> bool;
}
impl PluginCrashedEventInitGetters for PluginCrashedEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn browser_dump_id(&self) -> Option<&str> {
        self.browser_dump_id_shim()
    }
    fn gmp_plugin(&self) -> bool {
        self.gmp_plugin_shim()
    }
    fn plugin_dump_id(&self) -> &str {
        self.plugin_dump_id_shim()
    }
    fn plugin_filename(&self) -> Option<&str> {
        self.plugin_filename_shim()
    }
    fn plugin_id(&self) -> u32 {
        self.plugin_id_shim()
    }
    fn plugin_name(&self) -> &str {
        self.plugin_name_shim()
    }
    fn submitted_crash_report(&self) -> bool {
        self.submitted_crash_report_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `browserDumpID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn browser_dump_id(&mut self, val: Option<&str>) -> &mut Self {
        self.set_browser_dump_id_shim(val);
        self
    }
    #[doc = "Change the `gmpPlugin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn gmp_plugin(&mut self, val: bool) -> &mut Self {
        self.set_gmp_plugin_shim(val);
        self
    }
    #[doc = "Change the `pluginDumpID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn plugin_dump_id(&mut self, val: &str) -> &mut Self {
        self.set_plugin_dump_id_shim(val);
        self
    }
    #[doc = "Change the `pluginFilename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn plugin_filename(&mut self, val: Option<&str>) -> &mut Self {
        self.set_plugin_filename_shim(val);
        self
    }
    #[doc = "Change the `pluginID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn plugin_id(&mut self, val: u32) -> &mut Self {
        self.set_plugin_id_shim(val);
        self
    }
    #[doc = "Change the `pluginName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn plugin_name(&mut self, val: &str) -> &mut Self {
        self.set_plugin_name_shim(val);
        self
    }
    #[doc = "Change the `submittedCrashReport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn submitted_crash_report(&mut self, val: bool) -> &mut Self {
        self.set_submitted_crash_report_shim(val);
        self
    }
}
impl Default for PluginCrashedEventInit {
    fn default() -> Self {
        Self::new()
    }
}
