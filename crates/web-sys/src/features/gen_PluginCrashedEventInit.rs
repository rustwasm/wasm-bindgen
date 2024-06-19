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
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &PluginCrashedEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &PluginCrashedEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &PluginCrashedEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &PluginCrashedEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &PluginCrashedEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &PluginCrashedEventInit, val: bool);
    #[doc = "Get the `browserDumpID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    #[wasm_bindgen(method, getter = "browserDumpID")]
    pub fn get_browser_dump_id(this: &PluginCrashedEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "browserDumpID")]
    fn set_browser_dump_id(this: &PluginCrashedEventInit, val: Option<&str>);
    #[doc = "Get the `gmpPlugin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    #[wasm_bindgen(method, getter = "gmpPlugin")]
    pub fn get_gmp_plugin(this: &PluginCrashedEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "gmpPlugin")]
    fn set_gmp_plugin(this: &PluginCrashedEventInit, val: bool);
    #[doc = "Get the `pluginDumpID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    #[wasm_bindgen(method, getter = "pluginDumpID")]
    pub fn get_plugin_dump_id(this: &PluginCrashedEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "pluginDumpID")]
    fn set_plugin_dump_id(this: &PluginCrashedEventInit, val: &str);
    #[doc = "Get the `pluginFilename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    #[wasm_bindgen(method, getter = "pluginFilename")]
    pub fn get_plugin_filename(this: &PluginCrashedEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "pluginFilename")]
    fn set_plugin_filename(this: &PluginCrashedEventInit, val: Option<&str>);
    #[doc = "Get the `pluginID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    #[wasm_bindgen(method, getter = "pluginID")]
    pub fn get_plugin_id(this: &PluginCrashedEventInit) -> Option<u32>;
    #[wasm_bindgen(method, setter = "pluginID")]
    fn set_plugin_id(this: &PluginCrashedEventInit, val: u32);
    #[doc = "Get the `pluginName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    #[wasm_bindgen(method, getter = "pluginName")]
    pub fn get_plugin_name(this: &PluginCrashedEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "pluginName")]
    fn set_plugin_name(this: &PluginCrashedEventInit, val: &str);
    #[doc = "Get the `submittedCrashReport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    #[wasm_bindgen(method, getter = "submittedCrashReport")]
    pub fn get_submitted_crash_report(this: &PluginCrashedEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "submittedCrashReport")]
    fn set_submitted_crash_report(this: &PluginCrashedEventInit, val: bool);
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
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `browserDumpID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn browser_dump_id(&mut self, val: Option<&str>) -> &mut Self {
        self.set_browser_dump_id(val);
        self
    }
    #[doc = "Change the `gmpPlugin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn gmp_plugin(&mut self, val: bool) -> &mut Self {
        self.set_gmp_plugin(val);
        self
    }
    #[doc = "Change the `pluginDumpID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn plugin_dump_id(&mut self, val: &str) -> &mut Self {
        self.set_plugin_dump_id(val);
        self
    }
    #[doc = "Change the `pluginFilename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn plugin_filename(&mut self, val: Option<&str>) -> &mut Self {
        self.set_plugin_filename(val);
        self
    }
    #[doc = "Change the `pluginID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn plugin_id(&mut self, val: u32) -> &mut Self {
        self.set_plugin_id(val);
        self
    }
    #[doc = "Change the `pluginName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn plugin_name(&mut self, val: &str) -> &mut Self {
        self.set_plugin_name(val);
        self
    }
    #[doc = "Change the `submittedCrashReport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn submitted_crash_report(&mut self, val: bool) -> &mut Self {
        self.set_submitted_crash_report(val);
        self
    }
}
impl Default for PluginCrashedEventInit {
    fn default() -> Self {
        Self::new()
    }
}
