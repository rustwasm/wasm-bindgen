#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = NodeFilter)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NodeFilter` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeFilter`*"]
    pub type NodeFilter;
    #[wasm_bindgen(method, getter = "acceptNode")]
    fn accept_node_shim(this: &NodeFilter) -> &::js_sys::Function;
    #[wasm_bindgen(method, setter = "acceptNode")]
    fn set_accept_node_shim(this: &NodeFilter, val: &::js_sys::Function);
}
#[doc = "The trait to access properties on the `NodeFilter` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `NodeFilter`*"]
pub trait NodeFilterGetters {
    #[doc = "Get the `acceptNode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeFilter`*"]
    fn accept_node(&self) -> &::js_sys::Function;
}
impl NodeFilterGetters for NodeFilter {
    fn accept_node(&self) -> &::js_sys::Function {
        self.accept_node_shim()
    }
}
impl NodeFilter {
    #[doc = "Construct a new `NodeFilter`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeFilter`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `acceptNode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeFilter`*"]
    pub fn accept_node(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_accept_node_shim(val);
        self
    }
}
impl Default for NodeFilter {
    fn default() -> Self {
        Self::new()
    }
}
