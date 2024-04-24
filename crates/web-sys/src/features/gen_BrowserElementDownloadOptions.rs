#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BrowserElementDownloadOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BrowserElementDownloadOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    pub type BrowserElementDownloadOptions;
    #[wasm_bindgen(method, getter = "filename")]
    fn filename_shim(this: &BrowserElementDownloadOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "filename")]
    fn set_filename_shim(this: &BrowserElementDownloadOptions, val: Option<&str>);
    #[wasm_bindgen(method, getter = "referrer")]
    fn referrer_shim(this: &BrowserElementDownloadOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "referrer")]
    fn set_referrer_shim(this: &BrowserElementDownloadOptions, val: Option<&str>);
}
#[doc = "The trait to access properties on the `BrowserElementDownloadOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
pub trait BrowserElementDownloadOptionsGetters {
    #[doc = "Get the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    fn filename(&self) -> Option<String>;
    #[doc = "Get the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    fn referrer(&self) -> Option<String>;
}
impl BrowserElementDownloadOptionsGetters for BrowserElementDownloadOptions {
    fn filename(&self) -> Option<String> {
        self.filename_shim()
    }
    fn referrer(&self) -> Option<String> {
        self.referrer_shim()
    }
}
impl BrowserElementDownloadOptions {
    #[doc = "Construct a new `BrowserElementDownloadOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    pub fn filename(&mut self, val: Option<&str>) -> &mut Self {
        self.set_filename_shim(val);
        self
    }
    #[doc = "Change the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    pub fn referrer(&mut self, val: Option<&str>) -> &mut Self {
        self.set_referrer_shim(val);
        self
    }
}
impl Default for BrowserElementDownloadOptions {
    fn default() -> Self {
        Self::new()
    }
}
