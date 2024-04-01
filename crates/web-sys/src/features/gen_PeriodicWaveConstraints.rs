#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PeriodicWaveConstraints)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PeriodicWaveConstraints` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveConstraints`*"]
    pub type PeriodicWaveConstraints;
    #[wasm_bindgen(method, setter = "disableNormalization")]
    fn disable_normalization_shim(this: &PeriodicWaveConstraints, val: bool);
}
impl PeriodicWaveConstraints {
    #[doc = "Construct a new `PeriodicWaveConstraints`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveConstraints`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `disableNormalization` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveConstraints`*"]
    pub fn disable_normalization(&mut self, val: bool) -> &mut Self {
        self.disable_normalization_shim(val);
        self
    }
}
impl Default for PeriodicWaveConstraints {
    fn default() -> Self {
        Self::new()
    }
}
