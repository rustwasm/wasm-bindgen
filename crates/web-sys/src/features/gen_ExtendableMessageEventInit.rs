#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ExtendableMessageEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtendableMessageEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    pub type ExtendableMessageEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &ExtendableMessageEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &ExtendableMessageEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &ExtendableMessageEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &ExtendableMessageEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &ExtendableMessageEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &ExtendableMessageEventInit, val: bool);
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &ExtendableMessageEventInit) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "data")]
    fn set_data(this: &ExtendableMessageEventInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `lastEventId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "lastEventId")]
    pub fn get_last_event_id(this: &ExtendableMessageEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "lastEventId")]
    fn set_last_event_id(this: &ExtendableMessageEventInit, val: &str);
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "origin")]
    pub fn get_origin(this: &ExtendableMessageEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "origin")]
    fn set_origin(this: &ExtendableMessageEventInit, val: &str);
    #[doc = "Get the `ports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "ports")]
    pub fn get_ports(this: &ExtendableMessageEventInit) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "ports")]
    fn set_ports(this: &ExtendableMessageEventInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &ExtendableMessageEventInit) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, setter = "source")]
    fn set_source(this: &ExtendableMessageEventInit, val: Option<&::js_sys::Object>);
}
impl ExtendableMessageEventInit {
    #[doc = "Construct a new `ExtendableMessageEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    pub fn data(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_data(val);
        self
    }
    #[doc = "Change the `lastEventId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    pub fn last_event_id(&mut self, val: &str) -> &mut Self {
        self.set_last_event_id(val);
        self
    }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    pub fn origin(&mut self, val: &str) -> &mut Self {
        self.set_origin(val);
        self
    }
    #[doc = "Change the `ports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    pub fn ports(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_ports(val);
        self
    }
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
    pub fn source(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_source(val);
        self
    }
}
impl Default for ExtendableMessageEventInit {
    fn default() -> Self {
        Self::new()
    }
}
