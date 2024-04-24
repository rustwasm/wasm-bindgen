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
    #[wasm_bindgen(method, getter = "actions")]
    fn actions_shim(this: &NotificationOptions) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "actions")]
    fn set_actions_shim(this: &NotificationOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "badge")]
    fn badge_shim(this: &NotificationOptions) -> &str;
    #[wasm_bindgen(method, setter = "badge")]
    fn set_badge_shim(this: &NotificationOptions, val: &str);
    #[wasm_bindgen(method, getter = "body")]
    fn body_shim(this: &NotificationOptions) -> &str;
    #[wasm_bindgen(method, setter = "body")]
    fn set_body_shim(this: &NotificationOptions, val: &str);
    #[wasm_bindgen(method, getter = "data")]
    fn data_shim(this: &NotificationOptions) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "data")]
    fn set_data_shim(this: &NotificationOptions, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "NotificationDirection")]
    #[wasm_bindgen(method, getter = "dir")]
    fn dir_shim(this: &NotificationOptions) -> NotificationDirection;
    #[cfg(feature = "NotificationDirection")]
    #[wasm_bindgen(method, setter = "dir")]
    fn set_dir_shim(this: &NotificationOptions, val: NotificationDirection);
    #[wasm_bindgen(method, getter = "icon")]
    fn icon_shim(this: &NotificationOptions) -> &str;
    #[wasm_bindgen(method, setter = "icon")]
    fn set_icon_shim(this: &NotificationOptions, val: &str);
    #[wasm_bindgen(method, getter = "image")]
    fn image_shim(this: &NotificationOptions) -> &str;
    #[wasm_bindgen(method, setter = "image")]
    fn set_image_shim(this: &NotificationOptions, val: &str);
    #[wasm_bindgen(method, getter = "lang")]
    fn lang_shim(this: &NotificationOptions) -> &str;
    #[wasm_bindgen(method, setter = "lang")]
    fn set_lang_shim(this: &NotificationOptions, val: &str);
    #[wasm_bindgen(method, getter = "renotify")]
    fn renotify_shim(this: &NotificationOptions) -> bool;
    #[wasm_bindgen(method, setter = "renotify")]
    fn set_renotify_shim(this: &NotificationOptions, val: bool);
    #[wasm_bindgen(method, getter = "requireInteraction")]
    fn require_interaction_shim(this: &NotificationOptions) -> bool;
    #[wasm_bindgen(method, setter = "requireInteraction")]
    fn set_require_interaction_shim(this: &NotificationOptions, val: bool);
    #[wasm_bindgen(method, getter = "silent")]
    fn silent_shim(this: &NotificationOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "silent")]
    fn set_silent_shim(this: &NotificationOptions, val: Option<bool>);
    #[wasm_bindgen(method, getter = "tag")]
    fn tag_shim(this: &NotificationOptions) -> &str;
    #[wasm_bindgen(method, setter = "tag")]
    fn set_tag_shim(this: &NotificationOptions, val: &str);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &NotificationOptions) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &NotificationOptions, val: f64);
}
#[doc = "The trait to access properties on the `NotificationOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
pub trait NotificationOptionsGetters {
    #[doc = "Get the `actions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    fn actions(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `badge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    fn badge(&self) -> &str;
    #[doc = "Get the `body` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    fn body(&self) -> &str;
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    fn data(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(feature = "NotificationDirection")]
    #[doc = "Get the `dir` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationDirection`, `NotificationOptions`*"]
    fn dir(&self) -> NotificationDirection;
    #[doc = "Get the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    fn icon(&self) -> &str;
    #[doc = "Get the `image` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    fn image(&self) -> &str;
    #[doc = "Get the `lang` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    fn lang(&self) -> &str;
    #[doc = "Get the `renotify` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    fn renotify(&self) -> bool;
    #[doc = "Get the `requireInteraction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    fn require_interaction(&self) -> bool;
    #[doc = "Get the `silent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    fn silent(&self) -> Option<bool>;
    #[doc = "Get the `tag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    fn tag(&self) -> &str;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    fn timestamp(&self) -> f64;
}
impl NotificationOptionsGetters for NotificationOptions {
    fn actions(&self) -> &::wasm_bindgen::JsValue {
        self.actions_shim()
    }
    fn badge(&self) -> &str {
        self.badge_shim()
    }
    fn body(&self) -> &str {
        self.body_shim()
    }
    fn data(&self) -> &::wasm_bindgen::JsValue {
        self.data_shim()
    }
    #[cfg(feature = "NotificationDirection")]
    fn dir(&self) -> NotificationDirection {
        self.dir_shim()
    }
    fn icon(&self) -> &str {
        self.icon_shim()
    }
    fn image(&self) -> &str {
        self.image_shim()
    }
    fn lang(&self) -> &str {
        self.lang_shim()
    }
    fn renotify(&self) -> bool {
        self.renotify_shim()
    }
    fn require_interaction(&self) -> bool {
        self.require_interaction_shim()
    }
    fn silent(&self) -> Option<bool> {
        self.silent_shim()
    }
    fn tag(&self) -> &str {
        self.tag_shim()
    }
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
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
        self.set_actions_shim(val);
        self
    }
    #[doc = "Change the `badge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn badge(&mut self, val: &str) -> &mut Self {
        self.set_badge_shim(val);
        self
    }
    #[doc = "Change the `body` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn body(&mut self, val: &str) -> &mut Self {
        self.set_body_shim(val);
        self
    }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn data(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_data_shim(val);
        self
    }
    #[cfg(feature = "NotificationDirection")]
    #[doc = "Change the `dir` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationDirection`, `NotificationOptions`*"]
    pub fn dir(&mut self, val: NotificationDirection) -> &mut Self {
        self.set_dir_shim(val);
        self
    }
    #[doc = "Change the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn icon(&mut self, val: &str) -> &mut Self {
        self.set_icon_shim(val);
        self
    }
    #[doc = "Change the `image` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn image(&mut self, val: &str) -> &mut Self {
        self.set_image_shim(val);
        self
    }
    #[doc = "Change the `lang` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn lang(&mut self, val: &str) -> &mut Self {
        self.set_lang_shim(val);
        self
    }
    #[doc = "Change the `renotify` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn renotify(&mut self, val: bool) -> &mut Self {
        self.set_renotify_shim(val);
        self
    }
    #[doc = "Change the `requireInteraction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn require_interaction(&mut self, val: bool) -> &mut Self {
        self.set_require_interaction_shim(val);
        self
    }
    #[doc = "Change the `silent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn silent(&mut self, val: Option<bool>) -> &mut Self {
        self.set_silent_shim(val);
        self
    }
    #[doc = "Change the `tag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn tag(&mut self, val: &str) -> &mut Self {
        self.set_tag_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
}
impl Default for NotificationOptions {
    fn default() -> Self {
        Self::new()
    }
}
