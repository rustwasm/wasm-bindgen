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
    #[wasm_bindgen(method, getter = "adoptedCallback")]
    fn adopted_callback_shim(this: &LifecycleCallbacks) -> ::js_sys::Function;
    #[wasm_bindgen(method, setter = "adoptedCallback")]
    fn set_adopted_callback_shim(this: &LifecycleCallbacks, val: &::js_sys::Function);
    #[wasm_bindgen(method, getter = "attributeChangedCallback")]
    fn attribute_changed_callback_shim(this: &LifecycleCallbacks) -> ::js_sys::Function;
    #[wasm_bindgen(method, setter = "attributeChangedCallback")]
    fn set_attribute_changed_callback_shim(this: &LifecycleCallbacks, val: &::js_sys::Function);
    #[wasm_bindgen(method, getter = "connectedCallback")]
    fn connected_callback_shim(this: &LifecycleCallbacks) -> ::js_sys::Function;
    #[wasm_bindgen(method, setter = "connectedCallback")]
    fn set_connected_callback_shim(this: &LifecycleCallbacks, val: &::js_sys::Function);
    #[wasm_bindgen(method, getter = "disconnectedCallback")]
    fn disconnected_callback_shim(this: &LifecycleCallbacks) -> ::js_sys::Function;
    #[wasm_bindgen(method, setter = "disconnectedCallback")]
    fn set_disconnected_callback_shim(this: &LifecycleCallbacks, val: &::js_sys::Function);
}
#[doc = "The trait to access properties on the `LifecycleCallbacks` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
pub trait LifecycleCallbacksGetters {
    #[doc = "Get the `adoptedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    fn adopted_callback(&self) -> ::js_sys::Function;
    #[doc = "Get the `attributeChangedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    fn attribute_changed_callback(&self) -> ::js_sys::Function;
    #[doc = "Get the `connectedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    fn connected_callback(&self) -> ::js_sys::Function;
    #[doc = "Get the `disconnectedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    fn disconnected_callback(&self) -> ::js_sys::Function;
}
impl LifecycleCallbacksGetters for LifecycleCallbacks {
    fn adopted_callback(&self) -> ::js_sys::Function {
        self.adopted_callback_shim()
    }
    fn attribute_changed_callback(&self) -> ::js_sys::Function {
        self.attribute_changed_callback_shim()
    }
    fn connected_callback(&self) -> ::js_sys::Function {
        self.connected_callback_shim()
    }
    fn disconnected_callback(&self) -> ::js_sys::Function {
        self.disconnected_callback_shim()
    }
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
        self.set_adopted_callback_shim(val);
        self
    }
    #[doc = "Change the `attributeChangedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    pub fn attribute_changed_callback(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_attribute_changed_callback_shim(val);
        self
    }
    #[doc = "Change the `connectedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    pub fn connected_callback(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_connected_callback_shim(val);
        self
    }
    #[doc = "Change the `disconnectedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
    pub fn disconnected_callback(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_disconnected_callback_shim(val);
        self
    }
}
impl Default for LifecycleCallbacks {
    fn default() -> Self {
        Self::new()
    }
}
