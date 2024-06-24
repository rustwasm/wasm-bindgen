#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FocusOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FocusOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`*"]
    pub type FocusOptions;
    #[wasm_bindgen(method, setter = "focusVisible")]
    fn focus_visible_shim(this: &FocusOptions, val: bool);
    #[wasm_bindgen(method, setter = "preventScroll")]
    fn prevent_scroll_shim(this: &FocusOptions, val: bool);
}
impl FocusOptions {
    #[doc = "Construct a new `FocusOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `focusVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`*"]
    pub fn focus_visible(&mut self, val: bool) -> &mut Self {
        self.focus_visible_shim(val);
        self
    }
    #[doc = "Change the `preventScroll` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`*"]
    pub fn prevent_scroll(&mut self, val: bool) -> &mut Self {
        self.prevent_scroll_shim(val);
        self
    }
}
impl Default for FocusOptions {
    fn default() -> Self {
        Self::new()
    }
}
