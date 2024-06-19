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
    #[doc = "Get the `disableNormalization` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    #[wasm_bindgen(method, getter = "disableNormalization")]
    pub fn get_disable_normalization(this: &PeriodicWaveOptions) -> Option<bool>;
    #[wasm_bindgen(method, setter = "disableNormalization")]
    fn set_disable_normalization(this: &PeriodicWaveOptions, val: bool);
    #[doc = "Get the `imag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    #[wasm_bindgen(method, getter = "imag")]
    pub fn get_imag(this: &PeriodicWaveOptions) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "imag")]
    fn set_imag(this: &PeriodicWaveOptions, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `real` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    #[wasm_bindgen(method, getter = "real")]
    pub fn get_real(this: &PeriodicWaveOptions) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "real")]
    fn set_real(this: &PeriodicWaveOptions, val: &::wasm_bindgen::JsValue);
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
        self.set_disable_normalization(val);
        self
    }
    #[doc = "Change the `imag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    pub fn imag(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_imag(val);
        self
    }
    #[doc = "Change the `real` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    pub fn real(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_real(val);
        self
    }
}
impl Default for PeriodicWaveOptions {
    fn default() -> Self {
        Self::new()
    }
}
