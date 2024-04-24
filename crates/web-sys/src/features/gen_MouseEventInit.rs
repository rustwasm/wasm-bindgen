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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &MouseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &MouseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &MouseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &MouseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &MouseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &MouseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "detail")]
    fn detail_shim(this: &MouseEventInit) -> i32;
    #[wasm_bindgen(method, setter = "detail")]
    fn set_detail_shim(this: &MouseEventInit, val: i32);
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, getter = "view")]
    fn view_shim(this: &MouseEventInit) -> Option<Window>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, setter = "view")]
    fn set_view_shim(this: &MouseEventInit, val: Option<&Window>);
    #[wasm_bindgen(method, getter = "altKey")]
    fn alt_key_shim(this: &MouseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "altKey")]
    fn set_alt_key_shim(this: &MouseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "ctrlKey")]
    fn ctrl_key_shim(this: &MouseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "ctrlKey")]
    fn set_ctrl_key_shim(this: &MouseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "metaKey")]
    fn meta_key_shim(this: &MouseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "metaKey")]
    fn set_meta_key_shim(this: &MouseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierAltGraph")]
    fn modifier_alt_graph_shim(this: &MouseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierAltGraph")]
    fn set_modifier_alt_graph_shim(this: &MouseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierCapsLock")]
    fn modifier_caps_lock_shim(this: &MouseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierCapsLock")]
    fn set_modifier_caps_lock_shim(this: &MouseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierFn")]
    fn modifier_fn_shim(this: &MouseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierFn")]
    fn set_modifier_fn_shim(this: &MouseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierFnLock")]
    fn modifier_fn_lock_shim(this: &MouseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierFnLock")]
    fn set_modifier_fn_lock_shim(this: &MouseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierNumLock")]
    fn modifier_num_lock_shim(this: &MouseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierNumLock")]
    fn set_modifier_num_lock_shim(this: &MouseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierOS")]
    fn modifier_os_shim(this: &MouseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierOS")]
    fn set_modifier_os_shim(this: &MouseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierScrollLock")]
    fn modifier_scroll_lock_shim(this: &MouseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierScrollLock")]
    fn set_modifier_scroll_lock_shim(this: &MouseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierSymbol")]
    fn modifier_symbol_shim(this: &MouseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierSymbol")]
    fn set_modifier_symbol_shim(this: &MouseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierSymbolLock")]
    fn modifier_symbol_lock_shim(this: &MouseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierSymbolLock")]
    fn set_modifier_symbol_lock_shim(this: &MouseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "shiftKey")]
    fn shift_key_shim(this: &MouseEventInit) -> bool;
    #[wasm_bindgen(method, setter = "shiftKey")]
    fn set_shift_key_shim(this: &MouseEventInit, val: bool);
    #[wasm_bindgen(method, getter = "button")]
    fn button_shim(this: &MouseEventInit) -> i16;
    #[wasm_bindgen(method, setter = "button")]
    fn set_button_shim(this: &MouseEventInit, val: i16);
    #[wasm_bindgen(method, getter = "buttons")]
    fn buttons_shim(this: &MouseEventInit) -> u16;
    #[wasm_bindgen(method, setter = "buttons")]
    fn set_buttons_shim(this: &MouseEventInit, val: u16);
    #[wasm_bindgen(method, getter = "clientX")]
    fn client_x_shim(this: &MouseEventInit) -> i32;
    #[wasm_bindgen(method, setter = "clientX")]
    fn set_client_x_shim(this: &MouseEventInit, val: i32);
    #[wasm_bindgen(method, getter = "clientY")]
    fn client_y_shim(this: &MouseEventInit) -> i32;
    #[wasm_bindgen(method, setter = "clientY")]
    fn set_client_y_shim(this: &MouseEventInit, val: i32);
    #[wasm_bindgen(method, getter = "movementX")]
    fn movement_x_shim(this: &MouseEventInit) -> i32;
    #[wasm_bindgen(method, setter = "movementX")]
    fn set_movement_x_shim(this: &MouseEventInit, val: i32);
    #[wasm_bindgen(method, getter = "movementY")]
    fn movement_y_shim(this: &MouseEventInit) -> i32;
    #[wasm_bindgen(method, setter = "movementY")]
    fn set_movement_y_shim(this: &MouseEventInit, val: i32);
    #[cfg(feature = "EventTarget")]
    #[wasm_bindgen(method, getter = "relatedTarget")]
    fn related_target_shim(this: &MouseEventInit) -> Option<EventTarget>;
    #[cfg(feature = "EventTarget")]
    #[wasm_bindgen(method, setter = "relatedTarget")]
    fn set_related_target_shim(this: &MouseEventInit, val: Option<&EventTarget>);
    #[wasm_bindgen(method, getter = "screenX")]
    fn screen_x_shim(this: &MouseEventInit) -> i32;
    #[wasm_bindgen(method, setter = "screenX")]
    fn set_screen_x_shim(this: &MouseEventInit, val: i32);
    #[wasm_bindgen(method, getter = "screenY")]
    fn screen_y_shim(this: &MouseEventInit) -> i32;
    #[wasm_bindgen(method, setter = "screenY")]
    fn set_screen_y_shim(this: &MouseEventInit, val: i32);
}
#[doc = "The trait to access properties on the `MouseEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
pub trait MouseEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn detail(&self) -> i32;
    #[cfg(feature = "Window")]
    #[doc = "Get the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`, `Window`*"]
    fn view(&self) -> Option<Window>;
    #[doc = "Get the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn alt_key(&self) -> bool;
    #[doc = "Get the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn ctrl_key(&self) -> bool;
    #[doc = "Get the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn meta_key(&self) -> bool;
    #[doc = "Get the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn modifier_alt_graph(&self) -> bool;
    #[doc = "Get the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn modifier_caps_lock(&self) -> bool;
    #[doc = "Get the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn modifier_fn(&self) -> bool;
    #[doc = "Get the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn modifier_fn_lock(&self) -> bool;
    #[doc = "Get the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn modifier_num_lock(&self) -> bool;
    #[doc = "Get the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn modifier_os(&self) -> bool;
    #[doc = "Get the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn modifier_scroll_lock(&self) -> bool;
    #[doc = "Get the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn modifier_symbol(&self) -> bool;
    #[doc = "Get the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn modifier_symbol_lock(&self) -> bool;
    #[doc = "Get the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn shift_key(&self) -> bool;
    #[doc = "Get the `button` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn button(&self) -> i16;
    #[doc = "Get the `buttons` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn buttons(&self) -> u16;
    #[doc = "Get the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn client_x(&self) -> i32;
    #[doc = "Get the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn client_y(&self) -> i32;
    #[doc = "Get the `movementX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn movement_x(&self) -> i32;
    #[doc = "Get the `movementY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn movement_y(&self) -> i32;
    #[cfg(feature = "EventTarget")]
    #[doc = "Get the `relatedTarget` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `MouseEventInit`*"]
    fn related_target(&self) -> Option<EventTarget>;
    #[doc = "Get the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn screen_x(&self) -> i32;
    #[doc = "Get the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    fn screen_y(&self) -> i32;
}
impl MouseEventInitGetters for MouseEventInit {
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
    fn button(&self) -> i16 {
        self.button_shim()
    }
    fn buttons(&self) -> u16 {
        self.buttons_shim()
    }
    fn client_x(&self) -> i32 {
        self.client_x_shim()
    }
    fn client_y(&self) -> i32 {
        self.client_y_shim()
    }
    fn movement_x(&self) -> i32 {
        self.movement_x_shim()
    }
    fn movement_y(&self) -> i32 {
        self.movement_y_shim()
    }
    #[cfg(feature = "EventTarget")]
    fn related_target(&self) -> Option<EventTarget> {
        self.related_target_shim()
    }
    fn screen_x(&self) -> i32 {
        self.screen_x_shim()
    }
    fn screen_y(&self) -> i32 {
        self.screen_y_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn detail(&mut self, val: i32) -> &mut Self {
        self.set_detail_shim(val);
        self
    }
    #[cfg(feature = "Window")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`, `Window`*"]
    pub fn view(&mut self, val: Option<&Window>) -> &mut Self {
        self.set_view_shim(val);
        self
    }
    #[doc = "Change the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn alt_key(&mut self, val: bool) -> &mut Self {
        self.set_alt_key_shim(val);
        self
    }
    #[doc = "Change the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn ctrl_key(&mut self, val: bool) -> &mut Self {
        self.set_ctrl_key_shim(val);
        self
    }
    #[doc = "Change the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn meta_key(&mut self, val: bool) -> &mut Self {
        self.set_meta_key_shim(val);
        self
    }
    #[doc = "Change the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_alt_graph(&mut self, val: bool) -> &mut Self {
        self.set_modifier_alt_graph_shim(val);
        self
    }
    #[doc = "Change the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_caps_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_caps_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_fn(&mut self, val: bool) -> &mut Self {
        self.set_modifier_fn_shim(val);
        self
    }
    #[doc = "Change the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_fn_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_fn_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_num_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_num_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_os(&mut self, val: bool) -> &mut Self {
        self.set_modifier_os_shim(val);
        self
    }
    #[doc = "Change the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_scroll_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_scroll_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_symbol(&mut self, val: bool) -> &mut Self {
        self.set_modifier_symbol_shim(val);
        self
    }
    #[doc = "Change the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn modifier_symbol_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_symbol_lock_shim(val);
        self
    }
    #[doc = "Change the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn shift_key(&mut self, val: bool) -> &mut Self {
        self.set_shift_key_shim(val);
        self
    }
    #[doc = "Change the `button` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn button(&mut self, val: i16) -> &mut Self {
        self.set_button_shim(val);
        self
    }
    #[doc = "Change the `buttons` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn buttons(&mut self, val: u16) -> &mut Self {
        self.set_buttons_shim(val);
        self
    }
    #[doc = "Change the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn client_x(&mut self, val: i32) -> &mut Self {
        self.set_client_x_shim(val);
        self
    }
    #[doc = "Change the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn client_y(&mut self, val: i32) -> &mut Self {
        self.set_client_y_shim(val);
        self
    }
    #[doc = "Change the `movementX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn movement_x(&mut self, val: i32) -> &mut Self {
        self.set_movement_x_shim(val);
        self
    }
    #[doc = "Change the `movementY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn movement_y(&mut self, val: i32) -> &mut Self {
        self.set_movement_y_shim(val);
        self
    }
    #[cfg(feature = "EventTarget")]
    #[doc = "Change the `relatedTarget` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `MouseEventInit`*"]
    pub fn related_target(&mut self, val: Option<&EventTarget>) -> &mut Self {
        self.set_related_target_shim(val);
        self
    }
    #[doc = "Change the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn screen_x(&mut self, val: i32) -> &mut Self {
        self.set_screen_x_shim(val);
        self
    }
    #[doc = "Change the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEventInit`*"]
    pub fn screen_y(&mut self, val: i32) -> &mut Self {
        self.set_screen_y_shim(val);
        self
    }
}
impl Default for MouseEventInit {
    fn default() -> Self {
        Self::new()
    }
}
