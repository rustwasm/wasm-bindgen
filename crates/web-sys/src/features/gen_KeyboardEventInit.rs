#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = KeyboardEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `KeyboardEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub type KeyboardEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "detail")]
    fn detail_shim(this: &KeyboardEventInit) -> i32;
    #[wasm_bindgen(method, setter = "detail")]
    fn set_detail_shim(this: &KeyboardEventInit, val: i32);
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, getter = "view")]
    fn view_shim(this: &KeyboardEventInit) -> Option<Window>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, setter = "view")]
    fn set_view_shim(this: &KeyboardEventInit, val: Option<&Window>);
    #[wasm_bindgen(method, getter = "altKey")]
    fn alt_key_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "altKey")]
    fn set_alt_key_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "ctrlKey")]
    fn ctrl_key_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "ctrlKey")]
    fn set_ctrl_key_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "metaKey")]
    fn meta_key_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "metaKey")]
    fn set_meta_key_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierAltGraph")]
    fn modifier_alt_graph_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierAltGraph")]
    fn set_modifier_alt_graph_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierCapsLock")]
    fn modifier_caps_lock_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierCapsLock")]
    fn set_modifier_caps_lock_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierFn")]
    fn modifier_fn_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierFn")]
    fn set_modifier_fn_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierFnLock")]
    fn modifier_fn_lock_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierFnLock")]
    fn set_modifier_fn_lock_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierNumLock")]
    fn modifier_num_lock_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierNumLock")]
    fn set_modifier_num_lock_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierOS")]
    fn modifier_os_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierOS")]
    fn set_modifier_os_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierScrollLock")]
    fn modifier_scroll_lock_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierScrollLock")]
    fn set_modifier_scroll_lock_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierSymbol")]
    fn modifier_symbol_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierSymbol")]
    fn set_modifier_symbol_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierSymbolLock")]
    fn modifier_symbol_lock_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierSymbolLock")]
    fn set_modifier_symbol_lock_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "shiftKey")]
    fn shift_key_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "shiftKey")]
    fn set_shift_key_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "charCode")]
    fn char_code_shim(this: &KeyboardEventInit) -> u32;
    #[wasm_bindgen(method, setter = "charCode")]
    fn set_char_code_shim(this: &KeyboardEventInit, val: u32);
    #[wasm_bindgen(method, getter = "code")]
    fn code_shim(this: &KeyboardEventInit) -> String;
    #[wasm_bindgen(method, setter = "code")]
    fn set_code_shim(this: &KeyboardEventInit, val: &str);
    #[wasm_bindgen(method, getter = "isComposing")]
    fn is_composing_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "isComposing")]
    fn set_is_composing_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "key")]
    fn key_shim(this: &KeyboardEventInit) -> String;
    #[wasm_bindgen(method, setter = "key")]
    fn set_key_shim(this: &KeyboardEventInit, val: &str);
    #[wasm_bindgen(method, getter = "keyCode")]
    fn key_code_shim(this: &KeyboardEventInit) -> u32;
    #[wasm_bindgen(method, setter = "keyCode")]
    fn set_key_code_shim(this: &KeyboardEventInit, val: u32);
    #[wasm_bindgen(method, getter = "location")]
    fn location_shim(this: &KeyboardEventInit) -> u32;
    #[wasm_bindgen(method, setter = "location")]
    fn set_location_shim(this: &KeyboardEventInit, val: u32);
    #[wasm_bindgen(method, getter = "repeat")]
    fn repeat_shim(this: &KeyboardEventInit) -> bool;
    #[wasm_bindgen(method, setter = "repeat")]
    fn set_repeat_shim(this: &KeyboardEventInit, val: bool);
    #[wasm_bindgen(method, getter = "which")]
    fn which_shim(this: &KeyboardEventInit) -> u32;
    #[wasm_bindgen(method, setter = "which")]
    fn set_which_shim(this: &KeyboardEventInit, val: u32);
}
#[doc = "The trait to access properties on the `KeyboardEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
pub trait KeyboardEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn detail(&self) -> i32;
    #[cfg(feature = "Window")]
    #[doc = "Get the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`, `Window`*"]
    fn view(&self) -> Option<Window>;
    #[doc = "Get the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn alt_key(&self) -> bool;
    #[doc = "Get the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn ctrl_key(&self) -> bool;
    #[doc = "Get the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn meta_key(&self) -> bool;
    #[doc = "Get the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn modifier_alt_graph(&self) -> bool;
    #[doc = "Get the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn modifier_caps_lock(&self) -> bool;
    #[doc = "Get the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn modifier_fn(&self) -> bool;
    #[doc = "Get the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn modifier_fn_lock(&self) -> bool;
    #[doc = "Get the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn modifier_num_lock(&self) -> bool;
    #[doc = "Get the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn modifier_os(&self) -> bool;
    #[doc = "Get the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn modifier_scroll_lock(&self) -> bool;
    #[doc = "Get the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn modifier_symbol(&self) -> bool;
    #[doc = "Get the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn modifier_symbol_lock(&self) -> bool;
    #[doc = "Get the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn shift_key(&self) -> bool;
    #[doc = "Get the `charCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn char_code(&self) -> u32;
    #[doc = "Get the `code` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn code(&self) -> String;
    #[doc = "Get the `isComposing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn is_composing(&self) -> bool;
    #[doc = "Get the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn key(&self) -> String;
    #[doc = "Get the `keyCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn key_code(&self) -> u32;
    #[doc = "Get the `location` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn location(&self) -> u32;
    #[doc = "Get the `repeat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn repeat(&self) -> bool;
    #[doc = "Get the `which` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    fn which(&self) -> u32;
}
impl KeyboardEventInitGetters for KeyboardEventInit {
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
    fn view(&self) -> Option<Window> {
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
    fn char_code(&self) -> u32 {
        self.char_code_shim()
    }
    fn code(&self) -> String {
        self.code_shim()
    }
    fn is_composing(&self) -> bool {
        self.is_composing_shim()
    }
    fn key(&self) -> String {
        self.key_shim()
    }
    fn key_code(&self) -> u32 {
        self.key_code_shim()
    }
    fn location(&self) -> u32 {
        self.location_shim()
    }
    fn repeat(&self) -> bool {
        self.repeat_shim()
    }
    fn which(&self) -> u32 {
        self.which_shim()
    }
}
impl KeyboardEventInit {
    #[doc = "Construct a new `KeyboardEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn detail(&mut self, val: i32) -> &mut Self {
        self.set_detail_shim(val);
        self
    }
    #[cfg(feature = "Window")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`, `Window`*"]
    pub fn view(&mut self, val: Option<&Window>) -> &mut Self {
        self.set_view_shim(val);
        self
    }
    #[doc = "Change the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn alt_key(&mut self, val: bool) -> &mut Self {
        self.set_alt_key_shim(val);
        self
    }
    #[doc = "Change the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn ctrl_key(&mut self, val: bool) -> &mut Self {
        self.set_ctrl_key_shim(val);
        self
    }
    #[doc = "Change the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn meta_key(&mut self, val: bool) -> &mut Self {
        self.set_meta_key_shim(val);
        self
    }
    #[doc = "Change the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn modifier_alt_graph(&mut self, val: bool) -> &mut Self {
        self.set_modifier_alt_graph_shim(val);
        self
    }
    #[doc = "Change the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn modifier_caps_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_caps_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn modifier_fn(&mut self, val: bool) -> &mut Self {
        self.set_modifier_fn_shim(val);
        self
    }
    #[doc = "Change the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn modifier_fn_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_fn_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn modifier_num_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_num_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn modifier_os(&mut self, val: bool) -> &mut Self {
        self.set_modifier_os_shim(val);
        self
    }
    #[doc = "Change the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn modifier_scroll_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_scroll_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn modifier_symbol(&mut self, val: bool) -> &mut Self {
        self.set_modifier_symbol_shim(val);
        self
    }
    #[doc = "Change the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn modifier_symbol_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_symbol_lock_shim(val);
        self
    }
    #[doc = "Change the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn shift_key(&mut self, val: bool) -> &mut Self {
        self.set_shift_key_shim(val);
        self
    }
    #[doc = "Change the `charCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn char_code(&mut self, val: u32) -> &mut Self {
        self.set_char_code_shim(val);
        self
    }
    #[doc = "Change the `code` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn code(&mut self, val: &str) -> &mut Self {
        self.set_code_shim(val);
        self
    }
    #[doc = "Change the `isComposing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn is_composing(&mut self, val: bool) -> &mut Self {
        self.set_is_composing_shim(val);
        self
    }
    #[doc = "Change the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn key(&mut self, val: &str) -> &mut Self {
        self.set_key_shim(val);
        self
    }
    #[doc = "Change the `keyCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn key_code(&mut self, val: u32) -> &mut Self {
        self.set_key_code_shim(val);
        self
    }
    #[doc = "Change the `location` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn location(&mut self, val: u32) -> &mut Self {
        self.set_location_shim(val);
        self
    }
    #[doc = "Change the `repeat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn repeat(&mut self, val: bool) -> &mut Self {
        self.set_repeat_shim(val);
        self
    }
    #[doc = "Change the `which` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    pub fn which(&mut self, val: u32) -> &mut Self {
        self.set_which_shim(val);
        self
    }
}
impl Default for KeyboardEventInit {
    fn default() -> Self {
        Self::new()
    }
}
