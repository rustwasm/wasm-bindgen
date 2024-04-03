#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DNSLookupDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DnsLookupDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    pub type DnsLookupDict;
    #[wasm_bindgen(method, setter = "address")]
    fn address_shim(this: &DnsLookupDict, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "answer")]
    fn answer_shim(this: &DnsLookupDict, val: bool);
    #[wasm_bindgen(method, setter = "error")]
    fn error_shim(this: &DnsLookupDict, val: &str);
}
impl DnsLookupDict {
    #[doc = "Construct a new `DnsLookupDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `address` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    pub fn address(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.address_shim(val);
        self
    }
    #[doc = "Change the `answer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    pub fn answer(&mut self, val: bool) -> &mut Self {
        self.answer_shim(val);
        self
    }
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    pub fn error(&mut self, val: &str) -> &mut Self {
        self.error_shim(val);
        self
    }
}
impl Default for DnsLookupDict {
    fn default() -> Self {
        Self::new()
    }
}
