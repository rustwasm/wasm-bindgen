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
    #[wasm_bindgen(method, getter = "origin")]
    fn origin_shim(this: &BrowserElementExecuteScriptOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "origin")]
    fn set_origin_shim(this: &BrowserElementExecuteScriptOptions, val: Option<&str>);
    #[wasm_bindgen(method, getter = "url")]
    fn url_shim(this: &BrowserElementExecuteScriptOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "url")]
    fn set_url_shim(this: &BrowserElementExecuteScriptOptions, val: Option<&str>);
}
#[doc = "The trait to access properties on the `BrowserElementExecuteScriptOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
pub trait BrowserElementExecuteScriptOptionsGetters {
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
    fn origin(&self) -> Option<String>;
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
    fn url(&self) -> Option<String>;
}
impl BrowserElementExecuteScriptOptionsGetters for BrowserElementExecuteScriptOptions {
    fn origin(&self) -> Option<String> {
        self.origin_shim()
    }
    fn url(&self) -> Option<String> {
        self.url_shim()
    }
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
        self.set_origin_shim(val);
        self
    }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
    pub fn url(&mut self, val: Option<&str>) -> &mut Self {
        self.set_url_shim(val);
        self
    }
}
impl Default for BrowserElementExecuteScriptOptions {
    fn default() -> Self {
        Self::new()
    }
}
