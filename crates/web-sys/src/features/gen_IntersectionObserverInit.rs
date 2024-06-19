#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IntersectionObserverInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IntersectionObserverInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
    pub type IntersectionObserverInit;
    #[cfg(feature = "Element")]
    #[doc = "Get the `root` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserverInit`*"]
    #[wasm_bindgen(method, getter = "root")]
    pub fn get_root(this: &IntersectionObserverInit) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, setter = "root")]
    fn set_root(this: &IntersectionObserverInit, val: Option<&Element>);
    #[doc = "Get the `rootMargin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
    #[wasm_bindgen(method, getter = "rootMargin")]
    pub fn get_root_margin(this: &IntersectionObserverInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "rootMargin")]
    fn set_root_margin(this: &IntersectionObserverInit, val: &str);
    #[doc = "Get the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
    #[wasm_bindgen(method, getter = "threshold")]
    pub fn get_threshold(this: &IntersectionObserverInit) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "threshold")]
    fn set_threshold(this: &IntersectionObserverInit, val: &::wasm_bindgen::JsValue);
}
impl IntersectionObserverInit {
    #[doc = "Construct a new `IntersectionObserverInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "Element")]
    #[doc = "Change the `root` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserverInit`*"]
    pub fn root(&mut self, val: Option<&Element>) -> &mut Self {
        self.set_root(val);
        self
    }
    #[doc = "Change the `rootMargin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
    pub fn root_margin(&mut self, val: &str) -> &mut Self {
        self.set_root_margin(val);
        self
    }
    #[doc = "Change the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
    pub fn threshold(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_threshold(val);
        self
    }
}
impl Default for IntersectionObserverInit {
    fn default() -> Self {
        Self::new()
    }
}
