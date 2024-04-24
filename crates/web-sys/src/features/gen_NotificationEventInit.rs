#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = NotificationEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NotificationEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationEventInit`*"]
    pub type NotificationEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &NotificationEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &NotificationEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &NotificationEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &NotificationEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &NotificationEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &NotificationEventInit, val: bool);
    #[cfg(feature = "Notification")]
    #[wasm_bindgen(method, getter = "notification")]
    fn notification_shim(this: &NotificationEventInit) -> Notification;
    #[cfg(feature = "Notification")]
    #[wasm_bindgen(method, setter = "notification")]
    fn set_notification_shim(this: &NotificationEventInit, val: &Notification);
}
#[doc = "The trait to access properties on the `NotificationEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `NotificationEventInit`*"]
pub trait NotificationEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "Notification")]
    #[doc = "Get the `notification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`, `NotificationEventInit`*"]
    fn notification(&self) -> Notification;
}
impl NotificationEventInitGetters for NotificationEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "Notification")]
    fn notification(&self) -> Notification {
        self.notification_shim()
    }
}
impl NotificationEventInit {
    #[cfg(feature = "Notification")]
    #[doc = "Construct a new `NotificationEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`, `NotificationEventInit`*"]
    pub fn new(notification: &Notification) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.notification(notification);
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "Notification")]
    #[doc = "Change the `notification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`, `NotificationEventInit`*"]
    pub fn notification(&mut self, val: &Notification) -> &mut Self {
        self.set_notification_shim(val);
        self
    }
}
