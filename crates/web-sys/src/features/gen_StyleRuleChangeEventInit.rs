#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = StyleRuleChangeEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StyleRuleChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
    pub type StyleRuleChangeEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &StyleRuleChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &StyleRuleChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &StyleRuleChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &StyleRuleChangeEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &StyleRuleChangeEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &StyleRuleChangeEventInit, val: bool);
    #[cfg(feature = "CssRule")]
    #[wasm_bindgen(method, getter = "rule")]
    fn rule_shim(this: &StyleRuleChangeEventInit) -> Option<&CssRule>;
    #[cfg(feature = "CssRule")]
    #[wasm_bindgen(method, setter = "rule")]
    fn set_rule_shim(this: &StyleRuleChangeEventInit, val: Option<&CssRule>);
    #[cfg(feature = "CssStyleSheet")]
    #[wasm_bindgen(method, getter = "stylesheet")]
    fn stylesheet_shim(this: &StyleRuleChangeEventInit) -> Option<&CssStyleSheet>;
    #[cfg(feature = "CssStyleSheet")]
    #[wasm_bindgen(method, setter = "stylesheet")]
    fn set_stylesheet_shim(this: &StyleRuleChangeEventInit, val: Option<&CssStyleSheet>);
}
#[doc = "The trait to access properties on the `StyleRuleChangeEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
pub trait StyleRuleChangeEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "CssRule")]
    #[doc = "Get the `rule` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`, `StyleRuleChangeEventInit`*"]
    fn rule(&self) -> Option<&CssRule>;
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Get the `stylesheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`, `StyleRuleChangeEventInit`*"]
    fn stylesheet(&self) -> Option<&CssStyleSheet>;
}
impl StyleRuleChangeEventInitGetters for StyleRuleChangeEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "CssRule")]
    fn rule(&self) -> Option<&CssRule> {
        self.rule_shim()
    }
    #[cfg(feature = "CssStyleSheet")]
    fn stylesheet(&self) -> Option<&CssStyleSheet> {
        self.stylesheet_shim()
    }
}
impl StyleRuleChangeEventInit {
    #[doc = "Construct a new `StyleRuleChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "CssRule")]
    #[doc = "Change the `rule` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`, `StyleRuleChangeEventInit`*"]
    pub fn rule(&mut self, val: Option<&CssRule>) -> &mut Self {
        self.set_rule_shim(val);
        self
    }
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Change the `stylesheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`, `StyleRuleChangeEventInit`*"]
    pub fn stylesheet(&mut self, val: Option<&CssStyleSheet>) -> &mut Self {
        self.set_stylesheet_shim(val);
        self
    }
}
impl Default for StyleRuleChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
