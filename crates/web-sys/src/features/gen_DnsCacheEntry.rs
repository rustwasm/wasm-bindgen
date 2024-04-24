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
    #[wasm_bindgen(method, getter = "expiration")]
    fn expiration_shim(this: &DnsCacheEntry) -> f64;
    #[wasm_bindgen(method, setter = "expiration")]
    fn set_expiration_shim(this: &DnsCacheEntry, val: f64);
    #[wasm_bindgen(method, getter = "family")]
    fn family_shim(this: &DnsCacheEntry) -> &str;
    #[wasm_bindgen(method, setter = "family")]
    fn set_family_shim(this: &DnsCacheEntry, val: &str);
    #[wasm_bindgen(method, getter = "hostaddr")]
    fn hostaddr_shim(this: &DnsCacheEntry) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "hostaddr")]
    fn set_hostaddr_shim(this: &DnsCacheEntry, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "hostname")]
    fn hostname_shim(this: &DnsCacheEntry) -> &str;
    #[wasm_bindgen(method, setter = "hostname")]
    fn set_hostname_shim(this: &DnsCacheEntry, val: &str);
    #[wasm_bindgen(method, getter = "trr")]
    fn trr_shim(this: &DnsCacheEntry) -> bool;
    #[wasm_bindgen(method, setter = "trr")]
    fn set_trr_shim(this: &DnsCacheEntry, val: bool);
}
#[doc = "The trait to access properties on the `DnsCacheEntry` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
pub trait DnsCacheEntryGetters {
    #[doc = "Get the `expiration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    fn expiration(&self) -> f64;
    #[doc = "Get the `family` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    fn family(&self) -> &str;
    #[doc = "Get the `hostaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    fn hostaddr(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    fn hostname(&self) -> &str;
    #[doc = "Get the `trr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    fn trr(&self) -> bool;
}
impl DnsCacheEntryGetters for DnsCacheEntry {
    fn expiration(&self) -> f64 {
        self.expiration_shim()
    }
    fn family(&self) -> &str {
        self.family_shim()
    }
    fn hostaddr(&self) -> &::wasm_bindgen::JsValue {
        self.hostaddr_shim()
    }
    fn hostname(&self) -> &str {
        self.hostname_shim()
    }
    fn trr(&self) -> bool {
        self.trr_shim()
    }
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
        self.set_expiration_shim(val);
        self
    }
    #[doc = "Change the `family` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn family(&mut self, val: &str) -> &mut Self {
        self.set_family_shim(val);
        self
    }
    #[doc = "Change the `hostaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn hostaddr(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_hostaddr_shim(val);
        self
    }
    #[doc = "Change the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn hostname(&mut self, val: &str) -> &mut Self {
        self.set_hostname_shim(val);
        self
    }
    #[doc = "Change the `trr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn trr(&mut self, val: bool) -> &mut Self {
        self.set_trr_shim(val);
        self
    }
}
impl Default for DnsCacheEntry {
    fn default() -> Self {
        Self::new()
    }
}
