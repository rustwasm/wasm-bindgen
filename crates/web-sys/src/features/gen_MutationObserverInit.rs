#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MutationObserverInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MutationObserverInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub type MutationObserverInit;
    #[wasm_bindgen(method, getter = "animations")]
    fn animations_shim(this: &MutationObserverInit) -> bool;
    #[wasm_bindgen(method, setter = "animations")]
    fn set_animations_shim(this: &MutationObserverInit, val: bool);
    #[wasm_bindgen(method, getter = "attributeFilter")]
    fn attribute_filter_shim(this: &MutationObserverInit) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "attributeFilter")]
    fn set_attribute_filter_shim(this: &MutationObserverInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "attributeOldValue")]
    fn attribute_old_value_shim(this: &MutationObserverInit) -> bool;
    #[wasm_bindgen(method, setter = "attributeOldValue")]
    fn set_attribute_old_value_shim(this: &MutationObserverInit, val: bool);
    #[wasm_bindgen(method, getter = "attributes")]
    fn attributes_shim(this: &MutationObserverInit) -> bool;
    #[wasm_bindgen(method, setter = "attributes")]
    fn set_attributes_shim(this: &MutationObserverInit, val: bool);
    #[wasm_bindgen(method, getter = "characterData")]
    fn character_data_shim(this: &MutationObserverInit) -> bool;
    #[wasm_bindgen(method, setter = "characterData")]
    fn set_character_data_shim(this: &MutationObserverInit, val: bool);
    #[wasm_bindgen(method, getter = "characterDataOldValue")]
    fn character_data_old_value_shim(this: &MutationObserverInit) -> bool;
    #[wasm_bindgen(method, setter = "characterDataOldValue")]
    fn set_character_data_old_value_shim(this: &MutationObserverInit, val: bool);
    #[wasm_bindgen(method, getter = "childList")]
    fn child_list_shim(this: &MutationObserverInit) -> bool;
    #[wasm_bindgen(method, setter = "childList")]
    fn set_child_list_shim(this: &MutationObserverInit, val: bool);
    #[wasm_bindgen(method, getter = "nativeAnonymousChildList")]
    fn native_anonymous_child_list_shim(this: &MutationObserverInit) -> bool;
    #[wasm_bindgen(method, setter = "nativeAnonymousChildList")]
    fn set_native_anonymous_child_list_shim(this: &MutationObserverInit, val: bool);
    #[wasm_bindgen(method, getter = "subtree")]
    fn subtree_shim(this: &MutationObserverInit) -> bool;
    #[wasm_bindgen(method, setter = "subtree")]
    fn set_subtree_shim(this: &MutationObserverInit, val: bool);
}
#[doc = "The trait to access properties on the `MutationObserverInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
pub trait MutationObserverInitGetters {
    #[doc = "Get the `animations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn animations(&self) -> bool;
    #[doc = "Get the `attributeFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn attribute_filter(&self) -> ::js_sys::Array;
    #[doc = "Get the `attributeOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn attribute_old_value(&self) -> bool;
    #[doc = "Get the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn attributes(&self) -> bool;
    #[doc = "Get the `characterData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn character_data(&self) -> bool;
    #[doc = "Get the `characterDataOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn character_data_old_value(&self) -> bool;
    #[doc = "Get the `childList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn child_list(&self) -> bool;
    #[doc = "Get the `nativeAnonymousChildList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn native_anonymous_child_list(&self) -> bool;
    #[doc = "Get the `subtree` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn subtree(&self) -> bool;
}
impl MutationObserverInitGetters for MutationObserverInit {
    fn animations(&self) -> bool {
        self.animations_shim()
    }
    fn attribute_filter(&self) -> ::js_sys::Array {
        self.attribute_filter_shim()
    }
    fn attribute_old_value(&self) -> bool {
        self.attribute_old_value_shim()
    }
    fn attributes(&self) -> bool {
        self.attributes_shim()
    }
    fn character_data(&self) -> bool {
        self.character_data_shim()
    }
    fn character_data_old_value(&self) -> bool {
        self.character_data_old_value_shim()
    }
    fn child_list(&self) -> bool {
        self.child_list_shim()
    }
    fn native_anonymous_child_list(&self) -> bool {
        self.native_anonymous_child_list_shim()
    }
    fn subtree(&self) -> bool {
        self.subtree_shim()
    }
}
impl MutationObserverInit {
    #[doc = "Construct a new `MutationObserverInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `animations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn animations(&mut self, val: bool) -> &mut Self {
        self.set_animations_shim(val);
        self
    }
    #[doc = "Change the `attributeFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn attribute_filter(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_attribute_filter_shim(val);
        self
    }
    #[doc = "Change the `attributeOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn attribute_old_value(&mut self, val: bool) -> &mut Self {
        self.set_attribute_old_value_shim(val);
        self
    }
    #[doc = "Change the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn attributes(&mut self, val: bool) -> &mut Self {
        self.set_attributes_shim(val);
        self
    }
    #[doc = "Change the `characterData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn character_data(&mut self, val: bool) -> &mut Self {
        self.set_character_data_shim(val);
        self
    }
    #[doc = "Change the `characterDataOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn character_data_old_value(&mut self, val: bool) -> &mut Self {
        self.set_character_data_old_value_shim(val);
        self
    }
    #[doc = "Change the `childList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn child_list(&mut self, val: bool) -> &mut Self {
        self.set_child_list_shim(val);
        self
    }
    #[doc = "Change the `nativeAnonymousChildList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn native_anonymous_child_list(&mut self, val: bool) -> &mut Self {
        self.set_native_anonymous_child_list_shim(val);
        self
    }
    #[doc = "Change the `subtree` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn subtree(&mut self, val: bool) -> &mut Self {
        self.set_subtree_shim(val);
        self
    }
}
impl Default for MutationObserverInit {
    fn default() -> Self {
        Self::new()
    }
}
