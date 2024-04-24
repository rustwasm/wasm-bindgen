#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BaseComputedKeyframe)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BaseComputedKeyframe` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub type BaseComputedKeyframe;
    #[cfg(feature = "CompositeOperation")]
    #[wasm_bindgen(method, getter = "composite")]
    fn composite_shim(this: &BaseComputedKeyframe) -> Option<CompositeOperation>;
    #[cfg(feature = "CompositeOperation")]
    #[wasm_bindgen(method, setter = "composite")]
    fn set_composite_shim(this: &BaseComputedKeyframe, val: Option<CompositeOperation>);
    #[wasm_bindgen(method, getter = "easing")]
    fn easing_shim(this: &BaseComputedKeyframe) -> String;
    #[wasm_bindgen(method, setter = "easing")]
    fn set_easing_shim(this: &BaseComputedKeyframe, val: &str);
    #[wasm_bindgen(method, getter = "offset")]
    fn offset_shim(this: &BaseComputedKeyframe) -> Option<f64>;
    #[wasm_bindgen(method, setter = "offset")]
    fn set_offset_shim(this: &BaseComputedKeyframe, val: Option<f64>);
    #[wasm_bindgen(method, getter = "simulateComputeValuesFailure")]
    fn simulate_compute_values_failure_shim(this: &BaseComputedKeyframe) -> bool;
    #[wasm_bindgen(method, setter = "simulateComputeValuesFailure")]
    fn set_simulate_compute_values_failure_shim(this: &BaseComputedKeyframe, val: bool);
    #[wasm_bindgen(method, getter = "computedOffset")]
    fn computed_offset_shim(this: &BaseComputedKeyframe) -> f64;
    #[wasm_bindgen(method, setter = "computedOffset")]
    fn set_computed_offset_shim(this: &BaseComputedKeyframe, val: f64);
}
#[doc = "The trait to access properties on the `BaseComputedKeyframe` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
pub trait BaseComputedKeyframeGetters {
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Get the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`, `CompositeOperation`*"]
    fn composite(&self) -> Option<CompositeOperation>;
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    fn easing(&self) -> String;
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    fn offset(&self) -> Option<f64>;
    #[doc = "Get the `simulateComputeValuesFailure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    fn simulate_compute_values_failure(&self) -> bool;
    #[doc = "Get the `computedOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    fn computed_offset(&self) -> f64;
}
impl BaseComputedKeyframeGetters for BaseComputedKeyframe {
    #[cfg(feature = "CompositeOperation")]
    fn composite(&self) -> Option<CompositeOperation> {
        self.composite_shim()
    }
    fn easing(&self) -> String {
        self.easing_shim()
    }
    fn offset(&self) -> Option<f64> {
        self.offset_shim()
    }
    fn simulate_compute_values_failure(&self) -> bool {
        self.simulate_compute_values_failure_shim()
    }
    fn computed_offset(&self) -> f64 {
        self.computed_offset_shim()
    }
}
impl BaseComputedKeyframe {
    #[doc = "Construct a new `BaseComputedKeyframe`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`, `CompositeOperation`*"]
    pub fn composite(&mut self, val: Option<CompositeOperation>) -> &mut Self {
        self.set_composite_shim(val);
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.set_easing_shim(val);
        self
    }
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn offset(&mut self, val: Option<f64>) -> &mut Self {
        self.set_offset_shim(val);
        self
    }
    #[doc = "Change the `simulateComputeValuesFailure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn simulate_compute_values_failure(&mut self, val: bool) -> &mut Self {
        self.set_simulate_compute_values_failure_shim(val);
        self
    }
    #[doc = "Change the `computedOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn computed_offset(&mut self, val: f64) -> &mut Self {
        self.set_computed_offset_shim(val);
        self
    }
}
impl Default for BaseComputedKeyframe {
    fn default() -> Self {
        Self::new()
    }
}
