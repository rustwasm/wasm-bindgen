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
    #[wasm_bindgen(method, getter = "locale")]
    fn locale_shim(this: &DisplayNameResult) -> &str;
    #[wasm_bindgen(method, setter = "locale")]
    fn set_locale_shim(this: &DisplayNameResult, val: &str);
    #[wasm_bindgen(method, getter = "style")]
    fn style_shim(this: &DisplayNameResult) -> &str;
    #[wasm_bindgen(method, setter = "style")]
    fn set_style_shim(this: &DisplayNameResult, val: &str);
}
#[doc = "The trait to access properties on the `DisplayNameResult` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
pub trait DisplayNameResultGetters {
    #[doc = "Get the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    fn locale(&self) -> &str;
    #[doc = "Get the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    fn style(&self) -> &str;
}
impl DisplayNameResultGetters for DisplayNameResult {
    fn locale(&self) -> &str {
        self.locale_shim()
    }
    fn style(&self) -> &str {
        self.style_shim()
    }
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
        self.set_locale_shim(val);
        self
    }
    #[doc = "Change the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    pub fn style(&mut self, val: &str) -> &mut Self {
        self.set_style_shim(val);
        self
    }
}
impl Default for DisplayNameResult {
    fn default() -> Self {
        Self::new()
    }
}
