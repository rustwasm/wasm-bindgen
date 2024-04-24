#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MutationObservingInfo)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MutationObservingInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub type MutationObservingInfo;
    #[wasm_bindgen(method, getter = "animations")]
    fn animations_shim(this: &MutationObservingInfo) -> bool;
    #[wasm_bindgen(method, setter = "animations")]
    fn set_animations_shim(this: &MutationObservingInfo, val: bool);
    #[wasm_bindgen(method, getter = "attributeFilter")]
    fn attribute_filter_shim(this: &MutationObservingInfo) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "attributeFilter")]
    fn set_attribute_filter_shim(this: &MutationObservingInfo, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "attributeOldValue")]
    fn attribute_old_value_shim(this: &MutationObservingInfo) -> bool;
    #[wasm_bindgen(method, setter = "attributeOldValue")]
    fn set_attribute_old_value_shim(this: &MutationObservingInfo, val: bool);
    #[wasm_bindgen(method, getter = "attributes")]
    fn attributes_shim(this: &MutationObservingInfo) -> bool;
    #[wasm_bindgen(method, setter = "attributes")]
    fn set_attributes_shim(this: &MutationObservingInfo, val: bool);
    #[wasm_bindgen(method, getter = "characterData")]
    fn character_data_shim(this: &MutationObservingInfo) -> bool;
    #[wasm_bindgen(method, setter = "characterData")]
    fn set_character_data_shim(this: &MutationObservingInfo, val: bool);
    #[wasm_bindgen(method, getter = "characterDataOldValue")]
    fn character_data_old_value_shim(this: &MutationObservingInfo) -> bool;
    #[wasm_bindgen(method, setter = "characterDataOldValue")]
    fn set_character_data_old_value_shim(this: &MutationObservingInfo, val: bool);
    #[wasm_bindgen(method, getter = "childList")]
    fn child_list_shim(this: &MutationObservingInfo) -> bool;
    #[wasm_bindgen(method, setter = "childList")]
    fn set_child_list_shim(this: &MutationObservingInfo, val: bool);
    #[wasm_bindgen(method, getter = "nativeAnonymousChildList")]
    fn native_anonymous_child_list_shim(this: &MutationObservingInfo) -> bool;
    #[wasm_bindgen(method, setter = "nativeAnonymousChildList")]
    fn set_native_anonymous_child_list_shim(this: &MutationObservingInfo, val: bool);
    #[wasm_bindgen(method, getter = "subtree")]
    fn subtree_shim(this: &MutationObservingInfo) -> bool;
    #[wasm_bindgen(method, setter = "subtree")]
    fn set_subtree_shim(this: &MutationObservingInfo, val: bool);
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, getter = "observedNode")]
    fn observed_node_shim(this: &MutationObservingInfo) -> Option<&Node>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, setter = "observedNode")]
    fn set_observed_node_shim(this: &MutationObservingInfo, val: Option<&Node>);
}
#[doc = "The trait to access properties on the `MutationObservingInfo` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
pub trait MutationObservingInfoGetters {
    #[doc = "Get the `animations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    fn animations(&self) -> bool;
    #[doc = "Get the `attributeFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    fn attribute_filter(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `attributeOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    fn attribute_old_value(&self) -> bool;
    #[doc = "Get the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    fn attributes(&self) -> bool;
    #[doc = "Get the `characterData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    fn character_data(&self) -> bool;
    #[doc = "Get the `characterDataOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    fn character_data_old_value(&self) -> bool;
    #[doc = "Get the `childList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    fn child_list(&self) -> bool;
    #[doc = "Get the `nativeAnonymousChildList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    fn native_anonymous_child_list(&self) -> bool;
    #[doc = "Get the `subtree` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    fn subtree(&self) -> bool;
    #[cfg(feature = "Node")]
    #[doc = "Get the `observedNode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`, `Node`*"]
    fn observed_node(&self) -> Option<&Node>;
}
impl MutationObservingInfoGetters for MutationObservingInfo {
    fn animations(&self) -> bool {
        self.animations_shim()
    }
    fn attribute_filter(&self) -> &::wasm_bindgen::JsValue {
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
    #[cfg(feature = "Node")]
    fn observed_node(&self) -> Option<&Node> {
        self.observed_node_shim()
    }
}
impl MutationObservingInfo {
    #[doc = "Construct a new `MutationObservingInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `animations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn animations(&mut self, val: bool) -> &mut Self {
        self.set_animations_shim(val);
        self
    }
    #[doc = "Change the `attributeFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn attribute_filter(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_attribute_filter_shim(val);
        self
    }
    #[doc = "Change the `attributeOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn attribute_old_value(&mut self, val: bool) -> &mut Self {
        self.set_attribute_old_value_shim(val);
        self
    }
    #[doc = "Change the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn attributes(&mut self, val: bool) -> &mut Self {
        self.set_attributes_shim(val);
        self
    }
    #[doc = "Change the `characterData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn character_data(&mut self, val: bool) -> &mut Self {
        self.set_character_data_shim(val);
        self
    }
    #[doc = "Change the `characterDataOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn character_data_old_value(&mut self, val: bool) -> &mut Self {
        self.set_character_data_old_value_shim(val);
        self
    }
    #[doc = "Change the `childList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn child_list(&mut self, val: bool) -> &mut Self {
        self.set_child_list_shim(val);
        self
    }
    #[doc = "Change the `nativeAnonymousChildList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn native_anonymous_child_list(&mut self, val: bool) -> &mut Self {
        self.set_native_anonymous_child_list_shim(val);
        self
    }
    #[doc = "Change the `subtree` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn subtree(&mut self, val: bool) -> &mut Self {
        self.set_subtree_shim(val);
        self
    }
    #[cfg(feature = "Node")]
    #[doc = "Change the `observedNode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`, `Node`*"]
    pub fn observed_node(&mut self, val: Option<&Node>) -> &mut Self {
        self.set_observed_node_shim(val);
        self
    }
}
impl Default for MutationObservingInfo {
    fn default() -> Self {
        Self::new()
    }
}
