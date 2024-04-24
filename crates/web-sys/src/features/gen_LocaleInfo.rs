#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = LocaleInfo)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `LocaleInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    pub type LocaleInfo;
    #[wasm_bindgen(method, getter = "direction")]
    fn direction_shim(this: &LocaleInfo) -> String;
    #[wasm_bindgen(method, setter = "direction")]
    fn set_direction_shim(this: &LocaleInfo, val: &str);
    #[wasm_bindgen(method, getter = "locale")]
    fn locale_shim(this: &LocaleInfo) -> String;
    #[wasm_bindgen(method, setter = "locale")]
    fn set_locale_shim(this: &LocaleInfo, val: &str);
}
#[doc = "The trait to access properties on the `LocaleInfo` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
pub trait LocaleInfoGetters {
    #[doc = "Get the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    fn direction(&self) -> String;
    #[doc = "Get the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    fn locale(&self) -> String;
}
impl LocaleInfoGetters for LocaleInfo {
    fn direction(&self) -> String {
        self.direction_shim()
    }
    fn locale(&self) -> String {
        self.locale_shim()
    }
}
impl LocaleInfo {
    #[doc = "Construct a new `LocaleInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    pub fn direction(&mut self, val: &str) -> &mut Self {
        self.set_direction_shim(val);
        self
    }
    #[doc = "Change the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    pub fn locale(&mut self, val: &str) -> &mut Self {
        self.set_locale_shim(val);
        self
    }
}
impl Default for LocaleInfo {
    fn default() -> Self {
        Self::new()
    }
}
