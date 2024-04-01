#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DnsCacheEntry)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DnsCacheEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub type DnsCacheEntry;
    #[wasm_bindgen(method, setter = "expiration")]
    fn expiration_shim(this: &DnsCacheEntry, val: f64);
    #[wasm_bindgen(method, setter = "family")]
    fn family_shim(this: &DnsCacheEntry, val: &str);
    #[wasm_bindgen(method, setter = "hostaddr")]
    fn hostaddr_shim(this: &DnsCacheEntry, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "hostname")]
    fn hostname_shim(this: &DnsCacheEntry, val: &str);
    #[wasm_bindgen(method, setter = "trr")]
    fn trr_shim(this: &DnsCacheEntry, val: bool);
}
impl DnsCacheEntry {
    #[doc = "Construct a new `DnsCacheEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `expiration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn expiration(&mut self, val: f64) -> &mut Self {
        self.expiration_shim(val);
        self
    }
    #[doc = "Change the `family` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn family(&mut self, val: &str) -> &mut Self {
        self.family_shim(val);
        self
    }
    #[doc = "Change the `hostaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn hostaddr(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.hostaddr_shim(val);
        self
    }
    #[doc = "Change the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn hostname(&mut self, val: &str) -> &mut Self {
        self.hostname_shim(val);
        self
    }
    #[doc = "Change the `trr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn trr(&mut self, val: bool) -> &mut Self {
        self.trr_shim(val);
        self
    }
}
impl Default for DnsCacheEntry {
    fn default() -> Self {
        Self::new()
    }
}
