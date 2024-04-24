#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AssignedNodesOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AssignedNodesOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
    pub type AssignedNodesOptions;
    #[wasm_bindgen(method, getter = "flatten")]
    fn flatten_shim(this: &AssignedNodesOptions) -> bool;
    #[wasm_bindgen(method, setter = "flatten")]
    fn set_flatten_shim(this: &AssignedNodesOptions, val: bool);
}
#[doc = "The trait to access properties on the `AssignedNodesOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
pub trait AssignedNodesOptionsGetters {
    #[doc = "Get the `flatten` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
    fn flatten(&self) -> bool;
}
impl AssignedNodesOptionsGetters for AssignedNodesOptions {
    fn flatten(&self) -> bool {
        self.flatten_shim()
    }
}
impl AssignedNodesOptions {
    #[doc = "Construct a new `AssignedNodesOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `flatten` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
    pub fn flatten(&mut self, val: bool) -> &mut Self {
        self.set_flatten_shim(val);
        self
    }
}
impl Default for AssignedNodesOptions {
    fn default() -> Self {
        Self::new()
    }
}
