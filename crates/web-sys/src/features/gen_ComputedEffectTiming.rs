#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ComputedEffectTiming)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ComputedEffectTiming` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub type ComputedEffectTiming;
    #[wasm_bindgen(method, setter = "delay")]
    fn delay_shim(this: &ComputedEffectTiming, val: f64);
    #[cfg(feature = "PlaybackDirection")]
    #[wasm_bindgen(method, setter = "direction")]
    fn direction_shim(this: &ComputedEffectTiming, val: PlaybackDirection);
    #[wasm_bindgen(method, setter = "duration")]
    fn duration_shim(this: &ComputedEffectTiming, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "easing")]
    fn easing_shim(this: &ComputedEffectTiming, val: &str);
    #[wasm_bindgen(method, setter = "endDelay")]
    fn end_delay_shim(this: &ComputedEffectTiming, val: f64);
    #[cfg(feature = "FillMode")]
    #[wasm_bindgen(method, setter = "fill")]
    fn fill_shim(this: &ComputedEffectTiming, val: FillMode);
    #[wasm_bindgen(method, setter = "iterationStart")]
    fn iteration_start_shim(this: &ComputedEffectTiming, val: f64);
    #[wasm_bindgen(method, setter = "iterations")]
    fn iterations_shim(this: &ComputedEffectTiming, val: f64);
    #[wasm_bindgen(method, setter = "activeDuration")]
    fn active_duration_shim(this: &ComputedEffectTiming, val: f64);
    #[wasm_bindgen(method, setter = "currentIteration")]
    fn current_iteration_shim(this: &ComputedEffectTiming, val: Option<f64>);
    #[wasm_bindgen(method, setter = "endTime")]
    fn end_time_shim(this: &ComputedEffectTiming, val: f64);
    #[wasm_bindgen(method, setter = "localTime")]
    fn local_time_shim(this: &ComputedEffectTiming, val: Option<f64>);
    #[wasm_bindgen(method, setter = "progress")]
    fn progress_shim(this: &ComputedEffectTiming, val: Option<f64>);
}
impl ComputedEffectTiming {
    #[doc = "Construct a new `ComputedEffectTiming`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `delay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn delay(&mut self, val: f64) -> &mut Self {
        self.delay_shim(val);
        self
    }
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`, `PlaybackDirection`*"]
    pub fn direction(&mut self, val: PlaybackDirection) -> &mut Self {
        self.direction_shim(val);
        self
    }
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn duration(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.duration_shim(val);
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.easing_shim(val);
        self
    }
    #[doc = "Change the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn end_delay(&mut self, val: f64) -> &mut Self {
        self.end_delay_shim(val);
        self
    }
    #[cfg(feature = "FillMode")]
    #[doc = "Change the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`, `FillMode`*"]
    pub fn fill(&mut self, val: FillMode) -> &mut Self {
        self.fill_shim(val);
        self
    }
    #[doc = "Change the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn iteration_start(&mut self, val: f64) -> &mut Self {
        self.iteration_start_shim(val);
        self
    }
    #[doc = "Change the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn iterations(&mut self, val: f64) -> &mut Self {
        self.iterations_shim(val);
        self
    }
    #[doc = "Change the `activeDuration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn active_duration(&mut self, val: f64) -> &mut Self {
        self.active_duration_shim(val);
        self
    }
    #[doc = "Change the `currentIteration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn current_iteration(&mut self, val: Option<f64>) -> &mut Self {
        self.current_iteration_shim(val);
        self
    }
    #[doc = "Change the `endTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn end_time(&mut self, val: f64) -> &mut Self {
        self.end_time_shim(val);
        self
    }
    #[doc = "Change the `localTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn local_time(&mut self, val: Option<f64>) -> &mut Self {
        self.local_time_shim(val);
        self
    }
    #[doc = "Change the `progress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn progress(&mut self, val: Option<f64>) -> &mut Self {
        self.progress_shim(val);
        self
    }
}
impl Default for ComputedEffectTiming {
    fn default() -> Self {
        Self::new()
    }
}
