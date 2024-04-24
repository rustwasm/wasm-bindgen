#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CaretStateChangedEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CaretStateChangedEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub type CaretStateChangedEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &CaretStateChangedEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &CaretStateChangedEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &CaretStateChangedEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &CaretStateChangedEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &CaretStateChangedEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &CaretStateChangedEventInit, val: bool);
    #[cfg(feature = "DomRectReadOnly")]
    #[wasm_bindgen(method, getter = "boundingClientRect")]
    fn bounding_client_rect_shim(this: &CaretStateChangedEventInit) -> Option<DomRectReadOnly>;
    #[cfg(feature = "DomRectReadOnly")]
    #[wasm_bindgen(method, setter = "boundingClientRect")]
    fn set_bounding_client_rect_shim(
        this: &CaretStateChangedEventInit,
        val: Option<&DomRectReadOnly>,
    );
    #[wasm_bindgen(method, getter = "caretVisible")]
    fn caret_visible_shim(this: &CaretStateChangedEventInit) -> bool;
    #[wasm_bindgen(method, setter = "caretVisible")]
    fn set_caret_visible_shim(this: &CaretStateChangedEventInit, val: bool);
    #[wasm_bindgen(method, getter = "caretVisuallyVisible")]
    fn caret_visually_visible_shim(this: &CaretStateChangedEventInit) -> bool;
    #[wasm_bindgen(method, setter = "caretVisuallyVisible")]
    fn set_caret_visually_visible_shim(this: &CaretStateChangedEventInit, val: bool);
    #[wasm_bindgen(method, getter = "collapsed")]
    fn collapsed_shim(this: &CaretStateChangedEventInit) -> bool;
    #[wasm_bindgen(method, setter = "collapsed")]
    fn set_collapsed_shim(this: &CaretStateChangedEventInit, val: bool);
    #[cfg(feature = "CaretChangedReason")]
    #[wasm_bindgen(method, getter = "reason")]
    fn reason_shim(this: &CaretStateChangedEventInit) -> CaretChangedReason;
    #[cfg(feature = "CaretChangedReason")]
    #[wasm_bindgen(method, setter = "reason")]
    fn set_reason_shim(this: &CaretStateChangedEventInit, val: CaretChangedReason);
    #[wasm_bindgen(method, getter = "selectedTextContent")]
    fn selected_text_content_shim(this: &CaretStateChangedEventInit) -> String;
    #[wasm_bindgen(method, setter = "selectedTextContent")]
    fn set_selected_text_content_shim(this: &CaretStateChangedEventInit, val: &str);
    #[wasm_bindgen(method, getter = "selectionEditable")]
    fn selection_editable_shim(this: &CaretStateChangedEventInit) -> bool;
    #[wasm_bindgen(method, setter = "selectionEditable")]
    fn set_selection_editable_shim(this: &CaretStateChangedEventInit, val: bool);
    #[wasm_bindgen(method, getter = "selectionVisible")]
    fn selection_visible_shim(this: &CaretStateChangedEventInit) -> bool;
    #[wasm_bindgen(method, setter = "selectionVisible")]
    fn set_selection_visible_shim(this: &CaretStateChangedEventInit, val: bool);
}
#[doc = "The trait to access properties on the `CaretStateChangedEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
pub trait CaretStateChangedEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "Get the `boundingClientRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`, `DomRectReadOnly`*"]
    fn bounding_client_rect(&self) -> Option<DomRectReadOnly>;
    #[doc = "Get the `caretVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    fn caret_visible(&self) -> bool;
    #[doc = "Get the `caretVisuallyVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    fn caret_visually_visible(&self) -> bool;
    #[doc = "Get the `collapsed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    fn collapsed(&self) -> bool;
    #[cfg(feature = "CaretChangedReason")]
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretChangedReason`, `CaretStateChangedEventInit`*"]
    fn reason(&self) -> CaretChangedReason;
    #[doc = "Get the `selectedTextContent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    fn selected_text_content(&self) -> String;
    #[doc = "Get the `selectionEditable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    fn selection_editable(&self) -> bool;
    #[doc = "Get the `selectionVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    fn selection_visible(&self) -> bool;
}
impl CaretStateChangedEventInitGetters for CaretStateChangedEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "DomRectReadOnly")]
    fn bounding_client_rect(&self) -> Option<DomRectReadOnly> {
        self.bounding_client_rect_shim()
    }
    fn caret_visible(&self) -> bool {
        self.caret_visible_shim()
    }
    fn caret_visually_visible(&self) -> bool {
        self.caret_visually_visible_shim()
    }
    fn collapsed(&self) -> bool {
        self.collapsed_shim()
    }
    #[cfg(feature = "CaretChangedReason")]
    fn reason(&self) -> CaretChangedReason {
        self.reason_shim()
    }
    fn selected_text_content(&self) -> String {
        self.selected_text_content_shim()
    }
    fn selection_editable(&self) -> bool {
        self.selection_editable_shim()
    }
    fn selection_visible(&self) -> bool {
        self.selection_visible_shim()
    }
}
impl CaretStateChangedEventInit {
    #[doc = "Construct a new `CaretStateChangedEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "Change the `boundingClientRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`, `DomRectReadOnly`*"]
    pub fn bounding_client_rect(&mut self, val: Option<&DomRectReadOnly>) -> &mut Self {
        self.set_bounding_client_rect_shim(val);
        self
    }
    #[doc = "Change the `caretVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn caret_visible(&mut self, val: bool) -> &mut Self {
        self.set_caret_visible_shim(val);
        self
    }
    #[doc = "Change the `caretVisuallyVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn caret_visually_visible(&mut self, val: bool) -> &mut Self {
        self.set_caret_visually_visible_shim(val);
        self
    }
    #[doc = "Change the `collapsed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn collapsed(&mut self, val: bool) -> &mut Self {
        self.set_collapsed_shim(val);
        self
    }
    #[cfg(feature = "CaretChangedReason")]
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretChangedReason`, `CaretStateChangedEventInit`*"]
    pub fn reason(&mut self, val: CaretChangedReason) -> &mut Self {
        self.set_reason_shim(val);
        self
    }
    #[doc = "Change the `selectedTextContent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn selected_text_content(&mut self, val: &str) -> &mut Self {
        self.set_selected_text_content_shim(val);
        self
    }
    #[doc = "Change the `selectionEditable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn selection_editable(&mut self, val: bool) -> &mut Self {
        self.set_selection_editable_shim(val);
        self
    }
    #[doc = "Change the `selectionVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn selection_visible(&mut self, val: bool) -> &mut Self {
        self.set_selection_visible_shim(val);
        self
    }
}
impl Default for CaretStateChangedEventInit {
    fn default() -> Self {
        Self::new()
    }
}
