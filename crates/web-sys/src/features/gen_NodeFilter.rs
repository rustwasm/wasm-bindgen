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
    #[wasm_bindgen(method, setter = "acceptNode")]
    fn accept_node_shim(this: &NodeFilter, val: &::js_sys::Function);
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
        self.accept_node_shim(val);
        self
    }
}
impl Default for NodeFilter {
    fn default() -> Self {
        Self::new()
    }
}
