#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BrowserElementExecuteScriptOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BrowserElementExecuteScriptOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
    pub type BrowserElementExecuteScriptOptions;
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
    #[wasm_bindgen(method, getter = "origin")]
    pub fn get_origin(this: &BrowserElementExecuteScriptOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "origin")]
    fn set_origin(this: &BrowserElementExecuteScriptOptions, val: Option<&str>);
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &BrowserElementExecuteScriptOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "url")]
    fn set_url(this: &BrowserElementExecuteScriptOptions, val: Option<&str>);
}
impl BrowserElementExecuteScriptOptions {
    #[doc = "Construct a new `BrowserElementExecuteScriptOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
    pub fn origin(&mut self, val: Option<&str>) -> &mut Self {
        self.set_origin(val);
        self
    }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
    pub fn url(&mut self, val: Option<&str>) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for BrowserElementExecuteScriptOptions {
    fn default() -> Self {
        Self::new()
    }
}
