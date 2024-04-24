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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "detail")]
    fn detail_shim(this: &PointerEventInit) -> i32;
    #[wasm_bindgen(method, setter = "detail")]
    fn set_detail_shim(this: &PointerEventInit, val: i32);
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, getter = "view")]
    fn view_shim(this: &PointerEventInit) -> Option<Window>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, setter = "view")]
    fn set_view_shim(this: &PointerEventInit, val: Option<&Window>);
    #[wasm_bindgen(method, getter = "altKey")]
    fn alt_key_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "altKey")]
    fn set_alt_key_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "ctrlKey")]
    fn ctrl_key_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "ctrlKey")]
    fn set_ctrl_key_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "metaKey")]
    fn meta_key_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "metaKey")]
    fn set_meta_key_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierAltGraph")]
    fn modifier_alt_graph_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierAltGraph")]
    fn set_modifier_alt_graph_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierCapsLock")]
    fn modifier_caps_lock_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierCapsLock")]
    fn set_modifier_caps_lock_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierFn")]
    fn modifier_fn_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierFn")]
    fn set_modifier_fn_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierFnLock")]
    fn modifier_fn_lock_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierFnLock")]
    fn set_modifier_fn_lock_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierNumLock")]
    fn modifier_num_lock_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierNumLock")]
    fn set_modifier_num_lock_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierOS")]
    fn modifier_os_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierOS")]
    fn set_modifier_os_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierScrollLock")]
    fn modifier_scroll_lock_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierScrollLock")]
    fn set_modifier_scroll_lock_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierSymbol")]
    fn modifier_symbol_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierSymbol")]
    fn set_modifier_symbol_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "modifierSymbolLock")]
    fn modifier_symbol_lock_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "modifierSymbolLock")]
    fn set_modifier_symbol_lock_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "shiftKey")]
    fn shift_key_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "shiftKey")]
    fn set_shift_key_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "button")]
    fn button_shim(this: &PointerEventInit) -> i16;
    #[wasm_bindgen(method, setter = "button")]
    fn set_button_shim(this: &PointerEventInit, val: i16);
    #[wasm_bindgen(method, getter = "buttons")]
    fn buttons_shim(this: &PointerEventInit) -> u16;
    #[wasm_bindgen(method, setter = "buttons")]
    fn set_buttons_shim(this: &PointerEventInit, val: u16);
    #[wasm_bindgen(method, getter = "clientX")]
    fn client_x_shim(this: &PointerEventInit) -> i32;
    #[wasm_bindgen(method, setter = "clientX")]
    fn set_client_x_shim(this: &PointerEventInit, val: i32);
    #[wasm_bindgen(method, getter = "clientY")]
    fn client_y_shim(this: &PointerEventInit) -> i32;
    #[wasm_bindgen(method, setter = "clientY")]
    fn set_client_y_shim(this: &PointerEventInit, val: i32);
    #[wasm_bindgen(method, getter = "movementX")]
    fn movement_x_shim(this: &PointerEventInit) -> i32;
    #[wasm_bindgen(method, setter = "movementX")]
    fn set_movement_x_shim(this: &PointerEventInit, val: i32);
    #[wasm_bindgen(method, getter = "movementY")]
    fn movement_y_shim(this: &PointerEventInit) -> i32;
    #[wasm_bindgen(method, setter = "movementY")]
    fn set_movement_y_shim(this: &PointerEventInit, val: i32);
    #[cfg(feature = "EventTarget")]
    #[wasm_bindgen(method, getter = "relatedTarget")]
    fn related_target_shim(this: &PointerEventInit) -> Option<EventTarget>;
    #[cfg(feature = "EventTarget")]
    #[wasm_bindgen(method, setter = "relatedTarget")]
    fn set_related_target_shim(this: &PointerEventInit, val: Option<&EventTarget>);
    #[wasm_bindgen(method, getter = "screenX")]
    fn screen_x_shim(this: &PointerEventInit) -> i32;
    #[wasm_bindgen(method, setter = "screenX")]
    fn set_screen_x_shim(this: &PointerEventInit, val: i32);
    #[wasm_bindgen(method, getter = "screenY")]
    fn screen_y_shim(this: &PointerEventInit) -> i32;
    #[wasm_bindgen(method, setter = "screenY")]
    fn set_screen_y_shim(this: &PointerEventInit, val: i32);
    #[wasm_bindgen(method, getter = "coalescedEvents")]
    fn coalesced_events_shim(this: &PointerEventInit) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "coalescedEvents")]
    fn set_coalesced_events_shim(this: &PointerEventInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "height")]
    fn height_shim(this: &PointerEventInit) -> i32;
    #[wasm_bindgen(method, setter = "height")]
    fn set_height_shim(this: &PointerEventInit, val: i32);
    #[wasm_bindgen(method, getter = "isPrimary")]
    fn is_primary_shim(this: &PointerEventInit) -> bool;
    #[wasm_bindgen(method, setter = "isPrimary")]
    fn set_is_primary_shim(this: &PointerEventInit, val: bool);
    #[wasm_bindgen(method, getter = "pointerId")]
    fn pointer_id_shim(this: &PointerEventInit) -> i32;
    #[wasm_bindgen(method, setter = "pointerId")]
    fn set_pointer_id_shim(this: &PointerEventInit, val: i32);
    #[wasm_bindgen(method, getter = "pointerType")]
    fn pointer_type_shim(this: &PointerEventInit) -> String;
    #[wasm_bindgen(method, setter = "pointerType")]
    fn set_pointer_type_shim(this: &PointerEventInit, val: &str);
    #[wasm_bindgen(method, getter = "pressure")]
    fn pressure_shim(this: &PointerEventInit) -> f32;
    #[wasm_bindgen(method, setter = "pressure")]
    fn set_pressure_shim(this: &PointerEventInit, val: f32);
    #[wasm_bindgen(method, getter = "tangentialPressure")]
    fn tangential_pressure_shim(this: &PointerEventInit) -> f32;
    #[wasm_bindgen(method, setter = "tangentialPressure")]
    fn set_tangential_pressure_shim(this: &PointerEventInit, val: f32);
    #[wasm_bindgen(method, getter = "tiltX")]
    fn tilt_x_shim(this: &PointerEventInit) -> i32;
    #[wasm_bindgen(method, setter = "tiltX")]
    fn set_tilt_x_shim(this: &PointerEventInit, val: i32);
    #[wasm_bindgen(method, getter = "tiltY")]
    fn tilt_y_shim(this: &PointerEventInit) -> i32;
    #[wasm_bindgen(method, setter = "tiltY")]
    fn set_tilt_y_shim(this: &PointerEventInit, val: i32);
    #[wasm_bindgen(method, getter = "twist")]
    fn twist_shim(this: &PointerEventInit) -> i32;
    #[wasm_bindgen(method, setter = "twist")]
    fn set_twist_shim(this: &PointerEventInit, val: i32);
    #[wasm_bindgen(method, getter = "width")]
    fn width_shim(this: &PointerEventInit) -> i32;
    #[wasm_bindgen(method, setter = "width")]
    fn set_width_shim(this: &PointerEventInit, val: i32);
}
#[doc = "The trait to access properties on the `PointerEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
pub trait PointerEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn detail(&self) -> i32;
    #[cfg(feature = "Window")]
    #[doc = "Get the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`, `Window`*"]
    fn view(&self) -> Option<Window>;
    #[doc = "Get the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn alt_key(&self) -> bool;
    #[doc = "Get the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn ctrl_key(&self) -> bool;
    #[doc = "Get the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn meta_key(&self) -> bool;
    #[doc = "Get the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn modifier_alt_graph(&self) -> bool;
    #[doc = "Get the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn modifier_caps_lock(&self) -> bool;
    #[doc = "Get the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn modifier_fn(&self) -> bool;
    #[doc = "Get the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn modifier_fn_lock(&self) -> bool;
    #[doc = "Get the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn modifier_num_lock(&self) -> bool;
    #[doc = "Get the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn modifier_os(&self) -> bool;
    #[doc = "Get the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn modifier_scroll_lock(&self) -> bool;
    #[doc = "Get the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn modifier_symbol(&self) -> bool;
    #[doc = "Get the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn modifier_symbol_lock(&self) -> bool;
    #[doc = "Get the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn shift_key(&self) -> bool;
    #[doc = "Get the `button` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn button(&self) -> i16;
    #[doc = "Get the `buttons` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn buttons(&self) -> u16;
    #[doc = "Get the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn client_x(&self) -> i32;
    #[doc = "Get the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn client_y(&self) -> i32;
    #[doc = "Get the `movementX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn movement_x(&self) -> i32;
    #[doc = "Get the `movementY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn movement_y(&self) -> i32;
    #[cfg(feature = "EventTarget")]
    #[doc = "Get the `relatedTarget` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `PointerEventInit`*"]
    fn related_target(&self) -> Option<EventTarget>;
    #[doc = "Get the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn screen_x(&self) -> i32;
    #[doc = "Get the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn screen_y(&self) -> i32;
    #[doc = "Get the `coalescedEvents` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn coalesced_events(&self) -> ::js_sys::Array;
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn height(&self) -> i32;
    #[doc = "Get the `isPrimary` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn is_primary(&self) -> bool;
    #[doc = "Get the `pointerId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn pointer_id(&self) -> i32;
    #[doc = "Get the `pointerType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn pointer_type(&self) -> String;
    #[doc = "Get the `pressure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn pressure(&self) -> f32;
    #[doc = "Get the `tangentialPressure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn tangential_pressure(&self) -> f32;
    #[doc = "Get the `tiltX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn tilt_x(&self) -> i32;
    #[doc = "Get the `tiltY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn tilt_y(&self) -> i32;
    #[doc = "Get the `twist` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn twist(&self) -> i32;
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    fn width(&self) -> i32;
}
impl PointerEventInitGetters for PointerEventInit {
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
    fn coalesced_events(&self) -> ::js_sys::Array {
        self.coalesced_events_shim()
    }
    fn height(&self) -> i32 {
        self.height_shim()
    }
    fn is_primary(&self) -> bool {
        self.is_primary_shim()
    }
    fn pointer_id(&self) -> i32 {
        self.pointer_id_shim()
    }
    fn pointer_type(&self) -> String {
        self.pointer_type_shim()
    }
    fn pressure(&self) -> f32 {
        self.pressure_shim()
    }
    fn tangential_pressure(&self) -> f32 {
        self.tangential_pressure_shim()
    }
    fn tilt_x(&self) -> i32 {
        self.tilt_x_shim()
    }
    fn tilt_y(&self) -> i32 {
        self.tilt_y_shim()
    }
    fn twist(&self) -> i32 {
        self.twist_shim()
    }
    fn width(&self) -> i32 {
        self.width_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn detail(&mut self, val: i32) -> &mut Self {
        self.set_detail_shim(val);
        self
    }
    #[cfg(feature = "Window")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`, `Window`*"]
    pub fn view(&mut self, val: Option<&Window>) -> &mut Self {
        self.set_view_shim(val);
        self
    }
    #[doc = "Change the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn alt_key(&mut self, val: bool) -> &mut Self {
        self.set_alt_key_shim(val);
        self
    }
    #[doc = "Change the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn ctrl_key(&mut self, val: bool) -> &mut Self {
        self.set_ctrl_key_shim(val);
        self
    }
    #[doc = "Change the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn meta_key(&mut self, val: bool) -> &mut Self {
        self.set_meta_key_shim(val);
        self
    }
    #[doc = "Change the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_alt_graph(&mut self, val: bool) -> &mut Self {
        self.set_modifier_alt_graph_shim(val);
        self
    }
    #[doc = "Change the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_caps_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_caps_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_fn(&mut self, val: bool) -> &mut Self {
        self.set_modifier_fn_shim(val);
        self
    }
    #[doc = "Change the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_fn_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_fn_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_num_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_num_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_os(&mut self, val: bool) -> &mut Self {
        self.set_modifier_os_shim(val);
        self
    }
    #[doc = "Change the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_scroll_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_scroll_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_symbol(&mut self, val: bool) -> &mut Self {
        self.set_modifier_symbol_shim(val);
        self
    }
    #[doc = "Change the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn modifier_symbol_lock(&mut self, val: bool) -> &mut Self {
        self.set_modifier_symbol_lock_shim(val);
        self
    }
    #[doc = "Change the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn shift_key(&mut self, val: bool) -> &mut Self {
        self.set_shift_key_shim(val);
        self
    }
    #[doc = "Change the `button` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn button(&mut self, val: i16) -> &mut Self {
        self.set_button_shim(val);
        self
    }
    #[doc = "Change the `buttons` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn buttons(&mut self, val: u16) -> &mut Self {
        self.set_buttons_shim(val);
        self
    }
    #[doc = "Change the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn client_x(&mut self, val: i32) -> &mut Self {
        self.set_client_x_shim(val);
        self
    }
    #[doc = "Change the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn client_y(&mut self, val: i32) -> &mut Self {
        self.set_client_y_shim(val);
        self
    }
    #[doc = "Change the `movementX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn movement_x(&mut self, val: i32) -> &mut Self {
        self.set_movement_x_shim(val);
        self
    }
    #[doc = "Change the `movementY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn movement_y(&mut self, val: i32) -> &mut Self {
        self.set_movement_y_shim(val);
        self
    }
    #[cfg(feature = "EventTarget")]
    #[doc = "Change the `relatedTarget` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `PointerEventInit`*"]
    pub fn related_target(&mut self, val: Option<&EventTarget>) -> &mut Self {
        self.set_related_target_shim(val);
        self
    }
    #[doc = "Change the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn screen_x(&mut self, val: i32) -> &mut Self {
        self.set_screen_x_shim(val);
        self
    }
    #[doc = "Change the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn screen_y(&mut self, val: i32) -> &mut Self {
        self.set_screen_y_shim(val);
        self
    }
    #[doc = "Change the `coalescedEvents` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn coalesced_events(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_coalesced_events_shim(val);
        self
    }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height_shim(val);
        self
    }
    #[doc = "Change the `isPrimary` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn is_primary(&mut self, val: bool) -> &mut Self {
        self.set_is_primary_shim(val);
        self
    }
    #[doc = "Change the `pointerId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn pointer_id(&mut self, val: i32) -> &mut Self {
        self.set_pointer_id_shim(val);
        self
    }
    #[doc = "Change the `pointerType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn pointer_type(&mut self, val: &str) -> &mut Self {
        self.set_pointer_type_shim(val);
        self
    }
    #[doc = "Change the `pressure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn pressure(&mut self, val: f32) -> &mut Self {
        self.set_pressure_shim(val);
        self
    }
    #[doc = "Change the `tangentialPressure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn tangential_pressure(&mut self, val: f32) -> &mut Self {
        self.set_tangential_pressure_shim(val);
        self
    }
    #[doc = "Change the `tiltX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn tilt_x(&mut self, val: i32) -> &mut Self {
        self.set_tilt_x_shim(val);
        self
    }
    #[doc = "Change the `tiltY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn tilt_y(&mut self, val: i32) -> &mut Self {
        self.set_tilt_y_shim(val);
        self
    }
    #[doc = "Change the `twist` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn twist(&mut self, val: i32) -> &mut Self {
        self.set_twist_shim(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEventInit`*"]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width_shim(val);
        self
    }
}
impl Default for PointerEventInit {
    fn default() -> Self {
        Self::new()
    }
}
