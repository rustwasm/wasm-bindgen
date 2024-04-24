#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = StyleSheetChangeEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StyleSheetChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    pub type StyleSheetChangeEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &StyleSheetChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &StyleSheetChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &StyleSheetChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &StyleSheetChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &StyleSheetChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &StyleSheetChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "documentSheet")]
    fn document_sheet_shim(this: &StyleSheetChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "documentSheet")]
    fn set_document_sheet_shim(this: &StyleSheetChangeEventInit, val: bool);
    #[cfg(feature = "CssStyleSheet")]
    #[wasm_bindgen(method, getter = "stylesheet")]
    fn stylesheet_shim(this: &StyleSheetChangeEventInit) -> Option<CssStyleSheet>;
    #[cfg(feature = "CssStyleSheet")]
    #[wasm_bindgen(method, setter = "stylesheet")]
    fn set_stylesheet_shim(this: &StyleSheetChangeEventInit, val: Option<&CssStyleSheet>);
}
#[doc = "The trait to access properties on the `StyleSheetChangeEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
pub trait StyleSheetChangeEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `documentSheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    fn document_sheet(&self) -> bool;
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Get the `stylesheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`, `StyleSheetChangeEventInit`*"]
    fn stylesheet(&self) -> Option<CssStyleSheet>;
}
impl StyleSheetChangeEventInitGetters for StyleSheetChangeEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn document_sheet(&self) -> bool {
        self.document_sheet_shim()
    }
    #[cfg(feature = "CssStyleSheet")]
    fn stylesheet(&self) -> Option<CssStyleSheet> {
        self.stylesheet_shim()
    }
}
impl StyleSheetChangeEventInit {
    #[doc = "Construct a new `StyleSheetChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `documentSheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    pub fn document_sheet(&mut self, val: bool) -> &mut Self {
        self.set_document_sheet_shim(val);
        self
    }
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Change the `stylesheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`, `StyleSheetChangeEventInit`*"]
    pub fn stylesheet(&mut self, val: Option<&CssStyleSheet>) -> &mut Self {
        self.set_stylesheet_shim(val);
        self
    }
}
impl Default for StyleSheetChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
