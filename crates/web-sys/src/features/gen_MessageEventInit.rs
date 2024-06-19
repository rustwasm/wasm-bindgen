#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MessageEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MessageEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub type MessageEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &MessageEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &MessageEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &MessageEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &MessageEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &MessageEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &MessageEventInit, val: bool);
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &MessageEventInit) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "data")]
    fn set_data(this: &MessageEventInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `lastEventId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, getter = "lastEventId")]
    pub fn get_last_event_id(this: &MessageEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "lastEventId")]
    fn set_last_event_id(this: &MessageEventInit, val: &str);
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, getter = "origin")]
    pub fn get_origin(this: &MessageEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "origin")]
    fn set_origin(this: &MessageEventInit, val: &str);
    #[doc = "Get the `ports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, getter = "ports")]
    pub fn get_ports(this: &MessageEventInit) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "ports")]
    fn set_ports(this: &MessageEventInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &MessageEventInit) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, setter = "source")]
    fn set_source(this: &MessageEventInit, val: Option<&::js_sys::Object>);
}
impl MessageEventInit {
    #[doc = "Construct a new `MessageEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn data(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_data(val);
        self
    }
    #[doc = "Change the `lastEventId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn last_event_id(&mut self, val: &str) -> &mut Self {
        self.set_last_event_id(val);
        self
    }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn origin(&mut self, val: &str) -> &mut Self {
        self.set_origin(val);
        self
    }
    #[doc = "Change the `ports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn ports(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_ports(val);
        self
    }
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn source(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_source(val);
        self
    }
}
impl Default for MessageEventInit {
    fn default() -> Self {
        Self::new()
    }
}
