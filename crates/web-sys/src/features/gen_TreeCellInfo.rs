#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TreeCellInfo)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TreeCellInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    pub type TreeCellInfo;
    #[wasm_bindgen(method, getter = "childElt")]
    fn child_elt_shim(this: &TreeCellInfo) -> &str;
    #[wasm_bindgen(method, setter = "childElt")]
    fn set_child_elt_shim(this: &TreeCellInfo, val: &str);
    #[wasm_bindgen(method, getter = "row")]
    fn row_shim(this: &TreeCellInfo) -> i32;
    #[wasm_bindgen(method, setter = "row")]
    fn set_row_shim(this: &TreeCellInfo, val: i32);
}
#[doc = "The trait to access properties on the `TreeCellInfo` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
pub trait TreeCellInfoGetters {
    #[doc = "Get the `childElt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    fn child_elt(&self) -> &str;
    #[doc = "Get the `row` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    fn row(&self) -> i32;
}
impl TreeCellInfoGetters for TreeCellInfo {
    fn child_elt(&self) -> &str {
        self.child_elt_shim()
    }
    fn row(&self) -> i32 {
        self.row_shim()
    }
}
impl TreeCellInfo {
    #[doc = "Construct a new `TreeCellInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `childElt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    pub fn child_elt(&mut self, val: &str) -> &mut Self {
        self.set_child_elt_shim(val);
        self
    }
    #[doc = "Change the `row` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    pub fn row(&mut self, val: i32) -> &mut Self {
        self.set_row_shim(val);
        self
    }
}
impl Default for TreeCellInfo {
    fn default() -> Self {
        Self::new()
    }
}
