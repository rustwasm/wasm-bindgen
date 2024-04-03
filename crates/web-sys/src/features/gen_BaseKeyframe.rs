#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BaseKeyframe)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BaseKeyframe` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    pub type BaseKeyframe;
    #[cfg(feature = "CompositeOperation")]
    #[wasm_bindgen(method, setter = "composite")]
    fn composite_shim(this: &BaseKeyframe, val: Option<CompositeOperation>);
    #[wasm_bindgen(method, setter = "easing")]
    fn easing_shim(this: &BaseKeyframe, val: &str);
    #[wasm_bindgen(method, setter = "offset")]
    fn offset_shim(this: &BaseKeyframe, val: Option<f64>);
    #[wasm_bindgen(method, setter = "simulateComputeValuesFailure")]
    fn simulate_compute_values_failure_shim(this: &BaseKeyframe, val: bool);
}
impl BaseKeyframe {
    #[doc = "Construct a new `BaseKeyframe`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`, `CompositeOperation`*"]
    pub fn composite(&mut self, val: Option<CompositeOperation>) -> &mut Self {
        self.composite_shim(val);
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.easing_shim(val);
        self
    }
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    pub fn offset(&mut self, val: Option<f64>) -> &mut Self {
        self.offset_shim(val);
        self
    }
    #[doc = "Change the `simulateComputeValuesFailure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    pub fn simulate_compute_values_failure(&mut self, val: bool) -> &mut Self {
        self.simulate_compute_values_failure_shim(val);
        self
    }
}
impl Default for BaseKeyframe {
    fn default() -> Self {
        Self::new()
    }
}
