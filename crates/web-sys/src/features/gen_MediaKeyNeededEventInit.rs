#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaKeyNeededEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeyNeededEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    pub type MediaKeyNeededEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &MediaKeyNeededEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &MediaKeyNeededEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &MediaKeyNeededEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &MediaKeyNeededEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &MediaKeyNeededEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &MediaKeyNeededEventInit, val: bool);
    #[wasm_bindgen(method, getter = "initData")]
    fn init_data_shim(this: &MediaKeyNeededEventInit) -> Option<&::js_sys::ArrayBuffer>;
    #[wasm_bindgen(method, setter = "initData")]
    fn set_init_data_shim(this: &MediaKeyNeededEventInit, val: Option<&::js_sys::ArrayBuffer>);
    #[wasm_bindgen(method, getter = "initDataType")]
    fn init_data_type_shim(this: &MediaKeyNeededEventInit) -> &str;
    #[wasm_bindgen(method, setter = "initDataType")]
    fn set_init_data_type_shim(this: &MediaKeyNeededEventInit, val: &str);
}
#[doc = "The trait to access properties on the `MediaKeyNeededEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
pub trait MediaKeyNeededEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `initData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    fn init_data(&self) -> Option<&::js_sys::ArrayBuffer>;
    #[doc = "Get the `initDataType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    fn init_data_type(&self) -> &str;
}
impl MediaKeyNeededEventInitGetters for MediaKeyNeededEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn init_data(&self) -> Option<&::js_sys::ArrayBuffer> {
        self.init_data_shim()
    }
    fn init_data_type(&self) -> &str {
        self.init_data_type_shim()
    }
}
impl MediaKeyNeededEventInit {
    #[doc = "Construct a new `MediaKeyNeededEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `initData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    pub fn init_data(&mut self, val: Option<&::js_sys::ArrayBuffer>) -> &mut Self {
        self.set_init_data_shim(val);
        self
    }
    #[doc = "Change the `initDataType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    pub fn init_data_type(&mut self, val: &str) -> &mut Self {
        self.set_init_data_type_shim(val);
        self
    }
}
impl Default for MediaKeyNeededEventInit {
    fn default() -> Self {
        Self::new()
    }
}
