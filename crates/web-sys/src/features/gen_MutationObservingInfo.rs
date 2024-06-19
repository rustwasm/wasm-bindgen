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
    #[doc = "Get the `animations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    #[wasm_bindgen(method, getter = "animations")]
    pub fn get_animations(this: &MutationObservingInfo) -> Option<bool>;
    #[wasm_bindgen(method, setter = "animations")]
    fn set_animations(this: &MutationObservingInfo, val: bool);
    #[doc = "Get the `attributeFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    #[wasm_bindgen(method, getter = "attributeFilter")]
    pub fn get_attribute_filter(this: &MutationObservingInfo) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "attributeFilter")]
    fn set_attribute_filter(this: &MutationObservingInfo, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `attributeOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    #[wasm_bindgen(method, getter = "attributeOldValue")]
    pub fn get_attribute_old_value(this: &MutationObservingInfo) -> Option<bool>;
    #[wasm_bindgen(method, setter = "attributeOldValue")]
    fn set_attribute_old_value(this: &MutationObservingInfo, val: bool);
    #[doc = "Get the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    #[wasm_bindgen(method, getter = "attributes")]
    pub fn get_attributes(this: &MutationObservingInfo) -> Option<bool>;
    #[wasm_bindgen(method, setter = "attributes")]
    fn set_attributes(this: &MutationObservingInfo, val: bool);
    #[doc = "Get the `characterData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    #[wasm_bindgen(method, getter = "characterData")]
    pub fn get_character_data(this: &MutationObservingInfo) -> Option<bool>;
    #[wasm_bindgen(method, setter = "characterData")]
    fn set_character_data(this: &MutationObservingInfo, val: bool);
    #[doc = "Get the `characterDataOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    #[wasm_bindgen(method, getter = "characterDataOldValue")]
    pub fn get_character_data_old_value(this: &MutationObservingInfo) -> Option<bool>;
    #[wasm_bindgen(method, setter = "characterDataOldValue")]
    fn set_character_data_old_value(this: &MutationObservingInfo, val: bool);
    #[doc = "Get the `childList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    #[wasm_bindgen(method, getter = "childList")]
    pub fn get_child_list(this: &MutationObservingInfo) -> Option<bool>;
    #[wasm_bindgen(method, setter = "childList")]
    fn set_child_list(this: &MutationObservingInfo, val: bool);
    #[doc = "Get the `nativeAnonymousChildList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    #[wasm_bindgen(method, getter = "nativeAnonymousChildList")]
    pub fn get_native_anonymous_child_list(this: &MutationObservingInfo) -> Option<bool>;
    #[wasm_bindgen(method, setter = "nativeAnonymousChildList")]
    fn set_native_anonymous_child_list(this: &MutationObservingInfo, val: bool);
    #[doc = "Get the `subtree` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    #[wasm_bindgen(method, getter = "subtree")]
    pub fn get_subtree(this: &MutationObservingInfo) -> Option<bool>;
    #[wasm_bindgen(method, setter = "subtree")]
    fn set_subtree(this: &MutationObservingInfo, val: bool);
    #[cfg(feature = "Node")]
    #[doc = "Get the `observedNode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`, `Node`*"]
    #[wasm_bindgen(method, getter = "observedNode")]
    pub fn get_observed_node(this: &MutationObservingInfo) -> Option<Node>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, setter = "observedNode")]
    fn set_observed_node(this: &MutationObservingInfo, val: Option<&Node>);
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
        self.set_animations(val);
        self
    }
    #[doc = "Change the `attributeFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn attribute_filter(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_attribute_filter(val);
        self
    }
    #[doc = "Change the `attributeOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn attribute_old_value(&mut self, val: bool) -> &mut Self {
        self.set_attribute_old_value(val);
        self
    }
    #[doc = "Change the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn attributes(&mut self, val: bool) -> &mut Self {
        self.set_attributes(val);
        self
    }
    #[doc = "Change the `characterData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn character_data(&mut self, val: bool) -> &mut Self {
        self.set_character_data(val);
        self
    }
    #[doc = "Change the `characterDataOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn character_data_old_value(&mut self, val: bool) -> &mut Self {
        self.set_character_data_old_value(val);
        self
    }
    #[doc = "Change the `childList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn child_list(&mut self, val: bool) -> &mut Self {
        self.set_child_list(val);
        self
    }
    #[doc = "Change the `nativeAnonymousChildList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn native_anonymous_child_list(&mut self, val: bool) -> &mut Self {
        self.set_native_anonymous_child_list(val);
        self
    }
    #[doc = "Change the `subtree` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`*"]
    pub fn subtree(&mut self, val: bool) -> &mut Self {
        self.set_subtree(val);
        self
    }
    #[cfg(feature = "Node")]
    #[doc = "Change the `observedNode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObservingInfo`, `Node`*"]
    pub fn observed_node(&mut self, val: Option<&Node>) -> &mut Self {
        self.set_observed_node(val);
        self
    }
}
impl Default for MutationObservingInfo {
    fn default() -> Self {
        Self::new()
    }
}
