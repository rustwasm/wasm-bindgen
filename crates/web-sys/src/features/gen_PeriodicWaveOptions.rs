#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PeriodicWaveOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PeriodicWaveOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    pub type PeriodicWaveOptions;
    #[wasm_bindgen(method, getter = "disableNormalization")]
    fn disable_normalization_shim(this: &PeriodicWaveOptions) -> bool;
    #[wasm_bindgen(method, setter = "disableNormalization")]
    fn set_disable_normalization_shim(this: &PeriodicWaveOptions, val: bool);
    #[wasm_bindgen(method, getter = "imag")]
    fn imag_shim(this: &PeriodicWaveOptions) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "imag")]
    fn set_imag_shim(this: &PeriodicWaveOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "real")]
    fn real_shim(this: &PeriodicWaveOptions) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "real")]
    fn set_real_shim(this: &PeriodicWaveOptions, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `PeriodicWaveOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
pub trait PeriodicWaveOptionsGetters {
    #[doc = "Get the `disableNormalization` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    fn disable_normalization(&self) -> bool;
    #[doc = "Get the `imag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    fn imag(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `real` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    fn real(&self) -> &::wasm_bindgen::JsValue;
}
impl PeriodicWaveOptionsGetters for PeriodicWaveOptions {
    fn disable_normalization(&self) -> bool {
        self.disable_normalization_shim()
    }
    fn imag(&self) -> &::wasm_bindgen::JsValue {
        self.imag_shim()
    }
    fn real(&self) -> &::wasm_bindgen::JsValue {
        self.real_shim()
    }
}
impl PeriodicWaveOptions {
    #[doc = "Construct a new `PeriodicWaveOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `disableNormalization` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    pub fn disable_normalization(&mut self, val: bool) -> &mut Self {
        self.set_disable_normalization_shim(val);
        self
    }
    #[doc = "Change the `imag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    pub fn imag(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_imag_shim(val);
        self
    }
    #[doc = "Change the `real` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    pub fn real(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_real_shim(val);
        self
    }
}
impl Default for PeriodicWaveOptions {
    fn default() -> Self {
        Self::new()
    }
}
