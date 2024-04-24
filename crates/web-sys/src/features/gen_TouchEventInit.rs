#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TouchEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TouchEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub type TouchEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &TouchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &TouchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &TouchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &TouchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &TouchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &TouchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "detail")]
    fn detail_shim(this: &TouchEventInit) -> i32;
    #[wasm_bindgen(method, setter = "detail")]
    fn set_detail_shim(this: &TouchEventInit, val: i32);
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, getter = "view")]
    fn view_shim(this: &TouchEventInit) -> Option<&Window>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, setter = "view")]
    fn set_view_shim(this: &TouchEventInit, val: Option<&Window>);
    #[wasm_bindgen(method, getter = "altKey")]
    fn alt_key_shim(this: &TouchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "altKey")]
    fn set_alt_key_shim(this: &TouchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "ctrlKey")]
    fn ctrl_key_shim(this: &TouchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "ctrlKey")]
    fn set_ctrl_key_shim(this: &TouchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "metaKey")]
    fn meta_key_shim(this: &TouchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "metaKey")]
    fn set_meta_key_shim(this: &TouchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierAltGraph")]
    fn modifier_alt_graph_shim(this: &TouchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierAltGraph")]
    fn set_modifier_alt_graph_shim(this: &TouchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierCapsLock")]
    fn modifier_caps_lock_shim(this: &TouchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierCapsLock")]
    fn set_modifier_caps_lock_shim(this: &TouchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierFn")]
    fn modifier_fn_shim(this: &TouchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierFn")]
    fn set_modifier_fn_shim(this: &TouchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierFnLock")]
    fn modifier_fn_lock_shim(this: &TouchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierFnLock")]
    fn set_modifier_fn_lock_shim(this: &TouchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierNumLock")]
    fn modifier_num_lock_shim(this: &TouchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierNumLock")]
    fn set_modifier_num_lock_shim(this: &TouchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierOS")]
    fn modifier_os_shim(this: &TouchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierOS")]
    fn set_modifier_os_shim(this: &TouchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierScrollLock")]
    fn modifier_scroll_lock_shim(this: &TouchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierScrollLock")]
    fn set_modifier_scroll_lock_shim(this: &TouchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierSymbol")]
    fn modifier_symbol_shim(this: &TouchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierSymbol")]
    fn set_modifier_symbol_shim(this: &TouchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierSymbolLock")]
    fn modifier_symbol_lock_shim(this: &TouchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierSymbolLock")]
    fn set_modifier_symbol_lock_shim(this: &TouchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "shiftKey")]
    fn shift_key_shim(this: &TouchEventInit) -> bool;
    #[wasm_bindgen(method, setter = "shiftKey")]
    fn set_shift_key_shim(this: &TouchEventInit, val: bool);
    #[wasm_bindgen(method, getter = "changedTouches")]
    fn changed_touches_shim(this: &TouchEventInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "changedTouches")]
    fn set_changed_touches_shim(this: &TouchEventInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "targetTouches")]
    fn target_touches_shim(this: &TouchEventInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "targetTouches")]
    fn set_target_touches_shim(this: &TouchEventInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "touches")]
    fn touches_shim(this: &TouchEventInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "touches")]
    fn set_touches_shim(this: &TouchEventInit, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `TouchEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
pub trait TouchEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn detail(&self) -> i32;
    #[cfg(feature = "Window")]
    #[doc = "Get the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`, `Window`*"]
    fn view(&self) -> Option<&Window>;
    #[doc = "Get the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn alt_key(&self) -> bool;
    #[doc = "Get the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn ctrl_key(&self) -> bool;
    #[doc = "Get the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn meta_key(&self) -> bool;
    #[doc = "Get the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn modifier_alt_graph(&self) -> bool;
    #[doc = "Get the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn modifier_caps_lock(&self) -> bool;
    #[doc = "Get the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn modifier_fn(&self) -> bool;
    #[doc = "Get the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn modifier_fn_lock(&self) -> bool;
    #[doc = "Get the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn modifier_num_lock(&self) -> bool;
    #[doc = "Get the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn modifier_os(&self) -> bool;
    #[doc = "Get the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn modifier_scroll_lock(&self) -> bool;
    #[doc = "Get the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn modifier_symbol(&self) -> bool;
    #[doc = "Get the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn modifier_symbol_lock(&self) -> bool;
    #[doc = "Get the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn shift_key(&self) -> bool;
    #[doc = "Get the `changedTouches` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn changed_touches(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `targetTouches` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn target_touches(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `touches` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    fn touches(&self) -> &::wasm_bindgen::JsValue;
}
impl TouchEventInitGetters for TouchEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn detail(&self) -> i32 {
        self.detail_shim()
    }
    #[cfg(feature = "Window")]
    fn view(&self) -> Option<&Window> {
        self.view_shim()
    }
    fn alt_key(&self) -> bool {
        self.alt_key_shim()
    }
    fn ctrl_key(&self) -> bool {
        self.ctrl_key_shim()
    }
    fn meta_key(&self) -> bool {
        self.meta_key_shim()
    }
    fn modifier_alt_graph(&self) -> bool {
        self.modifier_alt_graph_shim()
    }
    fn modifier_caps_lock(&self) -> bool {
        self.modifier_caps_lock_shim()
    }
    fn modifier_fn(&self) -> bool {
        self.modifier_fn_shim()
    }
    fn modifier_fn_lock(&self) -> bool {
        self.modifier_fn_lock_shim()
    }
    fn modifier_num_lock(&self) -> bool {
        self.modifier_num_lock_shim()
    }
    fn modifier_os(&self) -> bool {
        self.modifier_os_shim()
    }
    fn modifier_scroll_lock(&self) -> bool {
        self.modifier_scroll_lock_shim()
    }
    fn modifier_symbol(&self) -> bool {
        self.modifier_symbol_shim()
    }
    fn modifier_symbol_lock(&self) -> bool {
        self.modifier_symbol_lock_shim()
    }
    fn shift_key(&self) -> bool {
        self.shift_key_shim()
    }
    fn changed_touches(&self) -> &::wasm_bindgen::JsValue {
        self.changed_touches_shim()
    }
    fn target_touches(&self) -> &::wasm_bindgen::JsValue {
        self.target_touches_shim()
    }
    fn touches(&self) -> &::wasm_bindgen::JsValue {
        self.touches_shim()
    }
}
impl TouchEventInit {
    #[doc = "Construct a new `TouchEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn detail(&mut self, val: i32) -> &mut Self {
        self.set_detail_shim(val);
        self
    }
    #[cfg(feature = "Window")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`, `Window`*"]
    pub fn view(&mut self, val: Option<&Window>) -> &mut Self {
        self.set_view_shim(val);
        self
    }
    #[doc = "Change the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn alt_key(&mut self, val: bool) -> &mut Self {
        self.set_alt_key_shim(val);
        self
    }
    #[doc = "Change the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn ctrl_key(&mut self, val: bool) -> &mut Self {
        self.set_ctrl_key_shim(val);
        self
    }
    #[doc = "Change the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn meta_key(&mut self, val: bool) -> &mut Self {
        self.set_meta_key_shim(val);
        self
    }
    #[doc = "Change the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn modifier_alt_graph(&mut self, val: bool) -> &mut Self {
        self.set_modifier_alt_graph_shim(val);
        self
    }
    #[doc = "Change the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn modifier_caps_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_caps_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn modifier_fn(&mut self, val: bool) -> &mut Self {
        self.set_modifier_fn_shim(val);
        self
    }
    #[doc = "Change the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn modifier_fn_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_fn_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn modifier_num_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_num_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn modifier_os(&mut self, val: bool) -> &mut Self {
        self.set_modifier_os_shim(val);
        self
    }
    #[doc = "Change the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn modifier_scroll_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_scroll_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn modifier_symbol(&mut self, val: bool) -> &mut Self {
        self.set_modifier_symbol_shim(val);
        self
    }
    #[doc = "Change the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn modifier_symbol_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_symbol_lock_shim(val);
        self
    }
    #[doc = "Change the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn shift_key(&mut self, val: bool) -> &mut Self {
        self.set_shift_key_shim(val);
        self
    }
    #[doc = "Change the `changedTouches` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn changed_touches(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_changed_touches_shim(val);
        self
    }
    #[doc = "Change the `targetTouches` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn target_touches(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_target_touches_shim(val);
        self
    }
    #[doc = "Change the `touches` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEventInit`*"]
    pub fn touches(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_touches_shim(val);
        self
    }
}
impl Default for TouchEventInit {
    fn default() -> Self {
        Self::new()
    }
}
