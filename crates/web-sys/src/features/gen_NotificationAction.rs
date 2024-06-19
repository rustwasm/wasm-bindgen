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
    #[doc = "Get the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    #[wasm_bindgen(method, getter = "action")]
    pub fn get_action(this: &NotificationAction) -> String;
    #[wasm_bindgen(method, setter = "action")]
    fn set_action(this: &NotificationAction, val: &str);
    #[doc = "Get the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    #[wasm_bindgen(method, getter = "icon")]
    pub fn get_icon(this: &NotificationAction) -> Option<String>;
    #[wasm_bindgen(method, setter = "icon")]
    fn set_icon(this: &NotificationAction, val: &str);
    #[doc = "Get the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &NotificationAction) -> String;
    #[wasm_bindgen(method, setter = "title")]
    fn set_title(this: &NotificationAction, val: &str);
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
        self.set_action(val);
        self
    }
    #[doc = "Change the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    pub fn icon(&mut self, val: &str) -> &mut Self {
        self.set_icon(val);
        self
    }
    #[doc = "Change the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
    pub fn title(&mut self, val: &str) -> &mut Self {
        self.set_title(val);
        self
    }
}
