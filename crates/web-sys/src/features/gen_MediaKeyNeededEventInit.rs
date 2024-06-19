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
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &MediaKeyNeededEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &MediaKeyNeededEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &MediaKeyNeededEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &MediaKeyNeededEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &MediaKeyNeededEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &MediaKeyNeededEventInit, val: bool);
    #[doc = "Get the `initData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    #[wasm_bindgen(method, getter = "initData")]
    pub fn get_init_data(this: &MediaKeyNeededEventInit) -> Option<::js_sys::ArrayBuffer>;
    #[wasm_bindgen(method, setter = "initData")]
    fn set_init_data(this: &MediaKeyNeededEventInit, val: Option<&::js_sys::ArrayBuffer>);
    #[doc = "Get the `initDataType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    #[wasm_bindgen(method, getter = "initDataType")]
    pub fn get_init_data_type(this: &MediaKeyNeededEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "initDataType")]
    fn set_init_data_type(this: &MediaKeyNeededEventInit, val: &str);
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
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `initData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    pub fn init_data(&mut self, val: Option<&::js_sys::ArrayBuffer>) -> &mut Self {
        self.set_init_data(val);
        self
    }
    #[doc = "Change the `initDataType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    pub fn init_data_type(&mut self, val: &str) -> &mut Self {
        self.set_init_data_type(val);
        self
    }
}
impl Default for MediaKeyNeededEventInit {
    fn default() -> Self {
        Self::new()
    }
}
