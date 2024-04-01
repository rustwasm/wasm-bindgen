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
    #[wasm_bindgen(method, setter = "animations")]
    fn animations_shim(this: &MutationObservingInfo, val: bool);
    #[wasm_bindgen(method, setter = "attributeFilter")]
    fn attribute_filter_shim(this: &MutationObservingInfo, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "attributeOldValue")]
    fn attribute_old_value_shim(this: &MutationObservingInfo, val: bool);
    #[wasm_bindgen(method, setter = "attributes")]
    fn attributes_shim(this: &MutationObservingInfo, val: bool);
    #[wasm_bindgen(method, setter = "characterData")]
    fn character_data_shim(this: &MutationObservingInfo, val: bool);
    #[wasm_bindgen(method, setter = "characterDataOldValue")]
    fn character_data_old_value_shim(this: &MutationObservingInfo, val: bool);
    #[wasm_bindgen(method, setter = "childList")]
    fn child_list_shim(this: &MutationObservingInfo, val: bool);
    #[wasm_bindgen(method, setter = "nativeAnonymousChildList")]
    fn native_anonymous_child_list_shim(this: &MutationObservingInfo, val: bool);
    #[wasm_bindgen(method, setter = "subtree")]
    fn subtree_shim(this: &MutationObservingInfo, val: bool);
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, setter = "observedNode")]
    fn observed_node_shim(this: &MutationObservingInfo, val: Option<&Node>);
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
        self.animations_shim(val);
        self
    }
    #[doc = "Change the `attributeFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn attribute_filter(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.attribute_filter_shim(val);
        self
    }
    #[doc = "Change the `attributeOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn attribute_old_value(&mut self, val: bool) -> &mut Self {
        self.attribute_old_value_shim(val);
        self
    }
    #[doc = "Change the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn attributes(&mut self, val: bool) -> &mut Self {
        self.attributes_shim(val);
        self
    }
    #[doc = "Change the `characterData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn character_data(&mut self, val: bool) -> &mut Self {
        self.character_data_shim(val);
        self
    }
    #[doc = "Change the `characterDataOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn character_data_old_value(&mut self, val: bool) -> &mut Self {
        self.character_data_old_value_shim(val);
        self
    }
    #[doc = "Change the `childList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn child_list(&mut self, val: bool) -> &mut Self {
        self.child_list_shim(val);
        self
    }
    #[doc = "Change the `nativeAnonymousChildList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn native_anonymous_child_list(&mut self, val: bool) -> &mut Self {
        self.native_anonymous_child_list_shim(val);
        self
    }
    #[doc = "Change the `subtree` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn subtree(&mut self, val: bool) -> &mut Self {
        self.subtree_shim(val);
        self
    }
    #[cfg(feature = "Node")]
    #[doc = "Change the `observedNode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`, `Node`*"]
    pub fn observed_node(&mut self, val: Option<&Node>) -> &mut Self {
        self.observed_node_shim(val);
        self
    }
}
impl Default for MutationObservingInfo {
    fn default() -> Self {
        Self::new()
    }
}
