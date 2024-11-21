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
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "detail")]
    pub fn get_detail(this: &KeyboardEventInit) -> Option<i32>;
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "detail")]
    pub fn set_detail(this: &KeyboardEventInit, val: i32);
    #[cfg(feature = "Window")]
    #[doc = "Get the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`, `Window`*"]
    #[wasm_bindgen(method, getter = "view")]
    pub fn get_view(this: &KeyboardEventInit) -> Option<Window>;
    #[cfg(feature = "Window")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`, `Window`*"]
    #[wasm_bindgen(method, setter = "view")]
    pub fn set_view(this: &KeyboardEventInit, val: Option<&Window>);
    #[doc = "Get the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "altKey")]
    pub fn get_alt_key(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "altKey")]
    pub fn set_alt_key(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "ctrlKey")]
    pub fn get_ctrl_key(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "ctrlKey")]
    pub fn set_ctrl_key(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "metaKey")]
    pub fn get_meta_key(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "metaKey")]
    pub fn set_meta_key(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierAltGraph")]
    pub fn get_modifier_alt_graph(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "modifierAltGraph")]
    pub fn set_modifier_alt_graph(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierCapsLock")]
    pub fn get_modifier_caps_lock(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "modifierCapsLock")]
    pub fn set_modifier_caps_lock(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierFn")]
    pub fn get_modifier_fn(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "modifierFn")]
    pub fn set_modifier_fn(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierFnLock")]
    pub fn get_modifier_fn_lock(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "modifierFnLock")]
    pub fn set_modifier_fn_lock(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierNumLock")]
    pub fn get_modifier_num_lock(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "modifierNumLock")]
    pub fn set_modifier_num_lock(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierOS")]
    pub fn get_modifier_os(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "modifierOS")]
    pub fn set_modifier_os(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierScrollLock")]
    pub fn get_modifier_scroll_lock(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "modifierScrollLock")]
    pub fn set_modifier_scroll_lock(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierSymbol")]
    pub fn get_modifier_symbol(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "modifierSymbol")]
    pub fn set_modifier_symbol(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierSymbolLock")]
    pub fn get_modifier_symbol_lock(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "modifierSymbolLock")]
    pub fn set_modifier_symbol_lock(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "shiftKey")]
    pub fn get_shift_key(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "shiftKey")]
    pub fn set_shift_key(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `charCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "charCode")]
    pub fn get_char_code(this: &KeyboardEventInit) -> Option<u32>;
    #[doc = "Change the `charCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "charCode")]
    pub fn set_char_code(this: &KeyboardEventInit, val: u32);
    #[doc = "Get the `code` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "code")]
    pub fn get_code(this: &KeyboardEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `code` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "code")]
    pub fn set_code(this: &KeyboardEventInit, val: &str);
    #[doc = "Get the `isComposing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "isComposing")]
    pub fn get_is_composing(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `isComposing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "isComposing")]
    pub fn set_is_composing(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "key")]
    pub fn get_key(this: &KeyboardEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "key")]
    pub fn set_key(this: &KeyboardEventInit, val: &str);
    #[doc = "Get the `keyCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "keyCode")]
    pub fn get_key_code(this: &KeyboardEventInit) -> Option<u32>;
    #[doc = "Change the `keyCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "keyCode")]
    pub fn set_key_code(this: &KeyboardEventInit, val: u32);
    #[doc = "Get the `location` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "location")]
    pub fn get_location(this: &KeyboardEventInit) -> Option<u32>;
    #[doc = "Change the `location` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "location")]
    pub fn set_location(this: &KeyboardEventInit, val: u32);
    #[doc = "Get the `repeat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "repeat")]
    pub fn get_repeat(this: &KeyboardEventInit) -> Option<bool>;
    #[doc = "Change the `repeat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "repeat")]
    pub fn set_repeat(this: &KeyboardEventInit, val: bool);
    #[doc = "Get the `which` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, getter = "which")]
    pub fn get_which(this: &KeyboardEventInit) -> Option<u32>;
    #[doc = "Change the `which` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEventInit`*"]
    #[wasm_bindgen(method, setter = "which")]
    pub fn set_which(this: &KeyboardEventInit, val: u32);
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
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[deprecated = "Use `set_detail()` instead."]
    pub fn detail(&mut self, val: i32) -> &mut Self {
        self.set_detail(val);
        self
    }
    #[cfg(feature = "Window")]
    #[deprecated = "Use `set_view()` instead."]
    pub fn view(&mut self, val: Option<&Window>) -> &mut Self {
        self.set_view(val);
        self
    }
    #[deprecated = "Use `set_alt_key()` instead."]
    pub fn alt_key(&mut self, val: bool) -> &mut Self {
        self.set_alt_key(val);
        self
    }
    #[deprecated = "Use `set_ctrl_key()` instead."]
    pub fn ctrl_key(&mut self, val: bool) -> &mut Self {
        self.set_ctrl_key(val);
        self
    }
    #[deprecated = "Use `set_meta_key()` instead."]
    pub fn meta_key(&mut self, val: bool) -> &mut Self {
        self.set_meta_key(val);
        self
    }
    #[deprecated = "Use `set_modifier_alt_graph()` instead."]
    pub fn modifier_alt_graph(&mut self, val: bool) -> &mut Self {
        self.set_modifier_alt_graph(val);
        self
    }
    #[deprecated = "Use `set_modifier_caps_lock()` instead."]
    pub fn modifier_caps_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_caps_lock(val);
        self
    }
    #[deprecated = "Use `set_modifier_fn()` instead."]
    pub fn modifier_fn(&mut self, val: bool) -> &mut Self {
        self.set_modifier_fn(val);
        self
    }
    #[deprecated = "Use `set_modifier_fn_lock()` instead."]
    pub fn modifier_fn_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_fn_lock(val);
        self
    }
    #[deprecated = "Use `set_modifier_num_lock()` instead."]
    pub fn modifier_num_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_num_lock(val);
        self
    }
    #[deprecated = "Use `set_modifier_os()` instead."]
    pub fn modifier_os(&mut self, val: bool) -> &mut Self {
        self.set_modifier_os(val);
        self
    }
    #[deprecated = "Use `set_modifier_scroll_lock()` instead."]
    pub fn modifier_scroll_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_scroll_lock(val);
        self
    }
    #[deprecated = "Use `set_modifier_symbol()` instead."]
    pub fn modifier_symbol(&mut self, val: bool) -> &mut Self {
        self.set_modifier_symbol(val);
        self
    }
    #[deprecated = "Use `set_modifier_symbol_lock()` instead."]
    pub fn modifier_symbol_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_symbol_lock(val);
        self
    }
    #[deprecated = "Use `set_shift_key()` instead."]
    pub fn shift_key(&mut self, val: bool) -> &mut Self {
        self.set_shift_key(val);
        self
    }
    #[deprecated = "Use `set_char_code()` instead."]
    pub fn char_code(&mut self, val: u32) -> &mut Self {
        self.set_char_code(val);
        self
    }
    #[deprecated = "Use `set_code()` instead."]
    pub fn code(&mut self, val: &str) -> &mut Self {
        self.set_code(val);
        self
    }
    #[deprecated = "Use `set_is_composing()` instead."]
    pub fn is_composing(&mut self, val: bool) -> &mut Self {
        self.set_is_composing(val);
        self
    }
    #[deprecated = "Use `set_key()` instead."]
    pub fn key(&mut self, val: &str) -> &mut Self {
        self.set_key(val);
        self
    }
    #[deprecated = "Use `set_key_code()` instead."]
    pub fn key_code(&mut self, val: u32) -> &mut Self {
        self.set_key_code(val);
        self
    }
    #[deprecated = "Use `set_location()` instead."]
    pub fn location(&mut self, val: u32) -> &mut Self {
        self.set_location(val);
        self
    }
    #[deprecated = "Use `set_repeat()` instead."]
    pub fn repeat(&mut self, val: bool) -> &mut Self {
        self.set_repeat(val);
        self
    }
    #[deprecated = "Use `set_which()` instead."]
    pub fn which(&mut self, val: u32) -> &mut Self {
        self.set_which(val);
        self
    }
}
impl Default for KeyboardEventInit {
    fn default() -> Self {
        Self::new()
    }
}
