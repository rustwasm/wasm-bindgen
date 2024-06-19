#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PointerEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PointerEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub type PointerEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &PointerEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &PointerEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &PointerEventInit, val: bool);
    #[doc = "Get the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "detail")]
    pub fn get_detail(this: &PointerEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "detail")]
    fn set_detail(this: &PointerEventInit, val: i32);
    #[cfg(feature = "Window")]
    #[doc = "Get the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`, `Window`*"]
    #[wasm_bindgen(method, getter = "view")]
    pub fn get_view(this: &PointerEventInit) -> Option<Window>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, setter = "view")]
    fn set_view(this: &PointerEventInit, val: Option<&Window>);
    #[doc = "Get the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "altKey")]
    pub fn get_alt_key(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "altKey")]
    fn set_alt_key(this: &PointerEventInit, val: bool);
    #[doc = "Get the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "ctrlKey")]
    pub fn get_ctrl_key(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "ctrlKey")]
    fn set_ctrl_key(this: &PointerEventInit, val: bool);
    #[doc = "Get the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "metaKey")]
    pub fn get_meta_key(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "metaKey")]
    fn set_meta_key(this: &PointerEventInit, val: bool);
    #[doc = "Get the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierAltGraph")]
    pub fn get_modifier_alt_graph(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierAltGraph")]
    fn set_modifier_alt_graph(this: &PointerEventInit, val: bool);
    #[doc = "Get the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierCapsLock")]
    pub fn get_modifier_caps_lock(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierCapsLock")]
    fn set_modifier_caps_lock(this: &PointerEventInit, val: bool);
    #[doc = "Get the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierFn")]
    pub fn get_modifier_fn(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierFn")]
    fn set_modifier_fn(this: &PointerEventInit, val: bool);
    #[doc = "Get the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierFnLock")]
    pub fn get_modifier_fn_lock(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierFnLock")]
    fn set_modifier_fn_lock(this: &PointerEventInit, val: bool);
    #[doc = "Get the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierNumLock")]
    pub fn get_modifier_num_lock(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierNumLock")]
    fn set_modifier_num_lock(this: &PointerEventInit, val: bool);
    #[doc = "Get the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierOS")]
    pub fn get_modifier_os(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierOS")]
    fn set_modifier_os(this: &PointerEventInit, val: bool);
    #[doc = "Get the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierScrollLock")]
    pub fn get_modifier_scroll_lock(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierScrollLock")]
    fn set_modifier_scroll_lock(this: &PointerEventInit, val: bool);
    #[doc = "Get the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierSymbol")]
    pub fn get_modifier_symbol(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierSymbol")]
    fn set_modifier_symbol(this: &PointerEventInit, val: bool);
    #[doc = "Get the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "modifierSymbolLock")]
    pub fn get_modifier_symbol_lock(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "modifierSymbolLock")]
    fn set_modifier_symbol_lock(this: &PointerEventInit, val: bool);
    #[doc = "Get the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "shiftKey")]
    pub fn get_shift_key(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "shiftKey")]
    fn set_shift_key(this: &PointerEventInit, val: bool);
    #[doc = "Get the `button` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "button")]
    pub fn get_button(this: &PointerEventInit) -> Option<i16>;
    #[wasm_bindgen(method, setter = "button")]
    fn set_button(this: &PointerEventInit, val: i16);
    #[doc = "Get the `buttons` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "buttons")]
    pub fn get_buttons(this: &PointerEventInit) -> Option<u16>;
    #[wasm_bindgen(method, setter = "buttons")]
    fn set_buttons(this: &PointerEventInit, val: u16);
    #[doc = "Get the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "clientX")]
    pub fn get_client_x(this: &PointerEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "clientX")]
    fn set_client_x(this: &PointerEventInit, val: i32);
    #[doc = "Get the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "clientY")]
    pub fn get_client_y(this: &PointerEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "clientY")]
    fn set_client_y(this: &PointerEventInit, val: i32);
    #[doc = "Get the `movementX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "movementX")]
    pub fn get_movement_x(this: &PointerEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "movementX")]
    fn set_movement_x(this: &PointerEventInit, val: i32);
    #[doc = "Get the `movementY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "movementY")]
    pub fn get_movement_y(this: &PointerEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "movementY")]
    fn set_movement_y(this: &PointerEventInit, val: i32);
    #[cfg(feature = "EventTarget")]
    #[doc = "Get the `relatedTarget` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "relatedTarget")]
    pub fn get_related_target(this: &PointerEventInit) -> Option<EventTarget>;
    #[cfg(feature = "EventTarget")]
    #[wasm_bindgen(method, setter = "relatedTarget")]
    fn set_related_target(this: &PointerEventInit, val: Option<&EventTarget>);
    #[doc = "Get the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "screenX")]
    pub fn get_screen_x(this: &PointerEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "screenX")]
    fn set_screen_x(this: &PointerEventInit, val: i32);
    #[doc = "Get the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "screenY")]
    pub fn get_screen_y(this: &PointerEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "screenY")]
    fn set_screen_y(this: &PointerEventInit, val: i32);
    #[doc = "Get the `coalescedEvents` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "coalescedEvents")]
    pub fn get_coalesced_events(this: &PointerEventInit) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "coalescedEvents")]
    fn set_coalesced_events(this: &PointerEventInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &PointerEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "height")]
    fn set_height(this: &PointerEventInit, val: i32);
    #[doc = "Get the `isPrimary` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "isPrimary")]
    pub fn get_is_primary(this: &PointerEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "isPrimary")]
    fn set_is_primary(this: &PointerEventInit, val: bool);
    #[doc = "Get the `pointerId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "pointerId")]
    pub fn get_pointer_id(this: &PointerEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "pointerId")]
    fn set_pointer_id(this: &PointerEventInit, val: i32);
    #[doc = "Get the `pointerType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "pointerType")]
    pub fn get_pointer_type(this: &PointerEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "pointerType")]
    fn set_pointer_type(this: &PointerEventInit, val: &str);
    #[doc = "Get the `pressure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "pressure")]
    pub fn get_pressure(this: &PointerEventInit) -> Option<f32>;
    #[wasm_bindgen(method, setter = "pressure")]
    fn set_pressure(this: &PointerEventInit, val: f32);
    #[doc = "Get the `tangentialPressure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "tangentialPressure")]
    pub fn get_tangential_pressure(this: &PointerEventInit) -> Option<f32>;
    #[wasm_bindgen(method, setter = "tangentialPressure")]
    fn set_tangential_pressure(this: &PointerEventInit, val: f32);
    #[doc = "Get the `tiltX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "tiltX")]
    pub fn get_tilt_x(this: &PointerEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "tiltX")]
    fn set_tilt_x(this: &PointerEventInit, val: i32);
    #[doc = "Get the `tiltY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "tiltY")]
    pub fn get_tilt_y(this: &PointerEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "tiltY")]
    fn set_tilt_y(this: &PointerEventInit, val: i32);
    #[doc = "Get the `twist` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "twist")]
    pub fn get_twist(this: &PointerEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "twist")]
    fn set_twist(this: &PointerEventInit, val: i32);
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &PointerEventInit) -> Option<i32>;
    #[wasm_bindgen(method, setter = "width")]
    fn set_width(this: &PointerEventInit, val: i32);
}
impl PointerEventInit {
    #[doc = "Construct a new `PointerEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn detail(&mut self, val: i32) -> &mut Self {
        self.set_detail(val);
        self
    }
    #[cfg(feature = "Window")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`, `Window`*"]
    pub fn view(&mut self, val: Option<&Window>) -> &mut Self {
        self.set_view(val);
        self
    }
    #[doc = "Change the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn alt_key(&mut self, val: bool) -> &mut Self {
        self.set_alt_key(val);
        self
    }
    #[doc = "Change the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn ctrl_key(&mut self, val: bool) -> &mut Self {
        self.set_ctrl_key(val);
        self
    }
    #[doc = "Change the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn meta_key(&mut self, val: bool) -> &mut Self {
        self.set_meta_key(val);
        self
    }
    #[doc = "Change the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_alt_graph(&mut self, val: bool) -> &mut Self {
        self.set_modifier_alt_graph(val);
        self
    }
    #[doc = "Change the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_caps_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_caps_lock(val);
        self
    }
    #[doc = "Change the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_fn(&mut self, val: bool) -> &mut Self {
        self.set_modifier_fn(val);
        self
    }
    #[doc = "Change the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_fn_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_fn_lock(val);
        self
    }
    #[doc = "Change the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_num_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_num_lock(val);
        self
    }
    #[doc = "Change the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_os(&mut self, val: bool) -> &mut Self {
        self.set_modifier_os(val);
        self
    }
    #[doc = "Change the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_scroll_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_scroll_lock(val);
        self
    }
    #[doc = "Change the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_symbol(&mut self, val: bool) -> &mut Self {
        self.set_modifier_symbol(val);
        self
    }
    #[doc = "Change the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_symbol_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_symbol_lock(val);
        self
    }
    #[doc = "Change the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn shift_key(&mut self, val: bool) -> &mut Self {
        self.set_shift_key(val);
        self
    }
    #[doc = "Change the `button` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn button(&mut self, val: i16) -> &mut Self {
        self.set_button(val);
        self
    }
    #[doc = "Change the `buttons` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn buttons(&mut self, val: u16) -> &mut Self {
        self.set_buttons(val);
        self
    }
    #[doc = "Change the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn client_x(&mut self, val: i32) -> &mut Self {
        self.set_client_x(val);
        self
    }
    #[doc = "Change the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn client_y(&mut self, val: i32) -> &mut Self {
        self.set_client_y(val);
        self
    }
    #[doc = "Change the `movementX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn movement_x(&mut self, val: i32) -> &mut Self {
        self.set_movement_x(val);
        self
    }
    #[doc = "Change the `movementY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn movement_y(&mut self, val: i32) -> &mut Self {
        self.set_movement_y(val);
        self
    }
    #[cfg(feature = "EventTarget")]
    #[doc = "Change the `relatedTarget` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `PointerEventInit`*"]
    pub fn related_target(&mut self, val: Option<&EventTarget>) -> &mut Self {
        self.set_related_target(val);
        self
    }
    #[doc = "Change the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn screen_x(&mut self, val: i32) -> &mut Self {
        self.set_screen_x(val);
        self
    }
    #[doc = "Change the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn screen_y(&mut self, val: i32) -> &mut Self {
        self.set_screen_y(val);
        self
    }
    #[doc = "Change the `coalescedEvents` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn coalesced_events(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_coalesced_events(val);
        self
    }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[doc = "Change the `isPrimary` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn is_primary(&mut self, val: bool) -> &mut Self {
        self.set_is_primary(val);
        self
    }
    #[doc = "Change the `pointerId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn pointer_id(&mut self, val: i32) -> &mut Self {
        self.set_pointer_id(val);
        self
    }
    #[doc = "Change the `pointerType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn pointer_type(&mut self, val: &str) -> &mut Self {
        self.set_pointer_type(val);
        self
    }
    #[doc = "Change the `pressure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn pressure(&mut self, val: f32) -> &mut Self {
        self.set_pressure(val);
        self
    }
    #[doc = "Change the `tangentialPressure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn tangential_pressure(&mut self, val: f32) -> &mut Self {
        self.set_tangential_pressure(val);
        self
    }
    #[doc = "Change the `tiltX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn tilt_x(&mut self, val: i32) -> &mut Self {
        self.set_tilt_x(val);
        self
    }
    #[doc = "Change the `tiltY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn tilt_y(&mut self, val: i32) -> &mut Self {
        self.set_tilt_y(val);
        self
    }
    #[doc = "Change the `twist` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn twist(&mut self, val: i32) -> &mut Self {
        self.set_twist(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
}
impl Default for PointerEventInit {
    fn default() -> Self {
        Self::new()
    }
}
