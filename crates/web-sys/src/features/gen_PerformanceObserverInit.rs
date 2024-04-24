#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PerformanceObserverInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceObserverInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
    pub type PerformanceObserverInit;
    #[wasm_bindgen(method, getter = "buffered")]
    fn buffered_shim(this: &PerformanceObserverInit) -> bool;
    #[wasm_bindgen(method, setter = "buffered")]
    fn set_buffered_shim(this: &PerformanceObserverInit, val: bool);
    #[wasm_bindgen(method, getter = "entryTypes")]
    fn entry_types_shim(this: &PerformanceObserverInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "entryTypes")]
    fn set_entry_types_shim(this: &PerformanceObserverInit, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `PerformanceObserverInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
pub trait PerformanceObserverInitGetters {
    #[doc = "Get the `buffered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
    fn buffered(&self) -> bool;
    #[doc = "Get the `entryTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
    fn entry_types(&self) -> &::wasm_bindgen::JsValue;
}
impl PerformanceObserverInitGetters for PerformanceObserverInit {
    fn buffered(&self) -> bool {
        self.buffered_shim()
    }
    fn entry_types(&self) -> &::wasm_bindgen::JsValue {
        self.entry_types_shim()
    }
}
impl PerformanceObserverInit {
    #[doc = "Construct a new `PerformanceObserverInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
    pub fn new(entry_types: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.entry_types(entry_types);
        ret
    }
    #[doc = "Change the `buffered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
    pub fn buffered(&mut self, val: bool) -> &mut Self {
        self.set_buffered_shim(val);
        self
    }
    #[doc = "Change the `entryTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
    pub fn entry_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_entry_types_shim(val);
        self
    }
}
