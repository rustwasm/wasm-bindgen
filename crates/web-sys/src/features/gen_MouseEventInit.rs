#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MouseEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MouseEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub type MouseEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &MouseEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &MouseEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &MouseEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &MouseEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &MouseEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &MouseEventInit, val: bool);
    #[doc = "Get the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "detail")]
    pub fn get_detail(this: &MouseEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "detail")]
    fn set_detail(this: &MouseEventInit, val: i32);
    #[cfg(feature = "Window")]
    #[doc = "Get the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`, `Window`*"]
    #[wasm_bindgen(method, getter = "view")]
    pub fn get_view(this: &MouseEventInit) -> Option<Window>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, setter = "view")]
    fn set_view(this: &MouseEventInit, val: Option<&Window>);
    #[doc = "Get the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "altKey")]
    pub fn get_alt_key(this: &MouseEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "altKey")]
    fn set_alt_key(this: &MouseEventInit, val: bool);
    #[doc = "Get the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "ctrlKey")]
    pub fn get_ctrl_key(this: &MouseEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "ctrlKey")]
    fn set_ctrl_key(this: &MouseEventInit, val: bool);
    #[doc = "Get the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "metaKey")]
    pub fn get_meta_key(this: &MouseEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "metaKey")]
    fn set_meta_key(this: &MouseEventInit, val: bool);
    #[doc = "Get the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierAltGraph")]
    pub fn get_modifier_alt_graph(this: &MouseEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierAltGraph")]
    fn set_modifier_alt_graph(this: &MouseEventInit, val: bool);
    #[doc = "Get the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierCapsLock")]
    pub fn get_modifier_caps_lock(this: &MouseEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierCapsLock")]
    fn set_modifier_caps_lock(this: &MouseEventInit, val: bool);
    #[doc = "Get the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierFn")]
    pub fn get_modifier_fn(this: &MouseEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierFn")]
    fn set_modifier_fn(this: &MouseEventInit, val: bool);
    #[doc = "Get the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierFnLock")]
    pub fn get_modifier_fn_lock(this: &MouseEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierFnLock")]
    fn set_modifier_fn_lock(this: &MouseEventInit, val: bool);
    #[doc = "Get the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierNumLock")]
    pub fn get_modifier_num_lock(this: &MouseEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierNumLock")]
    fn set_modifier_num_lock(this: &MouseEventInit, val: bool);
    #[doc = "Get the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierOS")]
    pub fn get_modifier_os(this: &MouseEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierOS")]
    fn set_modifier_os(this: &MouseEventInit, val: bool);
    #[doc = "Get the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierScrollLock")]
    pub fn get_modifier_scroll_lock(this: &MouseEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierScrollLock")]
    fn set_modifier_scroll_lock(this: &MouseEventInit, val: bool);
    #[doc = "Get the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierSymbol")]
    pub fn get_modifier_symbol(this: &MouseEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierSymbol")]
    fn set_modifier_symbol(this: &MouseEventInit, val: bool);
    #[doc = "Get the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierSymbolLock")]
    pub fn get_modifier_symbol_lock(this: &MouseEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierSymbolLock")]
    fn set_modifier_symbol_lock(this: &MouseEventInit, val: bool);
    #[doc = "Get the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "shiftKey")]
    pub fn get_shift_key(this: &MouseEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "shiftKey")]
    fn set_shift_key(this: &MouseEventInit, val: bool);
    #[doc = "Get the `button` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "button")]
    pub fn get_button(this: &MouseEventInit) -> Option<i16>;
    #[wasm_bindgen(method, setter = "button")]
    fn set_button(this: &MouseEventInit, val: i16);
    #[doc = "Get the `buttons` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "buttons")]
    pub fn get_buttons(this: &MouseEventInit) -> Option<u16>;
    #[wasm_bindgen(method, setter = "buttons")]
    fn set_buttons(this: &MouseEventInit, val: u16);
    #[doc = "Get the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "clientX")]
    pub fn get_client_x(this: &MouseEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "clientX")]
    fn set_client_x(this: &MouseEventInit, val: i32);
    #[doc = "Get the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "clientY")]
    pub fn get_client_y(this: &MouseEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "clientY")]
    fn set_client_y(this: &MouseEventInit, val: i32);
    #[doc = "Get the `movementX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "movementX")]
    pub fn get_movement_x(this: &MouseEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "movementX")]
    fn set_movement_x(this: &MouseEventInit, val: i32);
    #[doc = "Get the `movementY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "movementY")]
    pub fn get_movement_y(this: &MouseEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "movementY")]
    fn set_movement_y(this: &MouseEventInit, val: i32);
    #[cfg(feature = "EventTarget")]
    #[doc = "Get the `relatedTarget` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "relatedTarget")]
    pub fn get_related_target(this: &MouseEventInit) -> Option<EventTarget>;
    #[cfg(feature = "EventTarget")]
    #[wasm_bindgen(method, setter = "relatedTarget")]
    fn set_related_target(this: &MouseEventInit, val: Option<&EventTarget>);
    #[doc = "Get the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "screenX")]
    pub fn get_screen_x(this: &MouseEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "screenX")]
    fn set_screen_x(this: &MouseEventInit, val: i32);
    #[doc = "Get the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    #[wasm_bindgen(method, getter = "screenY")]
    pub fn get_screen_y(this: &MouseEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "screenY")]
    fn set_screen_y(this: &MouseEventInit, val: i32);
}
impl MouseEventInit {
    #[doc = "Construct a new `MouseEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn detail(&mut self, val: i32) -> &mut Self {
        self.set_detail(val);
        self
    }
    #[cfg(feature = "Window")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`, `Window`*"]
    pub fn view(&mut self, val: Option<&Window>) -> &mut Self {
        self.set_view(val);
        self
    }
    #[doc = "Change the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn alt_key(&mut self, val: bool) -> &mut Self {
        self.set_alt_key(val);
        self
    }
    #[doc = "Change the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn ctrl_key(&mut self, val: bool) -> &mut Self {
        self.set_ctrl_key(val);
        self
    }
    #[doc = "Change the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn meta_key(&mut self, val: bool) -> &mut Self {
        self.set_meta_key(val);
        self
    }
    #[doc = "Change the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_alt_graph(&mut self, val: bool) -> &mut Self {
        self.set_modifier_alt_graph(val);
        self
    }
    #[doc = "Change the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_caps_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_caps_lock(val);
        self
    }
    #[doc = "Change the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_fn(&mut self, val: bool) -> &mut Self {
        self.set_modifier_fn(val);
        self
    }
    #[doc = "Change the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_fn_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_fn_lock(val);
        self
    }
    #[doc = "Change the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_num_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_num_lock(val);
        self
    }
    #[doc = "Change the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_os(&mut self, val: bool) -> &mut Self {
        self.set_modifier_os(val);
        self
    }
    #[doc = "Change the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_scroll_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_scroll_lock(val);
        self
    }
    #[doc = "Change the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_symbol(&mut self, val: bool) -> &mut Self {
        self.set_modifier_symbol(val);
        self
    }
    #[doc = "Change the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_symbol_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_symbol_lock(val);
        self
    }
    #[doc = "Change the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn shift_key(&mut self, val: bool) -> &mut Self {
        self.set_shift_key(val);
        self
    }
    #[doc = "Change the `button` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn button(&mut self, val: i16) -> &mut Self {
        self.set_button(val);
        self
    }
    #[doc = "Change the `buttons` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn buttons(&mut self, val: u16) -> &mut Self {
        self.set_buttons(val);
        self
    }
    #[doc = "Change the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn client_x(&mut self, val: i32) -> &mut Self {
        self.set_client_x(val);
        self
    }
    #[doc = "Change the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn client_y(&mut self, val: i32) -> &mut Self {
        self.set_client_y(val);
        self
    }
    #[doc = "Change the `movementX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn movement_x(&mut self, val: i32) -> &mut Self {
        self.set_movement_x(val);
        self
    }
    #[doc = "Change the `movementY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn movement_y(&mut self, val: i32) -> &mut Self {
        self.set_movement_y(val);
        self
    }
    #[cfg(feature = "EventTarget")]
    #[doc = "Change the `relatedTarget` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `MouseEventInit`*"]
    pub fn related_target(&mut self, val: Option<&EventTarget>) -> &mut Self {
        self.set_related_target(val);
        self
    }
    #[doc = "Change the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn screen_x(&mut self, val: i32) -> &mut Self {
        self.set_screen_x(val);
        self
    }
    #[doc = "Change the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn screen_y(&mut self, val: i32) -> &mut Self {
        self.set_screen_y(val);
        self
    }
}
impl Default for MouseEventInit {
    fn default() -> Self {
        Self::new()
    }
}
