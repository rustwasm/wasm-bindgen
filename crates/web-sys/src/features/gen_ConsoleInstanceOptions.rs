#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConsoleInstanceOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleInstanceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    pub type ConsoleInstanceOptions;
    #[wasm_bindgen(method, getter = "consoleID")]
    fn console_id_shim(this: &ConsoleInstanceOptions) -> &str;
    #[wasm_bindgen(method, setter = "consoleID")]
    fn set_console_id_shim(this: &ConsoleInstanceOptions, val: &str);
    #[wasm_bindgen(method, getter = "dump")]
    fn dump_shim(this: &ConsoleInstanceOptions) -> &::js_sys::Function;
    #[wasm_bindgen(method, setter = "dump")]
    fn set_dump_shim(this: &ConsoleInstanceOptions, val: &::js_sys::Function);
    #[wasm_bindgen(method, getter = "innerID")]
    fn inner_id_shim(this: &ConsoleInstanceOptions) -> &str;
    #[wasm_bindgen(method, setter = "innerID")]
    fn set_inner_id_shim(this: &ConsoleInstanceOptions, val: &str);
    #[cfg(feature = "ConsoleLogLevel")]
    #[wasm_bindgen(method, getter = "maxLogLevel")]
    fn max_log_level_shim(this: &ConsoleInstanceOptions) -> ConsoleLogLevel;
    #[cfg(feature = "ConsoleLogLevel")]
    #[wasm_bindgen(method, setter = "maxLogLevel")]
    fn set_max_log_level_shim(this: &ConsoleInstanceOptions, val: ConsoleLogLevel);
    #[wasm_bindgen(method, getter = "maxLogLevelPref")]
    fn max_log_level_pref_shim(this: &ConsoleInstanceOptions) -> &str;
    #[wasm_bindgen(method, setter = "maxLogLevelPref")]
    fn set_max_log_level_pref_shim(this: &ConsoleInstanceOptions, val: &str);
    #[wasm_bindgen(method, getter = "prefix")]
    fn prefix_shim(this: &ConsoleInstanceOptions) -> &str;
    #[wasm_bindgen(method, setter = "prefix")]
    fn set_prefix_shim(this: &ConsoleInstanceOptions, val: &str);
}
#[doc = "The trait to access properties on the `ConsoleInstanceOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
pub trait ConsoleInstanceOptionsGetters {
    #[doc = "Get the `consoleID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    fn console_id(&self) -> &str;
    #[doc = "Get the `dump` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    fn dump(&self) -> &::js_sys::Function;
    #[doc = "Get the `innerID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    fn inner_id(&self) -> &str;
    #[cfg(feature = "ConsoleLogLevel")]
    #[doc = "Get the `maxLogLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`, `ConsoleLogLevel`*"]
    fn max_log_level(&self) -> ConsoleLogLevel;
    #[doc = "Get the `maxLogLevelPref` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    fn max_log_level_pref(&self) -> &str;
    #[doc = "Get the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    fn prefix(&self) -> &str;
}
impl ConsoleInstanceOptionsGetters for ConsoleInstanceOptions {
    fn console_id(&self) -> &str {
        self.console_id_shim()
    }
    fn dump(&self) -> &::js_sys::Function {
        self.dump_shim()
    }
    fn inner_id(&self) -> &str {
        self.inner_id_shim()
    }
    #[cfg(feature = "ConsoleLogLevel")]
    fn max_log_level(&self) -> ConsoleLogLevel {
        self.max_log_level_shim()
    }
    fn max_log_level_pref(&self) -> &str {
        self.max_log_level_pref_shim()
    }
    fn prefix(&self) -> &str {
        self.prefix_shim()
    }
}
impl ConsoleInstanceOptions {
    #[doc = "Construct a new `ConsoleInstanceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `consoleID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    pub fn console_id(&mut self, val: &str) -> &mut Self {
        self.set_console_id_shim(val);
        self
    }
    #[doc = "Change the `dump` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    pub fn dump(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_dump_shim(val);
        self
    }
    #[doc = "Change the `innerID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    pub fn inner_id(&mut self, val: &str) -> &mut Self {
        self.set_inner_id_shim(val);
        self
    }
    #[cfg(feature = "ConsoleLogLevel")]
    #[doc = "Change the `maxLogLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`, `ConsoleLogLevel`*"]
    pub fn max_log_level(&mut self, val: ConsoleLogLevel) -> &mut Self {
        self.set_max_log_level_shim(val);
        self
    }
    #[doc = "Change the `maxLogLevelPref` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    pub fn max_log_level_pref(&mut self, val: &str) -> &mut Self {
        self.set_max_log_level_pref_shim(val);
        self
    }
    #[doc = "Change the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    pub fn prefix(&mut self, val: &str) -> &mut Self {
        self.set_prefix_shim(val);
        self
    }
}
impl Default for ConsoleInstanceOptions {
    fn default() -> Self {
        Self::new()
    }
}
