#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DisplayNameResult)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DisplayNameResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    pub type DisplayNameResult;
    #[wasm_bindgen(method, setter = "locale")]
    fn locale_shim(this: &DisplayNameResult, val: &str);
    #[wasm_bindgen(method, setter = "style")]
    fn style_shim(this: &DisplayNameResult, val: &str);
}
impl DisplayNameResult {
    #[doc = "Construct a new `DisplayNameResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    pub fn locale(&mut self, val: &str) -> &mut Self {
        self.locale_shim(val);
        self
    }
    #[doc = "Change the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    pub fn style(&mut self, val: &str) -> &mut Self {
        self.style_shim(val);
        self
    }
}
impl Default for DisplayNameResult {
    fn default() -> Self {
        Self::new()
    }
}
