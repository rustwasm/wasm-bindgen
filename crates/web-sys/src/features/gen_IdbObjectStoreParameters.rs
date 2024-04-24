#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IDBObjectStoreParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbObjectStoreParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    pub type IdbObjectStoreParameters;
    #[wasm_bindgen(method, getter = "autoIncrement")]
    fn auto_increment_shim(this: &IdbObjectStoreParameters) -> bool;
    #[wasm_bindgen(method, setter = "autoIncrement")]
    fn set_auto_increment_shim(this: &IdbObjectStoreParameters, val: bool);
    #[wasm_bindgen(method, getter = "keyPath")]
    fn key_path_shim(this: &IdbObjectStoreParameters) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "keyPath")]
    fn set_key_path_shim(this: &IdbObjectStoreParameters, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `IdbObjectStoreParameters` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
pub trait IdbObjectStoreParametersGetters {
    #[doc = "Get the `autoIncrement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    fn auto_increment(&self) -> bool;
    #[doc = "Get the `keyPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    fn key_path(&self) -> Option<&::wasm_bindgen::JsValue>;
}
impl IdbObjectStoreParametersGetters for IdbObjectStoreParameters {
    fn auto_increment(&self) -> bool {
        self.auto_increment_shim()
    }
    fn key_path(&self) -> Option<&::wasm_bindgen::JsValue> {
        self.key_path_shim()
    }
}
impl IdbObjectStoreParameters {
    #[doc = "Construct a new `IdbObjectStoreParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `autoIncrement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    pub fn auto_increment(&mut self, val: bool) -> &mut Self {
        self.set_auto_increment_shim(val);
        self
    }
    #[doc = "Change the `keyPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    pub fn key_path(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        self.set_key_path_shim(val.unwrap_or(&::wasm_bindgen::JsValue::NULL));
        self
    }
}
impl Default for IdbObjectStoreParameters {
    fn default() -> Self {
        Self::new()
    }
}
