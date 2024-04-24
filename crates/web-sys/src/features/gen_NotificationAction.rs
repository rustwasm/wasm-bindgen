#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = NotificationAction)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NotificationAction` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    pub type NotificationAction;
    #[wasm_bindgen(method, getter = "action")]
    fn action_shim(this: &NotificationAction) -> &str;
    #[wasm_bindgen(method, setter = "action")]
    fn set_action_shim(this: &NotificationAction, val: &str);
    #[wasm_bindgen(method, getter = "icon")]
    fn icon_shim(this: &NotificationAction) -> &str;
    #[wasm_bindgen(method, setter = "icon")]
    fn set_icon_shim(this: &NotificationAction, val: &str);
    #[wasm_bindgen(method, getter = "title")]
    fn title_shim(this: &NotificationAction) -> &str;
    #[wasm_bindgen(method, setter = "title")]
    fn set_title_shim(this: &NotificationAction, val: &str);
}
#[doc = "The trait to access properties on the `NotificationAction` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
pub trait NotificationActionGetters {
    #[doc = "Get the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    fn action(&self) -> &str;
    #[doc = "Get the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    fn icon(&self) -> &str;
    #[doc = "Get the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    fn title(&self) -> &str;
}
impl NotificationActionGetters for NotificationAction {
    fn action(&self) -> &str {
        self.action_shim()
    }
    fn icon(&self) -> &str {
        self.icon_shim()
    }
    fn title(&self) -> &str {
        self.title_shim()
    }
}
impl NotificationAction {
    #[doc = "Construct a new `NotificationAction`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    pub fn new(action: &str, title: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.action(action);
        ret.title(title);
        ret
    }
    #[doc = "Change the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    pub fn action(&mut self, val: &str) -> &mut Self {
        self.set_action_shim(val);
        self
    }
    #[doc = "Change the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    pub fn icon(&mut self, val: &str) -> &mut Self {
        self.set_icon_shim(val);
        self
    }
    #[doc = "Change the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    pub fn title(&mut self, val: &str) -> &mut Self {
        self.set_title_shim(val);
        self
    }
}
