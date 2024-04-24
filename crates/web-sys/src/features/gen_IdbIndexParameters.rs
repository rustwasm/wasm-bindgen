#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IDBIndexParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbIndexParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    pub type IdbIndexParameters;
    #[wasm_bindgen(method, getter = "locale")]
    fn locale_shim(this: &IdbIndexParameters) -> Option<&str>;
    #[wasm_bindgen(method, setter = "locale")]
    fn set_locale_shim(this: &IdbIndexParameters, val: Option<&str>);
    #[wasm_bindgen(method, getter = "multiEntry")]
    fn multi_entry_shim(this: &IdbIndexParameters) -> bool;
    #[wasm_bindgen(method, setter = "multiEntry")]
    fn set_multi_entry_shim(this: &IdbIndexParameters, val: bool);
    #[wasm_bindgen(method, getter = "unique")]
    fn unique_shim(this: &IdbIndexParameters) -> bool;
    #[wasm_bindgen(method, setter = "unique")]
    fn set_unique_shim(this: &IdbIndexParameters, val: bool);
}
#[doc = "The trait to access properties on the `IdbIndexParameters` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
pub trait IdbIndexParametersGetters {
    #[doc = "Get the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    fn locale(&self) -> Option<&str>;
    #[doc = "Get the `multiEntry` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    fn multi_entry(&self) -> bool;
    #[doc = "Get the `unique` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    fn unique(&self) -> bool;
}
impl IdbIndexParametersGetters for IdbIndexParameters {
    fn locale(&self) -> Option<&str> {
        self.locale_shim()
    }
    fn multi_entry(&self) -> bool {
        self.multi_entry_shim()
    }
    fn unique(&self) -> bool {
        self.unique_shim()
    }
}
impl IdbIndexParameters {
    #[doc = "Construct a new `IdbIndexParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    pub fn locale(&mut self, val: Option<&str>) -> &mut Self {
        self.set_locale_shim(val);
        self
    }
    #[doc = "Change the `multiEntry` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    pub fn multi_entry(&mut self, val: bool) -> &mut Self {
        self.set_multi_entry_shim(val);
        self
    }
    #[doc = "Change the `unique` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    pub fn unique(&mut self, val: bool) -> &mut Self {
        self.set_unique_shim(val);
        self
    }
}
impl Default for IdbIndexParameters {
    fn default() -> Self {
        Self::new()
    }
}
