#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaKeyMessageEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeyMessageEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
    pub type MediaKeyMessageEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &MediaKeyMessageEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &MediaKeyMessageEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &MediaKeyMessageEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &MediaKeyMessageEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &MediaKeyMessageEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &MediaKeyMessageEventInit, val: bool);
    #[wasm_bindgen(method, getter = "message")]
    fn message_shim(this: &MediaKeyMessageEventInit) -> ::js_sys::ArrayBuffer;
    #[wasm_bindgen(method, setter = "message")]
    fn set_message_shim(this: &MediaKeyMessageEventInit, val: &::js_sys::ArrayBuffer);
    #[cfg(feature = "MediaKeyMessageType")]
    #[wasm_bindgen(method, getter = "messageType")]
    fn message_type_shim(this: &MediaKeyMessageEventInit) -> MediaKeyMessageType;
    #[cfg(feature = "MediaKeyMessageType")]
    #[wasm_bindgen(method, setter = "messageType")]
    fn set_message_type_shim(this: &MediaKeyMessageEventInit, val: MediaKeyMessageType);
}
#[doc = "The trait to access properties on the `MediaKeyMessageEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
pub trait MediaKeyMessageEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
    fn message(&self) -> ::js_sys::ArrayBuffer;
    #[cfg(feature = "MediaKeyMessageType")]
    #[doc = "Get the `messageType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`, `MediaKeyMessageType`*"]
    fn message_type(&self) -> MediaKeyMessageType;
}
impl MediaKeyMessageEventInitGetters for MediaKeyMessageEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn message(&self) -> ::js_sys::ArrayBuffer {
        self.message_shim()
    }
    #[cfg(feature = "MediaKeyMessageType")]
    fn message_type(&self) -> MediaKeyMessageType {
        self.message_type_shim()
    }
}
impl MediaKeyMessageEventInit {
    #[cfg(feature = "MediaKeyMessageType")]
    #[doc = "Construct a new `MediaKeyMessageEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`, `MediaKeyMessageType`*"]
    pub fn new(message: &::js_sys::ArrayBuffer, message_type: MediaKeyMessageType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::message(&mut ret, message);
        Self::message_type(&mut ret, message_type);
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
    pub fn message(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_message_shim(val);
        self
    }
    #[cfg(feature = "MediaKeyMessageType")]
    #[doc = "Change the `messageType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`, `MediaKeyMessageType`*"]
    pub fn message_type(&mut self, val: MediaKeyMessageType) -> &mut Self {
        self.set_message_type_shim(val);
        self
    }
}
