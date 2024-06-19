#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EventModifierInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EventModifierInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub type EventModifierInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &EventModifierInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &EventModifierInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &EventModifierInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &EventModifierInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &EventModifierInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &EventModifierInit, val: bool);
    #[doc = "Get the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "detail")]
    pub fn get_detail(this: &EventModifierInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "detail")]
    fn set_detail(this: &EventModifierInit, val: i32);
    #[cfg(feature = "Window")]
    #[doc = "Get the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`, `Window`*"]
    #[wasm_bindgen(method, getter = "view")]
    pub fn get_view(this: &EventModifierInit) -> Option<Window>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, setter = "view")]
    fn set_view(this: &EventModifierInit, val: Option<&Window>);
    #[doc = "Get the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "altKey")]
    pub fn get_alt_key(this: &EventModifierInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "altKey")]
    fn set_alt_key(this: &EventModifierInit, val: bool);
    #[doc = "Get the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "ctrlKey")]
    pub fn get_ctrl_key(this: &EventModifierInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "ctrlKey")]
    fn set_ctrl_key(this: &EventModifierInit, val: bool);
    #[doc = "Get the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "metaKey")]
    pub fn get_meta_key(this: &EventModifierInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "metaKey")]
    fn set_meta_key(this: &EventModifierInit, val: bool);
    #[doc = "Get the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "modifierAltGraph")]
    pub fn get_modifier_alt_graph(this: &EventModifierInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierAltGraph")]
    fn set_modifier_alt_graph(this: &EventModifierInit, val: bool);
    #[doc = "Get the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "modifierCapsLock")]
    pub fn get_modifier_caps_lock(this: &EventModifierInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierCapsLock")]
    fn set_modifier_caps_lock(this: &EventModifierInit, val: bool);
    #[doc = "Get the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "modifierFn")]
    pub fn get_modifier_fn(this: &EventModifierInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierFn")]
    fn set_modifier_fn(this: &EventModifierInit, val: bool);
    #[doc = "Get the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "modifierFnLock")]
    pub fn get_modifier_fn_lock(this: &EventModifierInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierFnLock")]
    fn set_modifier_fn_lock(this: &EventModifierInit, val: bool);
    #[doc = "Get the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "modifierNumLock")]
    pub fn get_modifier_num_lock(this: &EventModifierInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierNumLock")]
    fn set_modifier_num_lock(this: &EventModifierInit, val: bool);
    #[doc = "Get the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "modifierOS")]
    pub fn get_modifier_os(this: &EventModifierInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierOS")]
    fn set_modifier_os(this: &EventModifierInit, val: bool);
    #[doc = "Get the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "modifierScrollLock")]
    pub fn get_modifier_scroll_lock(this: &EventModifierInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierScrollLock")]
    fn set_modifier_scroll_lock(this: &EventModifierInit, val: bool);
    #[doc = "Get the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "modifierSymbol")]
    pub fn get_modifier_symbol(this: &EventModifierInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierSymbol")]
    fn set_modifier_symbol(this: &EventModifierInit, val: bool);
    #[doc = "Get the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "modifierSymbolLock")]
    pub fn get_modifier_symbol_lock(this: &EventModifierInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierSymbolLock")]
    fn set_modifier_symbol_lock(this: &EventModifierInit, val: bool);
    #[doc = "Get the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    #[wasm_bindgen(method, getter = "shiftKey")]
    pub fn get_shift_key(this: &EventModifierInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "shiftKey")]
    fn set_shift_key(this: &EventModifierInit, val: bool);
}
impl EventModifierInit {
    #[doc = "Construct a new `EventModifierInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn detail(&mut self, val: i32) -> &mut Self {
        self.set_detail(val);
        self
    }
    #[cfg(feature = "Window")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`, `Window`*"]
    pub fn view(&mut self, val: Option<&Window>) -> &mut Self {
        self.set_view(val);
        self
    }
    #[doc = "Change the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn alt_key(&mut self, val: bool) -> &mut Self {
        self.set_alt_key(val);
        self
    }
    #[doc = "Change the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn ctrl_key(&mut self, val: bool) -> &mut Self {
        self.set_ctrl_key(val);
        self
    }
    #[doc = "Change the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn meta_key(&mut self, val: bool) -> &mut Self {
        self.set_meta_key(val);
        self
    }
    #[doc = "Change the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn modifier_alt_graph(&mut self, val: bool) -> &mut Self {
        self.set_modifier_alt_graph(val);
        self
    }
    #[doc = "Change the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn modifier_caps_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_caps_lock(val);
        self
    }
    #[doc = "Change the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn modifier_fn(&mut self, val: bool) -> &mut Self {
        self.set_modifier_fn(val);
        self
    }
    #[doc = "Change the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn modifier_fn_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_fn_lock(val);
        self
    }
    #[doc = "Change the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn modifier_num_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_num_lock(val);
        self
    }
    #[doc = "Change the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn modifier_os(&mut self, val: bool) -> &mut Self {
        self.set_modifier_os(val);
        self
    }
    #[doc = "Change the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn modifier_scroll_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_scroll_lock(val);
        self
    }
    #[doc = "Change the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn modifier_symbol(&mut self, val: bool) -> &mut Self {
        self.set_modifier_symbol(val);
        self
    }
    #[doc = "Change the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn modifier_symbol_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_symbol_lock(val);
        self
    }
    #[doc = "Change the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventModifierInit`*"]
    pub fn shift_key(&mut self, val: bool) -> &mut Self {
        self.set_shift_key(val);
        self
    }
}
impl Default for EventModifierInit {
    fn default() -> Self {
        Self::new()
    }
}
