#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DragEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DragEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub type DragEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &DragEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &DragEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &DragEventInit, val: bool);
    #[wasm_bindgen(method, setter = "detail")]
    fn detail_shim(this: &DragEventInit, val: i32);
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, setter = "view")]
    fn view_shim(this: &DragEventInit, val: Option<&Window>);
    #[wasm_bindgen(method, setter = "altKey")]
    fn alt_key_shim(this: &DragEventInit, val: bool);
    #[wasm_bindgen(method, setter = "ctrlKey")]
    fn ctrl_key_shim(this: &DragEventInit, val: bool);
    #[wasm_bindgen(method, setter = "metaKey")]
    fn meta_key_shim(this: &DragEventInit, val: bool);
    #[wasm_bindgen(method, setter = "modifierAltGraph")]
    fn modifier_alt_graph_shim(this: &DragEventInit, val: bool);
    #[wasm_bindgen(method, setter = "modifierCapsLock")]
    fn modifier_caps_lock_shim(this: &DragEventInit, val: bool);
    #[wasm_bindgen(method, setter = "modifierFn")]
    fn modifier_fn_shim(this: &DragEventInit, val: bool);
    #[wasm_bindgen(method, setter = "modifierFnLock")]
    fn modifier_fn_lock_shim(this: &DragEventInit, val: bool);
    #[wasm_bindgen(method, setter = "modifierNumLock")]
    fn modifier_num_lock_shim(this: &DragEventInit, val: bool);
    #[wasm_bindgen(method, setter = "modifierOS")]
    fn modifier_os_shim(this: &DragEventInit, val: bool);
    #[wasm_bindgen(method, setter = "modifierScrollLock")]
    fn modifier_scroll_lock_shim(this: &DragEventInit, val: bool);
    #[wasm_bindgen(method, setter = "modifierSymbol")]
    fn modifier_symbol_shim(this: &DragEventInit, val: bool);
    #[wasm_bindgen(method, setter = "modifierSymbolLock")]
    fn modifier_symbol_lock_shim(this: &DragEventInit, val: bool);
    #[wasm_bindgen(method, setter = "shiftKey")]
    fn shift_key_shim(this: &DragEventInit, val: bool);
    #[wasm_bindgen(method, setter = "button")]
    fn button_shim(this: &DragEventInit, val: i16);
    #[wasm_bindgen(method, setter = "buttons")]
    fn buttons_shim(this: &DragEventInit, val: u16);
    #[wasm_bindgen(method, setter = "clientX")]
    fn client_x_shim(this: &DragEventInit, val: i32);
    #[wasm_bindgen(method, setter = "clientY")]
    fn client_y_shim(this: &DragEventInit, val: i32);
    #[wasm_bindgen(method, setter = "movementX")]
    fn movement_x_shim(this: &DragEventInit, val: i32);
    #[wasm_bindgen(method, setter = "movementY")]
    fn movement_y_shim(this: &DragEventInit, val: i32);
    #[cfg(feature = "EventTarget")]
    #[wasm_bindgen(method, setter = "relatedTarget")]
    fn related_target_shim(this: &DragEventInit, val: Option<&EventTarget>);
    #[wasm_bindgen(method, setter = "screenX")]
    fn screen_x_shim(this: &DragEventInit, val: i32);
    #[wasm_bindgen(method, setter = "screenY")]
    fn screen_y_shim(this: &DragEventInit, val: i32);
    #[cfg(feature = "DataTransfer")]
    #[wasm_bindgen(method, setter = "dataTransfer")]
    fn data_transfer_shim(this: &DragEventInit, val: Option<&DataTransfer>);
}
impl DragEventInit {
    #[doc = "Construct a new `DragEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn detail(&mut self, val: i32) -> &mut Self {
        self.detail_shim(val);
        self
    }
    #[cfg(feature = "Window")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`, `Window`*"]
    pub fn view(&mut self, val: Option<&Window>) -> &mut Self {
        self.view_shim(val);
        self
    }
    #[doc = "Change the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn alt_key(&mut self, val: bool) -> &mut Self {
        self.alt_key_shim(val);
        self
    }
    #[doc = "Change the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn ctrl_key(&mut self, val: bool) -> &mut Self {
        self.ctrl_key_shim(val);
        self
    }
    #[doc = "Change the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn meta_key(&mut self, val: bool) -> &mut Self {
        self.meta_key_shim(val);
        self
    }
    #[doc = "Change the `modifierAltGraph` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn modifier_alt_graph(&mut self, val: bool) -> &mut Self {
        self.modifier_alt_graph_shim(val);
        self
    }
    #[doc = "Change the `modifierCapsLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn modifier_caps_lock(&mut self, val: bool) -> &mut Self {
        self.modifier_caps_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierFn` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn modifier_fn(&mut self, val: bool) -> &mut Self {
        self.modifier_fn_shim(val);
        self
    }
    #[doc = "Change the `modifierFnLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn modifier_fn_lock(&mut self, val: bool) -> &mut Self {
        self.modifier_fn_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierNumLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn modifier_num_lock(&mut self, val: bool) -> &mut Self {
        self.modifier_num_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierOS` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn modifier_os(&mut self, val: bool) -> &mut Self {
        self.modifier_os_shim(val);
        self
    }
    #[doc = "Change the `modifierScrollLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn modifier_scroll_lock(&mut self, val: bool) -> &mut Self {
        self.modifier_scroll_lock_shim(val);
        self
    }
    #[doc = "Change the `modifierSymbol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn modifier_symbol(&mut self, val: bool) -> &mut Self {
        self.modifier_symbol_shim(val);
        self
    }
    #[doc = "Change the `modifierSymbolLock` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn modifier_symbol_lock(&mut self, val: bool) -> &mut Self {
        self.modifier_symbol_lock_shim(val);
        self
    }
    #[doc = "Change the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn shift_key(&mut self, val: bool) -> &mut Self {
        self.shift_key_shim(val);
        self
    }
    #[doc = "Change the `button` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn button(&mut self, val: i16) -> &mut Self {
        self.button_shim(val);
        self
    }
    #[doc = "Change the `buttons` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn buttons(&mut self, val: u16) -> &mut Self {
        self.buttons_shim(val);
        self
    }
    #[doc = "Change the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn client_x(&mut self, val: i32) -> &mut Self {
        self.client_x_shim(val);
        self
    }
    #[doc = "Change the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn client_y(&mut self, val: i32) -> &mut Self {
        self.client_y_shim(val);
        self
    }
    #[doc = "Change the `movementX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn movement_x(&mut self, val: i32) -> &mut Self {
        self.movement_x_shim(val);
        self
    }
    #[doc = "Change the `movementY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn movement_y(&mut self, val: i32) -> &mut Self {
        self.movement_y_shim(val);
        self
    }
    #[cfg(feature = "EventTarget")]
    #[doc = "Change the `relatedTarget` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`, `EventTarget`*"]
    pub fn related_target(&mut self, val: Option<&EventTarget>) -> &mut Self {
        self.related_target_shim(val);
        self
    }
    #[doc = "Change the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn screen_x(&mut self, val: i32) -> &mut Self {
        self.screen_x_shim(val);
        self
    }
    #[doc = "Change the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEventInit`*"]
    pub fn screen_y(&mut self, val: i32) -> &mut Self {
        self.screen_y_shim(val);
        self
    }
    #[cfg(feature = "DataTransfer")]
    #[doc = "Change the `dataTransfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `DragEventInit`*"]
    pub fn data_transfer(&mut self, val: Option<&DataTransfer>) -> &mut Self {
        self.data_transfer_shim(val);
        self
    }
}
impl Default for DragEventInit {
    fn default() -> Self {
        Self::new()
    }
}
