#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BlockParsingOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BlockParsingOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlockParsingOptions`*"]
    pub type BlockParsingOptions;
    #[wasm_bindgen(method, getter = "blockScriptCreated")]
    fn block_script_created_shim(this: &BlockParsingOptions) -> bool;
    #[wasm_bindgen(method, setter = "blockScriptCreated")]
    fn set_block_script_created_shim(this: &BlockParsingOptions, val: bool);
}
#[doc = "The trait to access properties on the `BlockParsingOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BlockParsingOptions`*"]
pub trait BlockParsingOptionsGetters {
    #[doc = "Get the `blockScriptCreated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlockParsingOptions`*"]
    fn block_script_created(&self) -> bool;
}
impl BlockParsingOptionsGetters for BlockParsingOptions {
    fn block_script_created(&self) -> bool {
        self.block_script_created_shim()
    }
}
impl BlockParsingOptions {
    #[doc = "Construct a new `BlockParsingOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlockParsingOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `blockScriptCreated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlockParsingOptions`*"]
    pub fn block_script_created(&mut self, val: bool) -> &mut Self {
        self.set_block_script_created_shim(val);
        self
    }
}
impl Default for BlockParsingOptions {
    fn default() -> Self {
        Self::new()
    }
}
