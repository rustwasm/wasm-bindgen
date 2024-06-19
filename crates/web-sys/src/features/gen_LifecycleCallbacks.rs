#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = LifecycleCallbacks)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `LifecycleCallbacks` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    pub type LifecycleCallbacks;
    #[doc = "Get the `adoptedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    #[wasm_bindgen(method, getter = "adoptedCallback")]
    pub fn get_adopted_callback(this: &LifecycleCallbacks) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter = "adoptedCallback")]
    fn set_adopted_callback(this: &LifecycleCallbacks, val: &::js_sys::Function);
    #[doc = "Get the `attributeChangedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    #[wasm_bindgen(method, getter = "attributeChangedCallback")]
    pub fn get_attribute_changed_callback(this: &LifecycleCallbacks) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter = "attributeChangedCallback")]
    fn set_attribute_changed_callback(this: &LifecycleCallbacks, val: &::js_sys::Function);
    #[doc = "Get the `connectedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    #[wasm_bindgen(method, getter = "connectedCallback")]
    pub fn get_connected_callback(this: &LifecycleCallbacks) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter = "connectedCallback")]
    fn set_connected_callback(this: &LifecycleCallbacks, val: &::js_sys::Function);
    #[doc = "Get the `disconnectedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    #[wasm_bindgen(method, getter = "disconnectedCallback")]
    pub fn get_disconnected_callback(this: &LifecycleCallbacks) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter = "disconnectedCallback")]
    fn set_disconnected_callback(this: &LifecycleCallbacks, val: &::js_sys::Function);
}
impl LifecycleCallbacks {
    #[doc = "Construct a new `LifecycleCallbacks`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `adoptedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    pub fn adopted_callback(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_adopted_callback(val);
        self
    }
    #[doc = "Change the `attributeChangedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    pub fn attribute_changed_callback(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_attribute_changed_callback(val);
        self
    }
    #[doc = "Change the `connectedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    pub fn connected_callback(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_connected_callback(val);
        self
    }
    #[doc = "Change the `disconnectedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    pub fn disconnected_callback(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_disconnected_callback(val);
        self
    }
}
impl Default for LifecycleCallbacks {
    fn default() -> Self {
        Self::new()
    }
}
