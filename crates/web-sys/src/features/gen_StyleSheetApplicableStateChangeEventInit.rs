#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = StyleSheetApplicableStateChangeEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StyleSheetApplicableStateChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetApplicableStateChangeEventInit`*"]
    pub type StyleSheetApplicableStateChangeEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &StyleSheetApplicableStateChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &StyleSheetApplicableStateChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &StyleSheetApplicableStateChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &StyleSheetApplicableStateChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &StyleSheetApplicableStateChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &StyleSheetApplicableStateChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "applicable")]
    fn applicable_shim(this: &StyleSheetApplicableStateChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "applicable")]
    fn set_applicable_shim(this: &StyleSheetApplicableStateChangeEventInit, val: bool);
    #[cfg(feature = "CssStyleSheet")]
    #[wasm_bindgen(method, getter = "stylesheet")]
    fn stylesheet_shim(this: &StyleSheetApplicableStateChangeEventInit) -> Option<&CssStyleSheet>;
    #[cfg(feature = "CssStyleSheet")]
    #[wasm_bindgen(method, setter = "stylesheet")]
    fn set_stylesheet_shim(
        this: &StyleSheetApplicableStateChangeEventInit,
        val: Option<&CssStyleSheet>,
    );
}
#[doc = "The trait to access properties on the `StyleSheetApplicableStateChangeEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `StyleSheetApplicableStateChangeEventInit`*"]
pub trait StyleSheetApplicableStateChangeEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetApplicableStateChangeEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetApplicableStateChangeEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetApplicableStateChangeEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `applicable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetApplicableStateChangeEventInit`*"]
    fn applicable(&self) -> bool;
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Get the `stylesheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`, `StyleSheetApplicableStateChangeEventInit`*"]
    fn stylesheet(&self) -> Option<&CssStyleSheet>;
}
impl StyleSheetApplicableStateChangeEventInitGetters for StyleSheetApplicableStateChangeEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn applicable(&self) -> bool {
        self.applicable_shim()
    }
    #[cfg(feature = "CssStyleSheet")]
    fn stylesheet(&self) -> Option<&CssStyleSheet> {
        self.stylesheet_shim()
    }
}
impl StyleSheetApplicableStateChangeEventInit {
    #[doc = "Construct a new `StyleSheetApplicableStateChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetApplicableStateChangeEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetApplicableStateChangeEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetApplicableStateChangeEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetApplicableStateChangeEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `applicable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetApplicableStateChangeEventInit`*"]
    pub fn applicable(&mut self, val: bool) -> &mut Self {
        self.set_applicable_shim(val);
        self
    }
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Change the `stylesheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`, `StyleSheetApplicableStateChangeEventInit`*"]
    pub fn stylesheet(&mut self, val: Option<&CssStyleSheet>) -> &mut Self {
        self.set_stylesheet_shim(val);
        self
    }
}
impl Default for StyleSheetApplicableStateChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
