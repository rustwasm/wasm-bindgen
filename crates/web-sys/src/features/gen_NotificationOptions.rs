#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = NotificationOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NotificationOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub type NotificationOptions;
    #[wasm_bindgen(method, setter = "actions")]
    fn actions_shim(this: &NotificationOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "badge")]
    fn badge_shim(this: &NotificationOptions, val: &str);
    #[wasm_bindgen(method, setter = "body")]
    fn body_shim(this: &NotificationOptions, val: &str);
    #[wasm_bindgen(method, setter = "data")]
    fn data_shim(this: &NotificationOptions, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "NotificationDirection")]
    #[wasm_bindgen(method, setter = "dir")]
    fn dir_shim(this: &NotificationOptions, val: NotificationDirection);
    #[wasm_bindgen(method, setter = "icon")]
    fn icon_shim(this: &NotificationOptions, val: &str);
    #[wasm_bindgen(method, setter = "image")]
    fn image_shim(this: &NotificationOptions, val: &str);
    #[wasm_bindgen(method, setter = "lang")]
    fn lang_shim(this: &NotificationOptions, val: &str);
    #[wasm_bindgen(method, setter = "renotify")]
    fn renotify_shim(this: &NotificationOptions, val: bool);
    #[wasm_bindgen(method, setter = "requireInteraction")]
    fn require_interaction_shim(this: &NotificationOptions, val: bool);
    #[wasm_bindgen(method, setter = "silent")]
    fn silent_shim(this: &NotificationOptions, val: Option<bool>);
    #[wasm_bindgen(method, setter = "tag")]
    fn tag_shim(this: &NotificationOptions, val: &str);
    #[wasm_bindgen(method, setter = "timestamp")]
    fn timestamp_shim(this: &NotificationOptions, val: f64);
}
impl NotificationOptions {
    #[doc = "Construct a new `NotificationOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `actions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn actions(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.actions_shim(val);
        self
    }
    #[doc = "Change the `badge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn badge(&mut self, val: &str) -> &mut Self {
        self.badge_shim(val);
        self
    }
    #[doc = "Change the `body` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn body(&mut self, val: &str) -> &mut Self {
        self.body_shim(val);
        self
    }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn data(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.data_shim(val);
        self
    }
    #[cfg(feature = "NotificationDirection")]
    #[doc = "Change the `dir` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationDirection`, `NotificationOptions`*"]
    pub fn dir(&mut self, val: NotificationDirection) -> &mut Self {
        self.dir_shim(val);
        self
    }
    #[doc = "Change the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn icon(&mut self, val: &str) -> &mut Self {
        self.icon_shim(val);
        self
    }
    #[doc = "Change the `image` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn image(&mut self, val: &str) -> &mut Self {
        self.image_shim(val);
        self
    }
    #[doc = "Change the `lang` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn lang(&mut self, val: &str) -> &mut Self {
        self.lang_shim(val);
        self
    }
    #[doc = "Change the `renotify` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn renotify(&mut self, val: bool) -> &mut Self {
        self.renotify_shim(val);
        self
    }
    #[doc = "Change the `requireInteraction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn require_interaction(&mut self, val: bool) -> &mut Self {
        self.require_interaction_shim(val);
        self
    }
    #[doc = "Change the `silent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn silent(&mut self, val: Option<bool>) -> &mut Self {
        self.silent_shim(val);
        self
    }
    #[doc = "Change the `tag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn tag(&mut self, val: &str) -> &mut Self {
        self.tag_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.timestamp_shim(val);
        self
    }
}
impl Default for NotificationOptions {
    fn default() -> Self {
        Self::new()
    }
}
