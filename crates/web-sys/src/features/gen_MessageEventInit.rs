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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &MessageEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &MessageEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &MessageEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &MessageEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &MessageEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &MessageEventInit, val: bool);
    #[wasm_bindgen(method, getter = "data")]
    fn data_shim(this: &MessageEventInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "data")]
    fn set_data_shim(this: &MessageEventInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "lastEventId")]
    fn last_event_id_shim(this: &MessageEventInit) -> &str;
    #[wasm_bindgen(method, setter = "lastEventId")]
    fn set_last_event_id_shim(this: &MessageEventInit, val: &str);
    #[wasm_bindgen(method, getter = "origin")]
    fn origin_shim(this: &MessageEventInit) -> &str;
    #[wasm_bindgen(method, setter = "origin")]
    fn set_origin_shim(this: &MessageEventInit, val: &str);
    #[wasm_bindgen(method, getter = "ports")]
    fn ports_shim(this: &MessageEventInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "ports")]
    fn set_ports_shim(this: &MessageEventInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "source")]
    fn source_shim(this: &MessageEventInit) -> Option<&::js_sys::Object>;
    #[wasm_bindgen(method, setter = "source")]
    fn set_source_shim(this: &MessageEventInit, val: Option<&::js_sys::Object>);
}
#[doc = "The trait to access properties on the `MessageEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
pub trait MessageEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    fn data(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `lastEventId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    fn last_event_id(&self) -> &str;
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    fn origin(&self) -> &str;
    #[doc = "Get the `ports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    fn ports(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    fn source(&self) -> Option<&::js_sys::Object>;
}
impl MessageEventInitGetters for MessageEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn data(&self) -> &::wasm_bindgen::JsValue {
        self.data_shim()
    }
    fn last_event_id(&self) -> &str {
        self.last_event_id_shim()
    }
    fn origin(&self) -> &str {
        self.origin_shim()
    }
    fn ports(&self) -> &::wasm_bindgen::JsValue {
        self.ports_shim()
    }
    fn source(&self) -> Option<&::js_sys::Object> {
        self.source_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn data(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_data_shim(val);
        self
    }
    #[doc = "Change the `lastEventId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn last_event_id(&mut self, val: &str) -> &mut Self {
        self.set_last_event_id_shim(val);
        self
    }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn origin(&mut self, val: &str) -> &mut Self {
        self.set_origin_shim(val);
        self
    }
    #[doc = "Change the `ports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn ports(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_ports_shim(val);
        self
    }
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn source(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_source_shim(val);
        self
    }
}
impl Default for MessageEventInit {
    fn default() -> Self {
        Self::new()
    }
}
