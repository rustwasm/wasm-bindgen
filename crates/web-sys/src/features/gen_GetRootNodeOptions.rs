#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GetRootNodeOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GetRootNodeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetRootNodeOptions`*"]
    pub type GetRootNodeOptions;
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &GetRootNodeOptions, val: bool);
}
impl GetRootNodeOptions {
    #[doc = "Construct a new `GetRootNodeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetRootNodeOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetRootNodeOptions`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
}
impl Default for GetRootNodeOptions {
    fn default() -> Self {
        Self::new()
    }
}
