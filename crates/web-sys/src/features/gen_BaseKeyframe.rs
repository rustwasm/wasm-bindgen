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
    #[doc = "Get the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`, `CompositeOperation`*"]
    #[wasm_bindgen(method, getter = "composite")]
    pub fn get_composite(this: &BaseKeyframe) -> Option<CompositeOperation>;
    #[cfg(feature = "CompositeOperation")]
    #[wasm_bindgen(method, setter = "composite")]
    fn set_composite(this: &BaseKeyframe, val: Option<CompositeOperation>);
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    #[wasm_bindgen(method, getter = "easing")]
    pub fn get_easing(this: &BaseKeyframe) -> Option<String>;
    #[wasm_bindgen(method, setter = "easing")]
    fn set_easing(this: &BaseKeyframe, val: &str);
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    #[wasm_bindgen(method, getter = "offset")]
    pub fn get_offset(this: &BaseKeyframe) -> Option<f64>;
    #[wasm_bindgen(method, setter = "offset")]
    fn set_offset(this: &BaseKeyframe, val: Option<f64>);
    #[doc = "Get the `simulateComputeValuesFailure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    #[wasm_bindgen(method, getter = "simulateComputeValuesFailure")]
    pub fn get_simulate_compute_values_failure(this: &BaseKeyframe) -> Option<bool>;
    #[wasm_bindgen(method, setter = "simulateComputeValuesFailure")]
    fn set_simulate_compute_values_failure(this: &BaseKeyframe, val: bool);
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
        self.set_composite(val);
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.set_easing(val);
        self
    }
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    pub fn offset(&mut self, val: Option<f64>) -> &mut Self {
        self.set_offset(val);
        self
    }
    #[doc = "Change the `simulateComputeValuesFailure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    pub fn simulate_compute_values_failure(&mut self, val: bool) -> &mut Self {
        self.set_simulate_compute_values_failure(val);
        self
    }
}
impl Default for BaseKeyframe {
    fn default() -> Self {
        Self::new()
    }
}
