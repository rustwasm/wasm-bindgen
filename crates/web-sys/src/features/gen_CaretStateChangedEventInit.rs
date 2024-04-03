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
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &CaretStateChangedEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &CaretStateChangedEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &CaretStateChangedEventInit, val: bool);
    #[cfg(feature = "DomRectReadOnly")]
    #[wasm_bindgen(method, setter = "boundingClientRect")]
    fn bounding_client_rect_shim(this: &CaretStateChangedEventInit, val: Option<&DomRectReadOnly>);
    #[wasm_bindgen(method, setter = "caretVisible")]
    fn caret_visible_shim(this: &CaretStateChangedEventInit, val: bool);
    #[wasm_bindgen(method, setter = "caretVisuallyVisible")]
    fn caret_visually_visible_shim(this: &CaretStateChangedEventInit, val: bool);
    #[wasm_bindgen(method, setter = "collapsed")]
    fn collapsed_shim(this: &CaretStateChangedEventInit, val: bool);
    #[cfg(feature = "CaretChangedReason")]
    #[wasm_bindgen(method, setter = "reason")]
    fn reason_shim(this: &CaretStateChangedEventInit, val: CaretChangedReason);
    #[wasm_bindgen(method, setter = "selectedTextContent")]
    fn selected_text_content_shim(this: &CaretStateChangedEventInit, val: &str);
    #[wasm_bindgen(method, setter = "selectionEditable")]
    fn selection_editable_shim(this: &CaretStateChangedEventInit, val: bool);
    #[wasm_bindgen(method, setter = "selectionVisible")]
    fn selection_visible_shim(this: &CaretStateChangedEventInit, val: bool);
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
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "Change the `boundingClientRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`, `DomRectReadOnly`*"]
    pub fn bounding_client_rect(&mut self, val: Option<&DomRectReadOnly>) -> &mut Self {
        self.bounding_client_rect_shim(val);
        self
    }
    #[doc = "Change the `caretVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn caret_visible(&mut self, val: bool) -> &mut Self {
        self.caret_visible_shim(val);
        self
    }
    #[doc = "Change the `caretVisuallyVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn caret_visually_visible(&mut self, val: bool) -> &mut Self {
        self.caret_visually_visible_shim(val);
        self
    }
    #[doc = "Change the `collapsed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn collapsed(&mut self, val: bool) -> &mut Self {
        self.collapsed_shim(val);
        self
    }
    #[cfg(feature = "CaretChangedReason")]
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretChangedReason`, `CaretStateChangedEventInit`*"]
    pub fn reason(&mut self, val: CaretChangedReason) -> &mut Self {
        self.reason_shim(val);
        self
    }
    #[doc = "Change the `selectedTextContent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn selected_text_content(&mut self, val: &str) -> &mut Self {
        self.selected_text_content_shim(val);
        self
    }
    #[doc = "Change the `selectionEditable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn selection_editable(&mut self, val: bool) -> &mut Self {
        self.selection_editable_shim(val);
        self
    }
    #[doc = "Change the `selectionVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretStateChangedEventInit`*"]
    pub fn selection_visible(&mut self, val: bool) -> &mut Self {
        self.selection_visible_shim(val);
        self
    }
}
impl Default for CaretStateChangedEventInit {
    fn default() -> Self {
        Self::new()
    }
}
