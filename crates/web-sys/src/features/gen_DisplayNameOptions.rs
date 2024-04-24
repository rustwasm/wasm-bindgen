#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DisplayNameOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DisplayNameOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    pub type DisplayNameOptions;
    #[wasm_bindgen(method, getter = "keys")]
    fn keys_shim(this: &DisplayNameOptions) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "keys")]
    fn set_keys_shim(this: &DisplayNameOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "style")]
    fn style_shim(this: &DisplayNameOptions) -> String;
    #[wasm_bindgen(method, setter = "style")]
    fn set_style_shim(this: &DisplayNameOptions, val: &str);
}
#[doc = "The trait to access properties on the `DisplayNameOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
pub trait DisplayNameOptionsGetters {
    #[doc = "Get the `keys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    fn keys(&self) -> ::js_sys::Array;
    #[doc = "Get the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    fn style(&self) -> String;
}
impl DisplayNameOptionsGetters for DisplayNameOptions {
    fn keys(&self) -> ::js_sys::Array {
        self.keys_shim()
    }
    fn style(&self) -> String {
        self.style_shim()
    }
}
impl DisplayNameOptions {
    #[doc = "Construct a new `DisplayNameOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `keys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    pub fn keys(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_keys_shim(val);
        self
    }
    #[doc = "Change the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    pub fn style(&mut self, val: &str) -> &mut Self {
        self.set_style_shim(val);
        self
    }
}
impl Default for DisplayNameOptions {
    fn default() -> Self {
        Self::new()
    }
}
