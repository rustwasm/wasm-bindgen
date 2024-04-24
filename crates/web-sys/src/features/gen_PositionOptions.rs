#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PositionOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PositionOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
    pub type PositionOptions;
    #[wasm_bindgen(method, getter = "enableHighAccuracy")]
    fn enable_high_accuracy_shim(this: &PositionOptions) -> bool;
    #[wasm_bindgen(method, setter = "enableHighAccuracy")]
    fn set_enable_high_accuracy_shim(this: &PositionOptions, val: bool);
    #[wasm_bindgen(method, getter = "maximumAge")]
    fn maximum_age_shim(this: &PositionOptions) -> u32;
    #[wasm_bindgen(method, setter = "maximumAge")]
    fn set_maximum_age_shim(this: &PositionOptions, val: u32);
    #[wasm_bindgen(method, getter = "timeout")]
    fn timeout_shim(this: &PositionOptions) -> u32;
    #[wasm_bindgen(method, setter = "timeout")]
    fn set_timeout_shim(this: &PositionOptions, val: u32);
}
#[doc = "The trait to access properties on the `PositionOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
pub trait PositionOptionsGetters {
    #[doc = "Get the `enableHighAccuracy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
    fn enable_high_accuracy(&self) -> bool;
    #[doc = "Get the `maximumAge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
    fn maximum_age(&self) -> u32;
    #[doc = "Get the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
    fn timeout(&self) -> u32;
}
impl PositionOptionsGetters for PositionOptions {
    fn enable_high_accuracy(&self) -> bool {
        self.enable_high_accuracy_shim()
    }
    fn maximum_age(&self) -> u32 {
        self.maximum_age_shim()
    }
    fn timeout(&self) -> u32 {
        self.timeout_shim()
    }
}
impl PositionOptions {
    #[doc = "Construct a new `PositionOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `enableHighAccuracy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
    pub fn enable_high_accuracy(&mut self, val: bool) -> &mut Self {
        self.set_enable_high_accuracy_shim(val);
        self
    }
    #[doc = "Change the `maximumAge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
    pub fn maximum_age(&mut self, val: u32) -> &mut Self {
        self.set_maximum_age_shim(val);
        self
    }
    #[doc = "Change the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
    pub fn timeout(&mut self, val: u32) -> &mut Self {
        self.set_timeout_shim(val);
        self
    }
}
impl Default for PositionOptions {
    fn default() -> Self {
        Self::new()
    }
}
