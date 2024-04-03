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
    #[wasm_bindgen(method, setter = "consoleID")]
    fn console_id_shim(this: &ConsoleInstanceOptions, val: &str);
    #[wasm_bindgen(method, setter = "dump")]
    fn dump_shim(this: &ConsoleInstanceOptions, val: &::js_sys::Function);
    #[wasm_bindgen(method, setter = "innerID")]
    fn inner_id_shim(this: &ConsoleInstanceOptions, val: &str);
    #[cfg(feature = "ConsoleLogLevel")]
    #[wasm_bindgen(method, setter = "maxLogLevel")]
    fn max_log_level_shim(this: &ConsoleInstanceOptions, val: ConsoleLogLevel);
    #[wasm_bindgen(method, setter = "maxLogLevelPref")]
    fn max_log_level_pref_shim(this: &ConsoleInstanceOptions, val: &str);
    #[wasm_bindgen(method, setter = "prefix")]
    fn prefix_shim(this: &ConsoleInstanceOptions, val: &str);
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
        self.console_id_shim(val);
        self
    }
    #[doc = "Change the `dump` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    pub fn dump(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.dump_shim(val);
        self
    }
    #[doc = "Change the `innerID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    pub fn inner_id(&mut self, val: &str) -> &mut Self {
        self.inner_id_shim(val);
        self
    }
    #[cfg(feature = "ConsoleLogLevel")]
    #[doc = "Change the `maxLogLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`, `ConsoleLogLevel`*"]
    pub fn max_log_level(&mut self, val: ConsoleLogLevel) -> &mut Self {
        self.max_log_level_shim(val);
        self
    }
    #[doc = "Change the `maxLogLevelPref` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    pub fn max_log_level_pref(&mut self, val: &str) -> &mut Self {
        self.max_log_level_pref_shim(val);
        self
    }
    #[doc = "Change the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    pub fn prefix(&mut self, val: &str) -> &mut Self {
        self.prefix_shim(val);
        self
    }
}
impl Default for ConsoleInstanceOptions {
    fn default() -> Self {
        Self::new()
    }
}
