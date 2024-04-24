#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DNSCacheDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DnsCacheDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
    pub type DnsCacheDict;
    #[wasm_bindgen(method, getter = "entries")]
    fn entries_shim(this: &DnsCacheDict) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "entries")]
    fn set_entries_shim(this: &DnsCacheDict, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `DnsCacheDict` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
pub trait DnsCacheDictGetters {
    #[doc = "Get the `entries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
    fn entries(&self) -> ::js_sys::Array;
}
impl DnsCacheDictGetters for DnsCacheDict {
    fn entries(&self) -> ::js_sys::Array {
        self.entries_shim()
    }
}
impl DnsCacheDict {
    #[doc = "Construct a new `DnsCacheDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `entries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
    pub fn entries(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_entries_shim(val);
        self
    }
}
impl Default for DnsCacheDict {
    fn default() -> Self {
        Self::new()
    }
}
