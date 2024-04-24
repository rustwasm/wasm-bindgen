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
    #[wasm_bindgen(method, getter = "composite")]
    fn composite_shim(this: &BaseKeyframe) -> Option<CompositeOperation>;
    #[cfg(feature = "CompositeOperation")]
    #[wasm_bindgen(method, setter = "composite")]
    fn set_composite_shim(this: &BaseKeyframe, val: Option<CompositeOperation>);
    #[wasm_bindgen(method, getter = "easing")]
    fn easing_shim(this: &BaseKeyframe) -> String;
    #[wasm_bindgen(method, setter = "easing")]
    fn set_easing_shim(this: &BaseKeyframe, val: &str);
    #[wasm_bindgen(method, getter = "offset")]
    fn offset_shim(this: &BaseKeyframe) -> Option<f64>;
    #[wasm_bindgen(method, setter = "offset")]
    fn set_offset_shim(this: &BaseKeyframe, val: Option<f64>);
    #[wasm_bindgen(method, getter = "simulateComputeValuesFailure")]
    fn simulate_compute_values_failure_shim(this: &BaseKeyframe) -> bool;
    #[wasm_bindgen(method, setter = "simulateComputeValuesFailure")]
    fn set_simulate_compute_values_failure_shim(this: &BaseKeyframe, val: bool);
}
#[doc = "The trait to access properties on the `BaseKeyframe` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
pub trait BaseKeyframeGetters {
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Get the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`, `CompositeOperation`*"]
    fn composite(&self) -> Option<CompositeOperation>;
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    fn easing(&self) -> String;
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    fn offset(&self) -> Option<f64>;
    #[doc = "Get the `simulateComputeValuesFailure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    fn simulate_compute_values_failure(&self) -> bool;
}
impl BaseKeyframeGetters for BaseKeyframe {
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
        self.set_composite_shim(val);
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.set_easing_shim(val);
        self
    }
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    pub fn offset(&mut self, val: Option<f64>) -> &mut Self {
        self.set_offset_shim(val);
        self
    }
    #[doc = "Change the `simulateComputeValuesFailure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
    pub fn simulate_compute_values_failure(&mut self, val: bool) -> &mut Self {
        self.set_simulate_compute_values_failure_shim(val);
        self
    }
}
impl Default for BaseKeyframe {
    fn default() -> Self {
        Self::new()
    }
}
