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
    #[wasm_bindgen(method, getter = "delay")]
    fn delay_shim(this: &ComputedEffectTiming) -> f64;
    #[wasm_bindgen(method, setter = "delay")]
    fn set_delay_shim(this: &ComputedEffectTiming, val: f64);
    #[cfg(feature = "PlaybackDirection")]
    #[wasm_bindgen(method, getter = "direction")]
    fn direction_shim(this: &ComputedEffectTiming) -> PlaybackDirection;
    #[cfg(feature = "PlaybackDirection")]
    #[wasm_bindgen(method, setter = "direction")]
    fn set_direction_shim(this: &ComputedEffectTiming, val: PlaybackDirection);
    #[wasm_bindgen(method, getter = "duration")]
    fn duration_shim(this: &ComputedEffectTiming) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "duration")]
    fn set_duration_shim(this: &ComputedEffectTiming, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "easing")]
    fn easing_shim(this: &ComputedEffectTiming) -> String;
    #[wasm_bindgen(method, setter = "easing")]
    fn set_easing_shim(this: &ComputedEffectTiming, val: &str);
    #[wasm_bindgen(method, getter = "endDelay")]
    fn end_delay_shim(this: &ComputedEffectTiming) -> f64;
    #[wasm_bindgen(method, setter = "endDelay")]
    fn set_end_delay_shim(this: &ComputedEffectTiming, val: f64);
    #[cfg(feature = "FillMode")]
    #[wasm_bindgen(method, getter = "fill")]
    fn fill_shim(this: &ComputedEffectTiming) -> FillMode;
    #[cfg(feature = "FillMode")]
    #[wasm_bindgen(method, setter = "fill")]
    fn set_fill_shim(this: &ComputedEffectTiming, val: FillMode);
    #[wasm_bindgen(method, getter = "iterationStart")]
    fn iteration_start_shim(this: &ComputedEffectTiming) -> f64;
    #[wasm_bindgen(method, setter = "iterationStart")]
    fn set_iteration_start_shim(this: &ComputedEffectTiming, val: f64);
    #[wasm_bindgen(method, getter = "iterations")]
    fn iterations_shim(this: &ComputedEffectTiming) -> f64;
    #[wasm_bindgen(method, setter = "iterations")]
    fn set_iterations_shim(this: &ComputedEffectTiming, val: f64);
    #[wasm_bindgen(method, getter = "activeDuration")]
    fn active_duration_shim(this: &ComputedEffectTiming) -> f64;
    #[wasm_bindgen(method, setter = "activeDuration")]
    fn set_active_duration_shim(this: &ComputedEffectTiming, val: f64);
    #[wasm_bindgen(method, getter = "currentIteration")]
    fn current_iteration_shim(this: &ComputedEffectTiming) -> Option<f64>;
    #[wasm_bindgen(method, setter = "currentIteration")]
    fn set_current_iteration_shim(this: &ComputedEffectTiming, val: Option<f64>);
    #[wasm_bindgen(method, getter = "endTime")]
    fn end_time_shim(this: &ComputedEffectTiming) -> f64;
    #[wasm_bindgen(method, setter = "endTime")]
    fn set_end_time_shim(this: &ComputedEffectTiming, val: f64);
    #[wasm_bindgen(method, getter = "localTime")]
    fn local_time_shim(this: &ComputedEffectTiming) -> Option<f64>;
    #[wasm_bindgen(method, setter = "localTime")]
    fn set_local_time_shim(this: &ComputedEffectTiming, val: Option<f64>);
    #[wasm_bindgen(method, getter = "progress")]
    fn progress_shim(this: &ComputedEffectTiming) -> Option<f64>;
    #[wasm_bindgen(method, setter = "progress")]
    fn set_progress_shim(this: &ComputedEffectTiming, val: Option<f64>);
}
#[doc = "The trait to access properties on the `ComputedEffectTiming` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
pub trait ComputedEffectTimingGetters {
    #[doc = "Get the `delay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    fn delay(&self) -> f64;
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Get the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`, `PlaybackDirection`*"]
    fn direction(&self) -> PlaybackDirection;
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    fn duration(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    fn easing(&self) -> String;
    #[doc = "Get the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    fn end_delay(&self) -> f64;
    #[cfg(feature = "FillMode")]
    #[doc = "Get the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`, `FillMode`*"]
    fn fill(&self) -> FillMode;
    #[doc = "Get the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    fn iteration_start(&self) -> f64;
    #[doc = "Get the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    fn iterations(&self) -> f64;
    #[doc = "Get the `activeDuration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    fn active_duration(&self) -> f64;
    #[doc = "Get the `currentIteration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    fn current_iteration(&self) -> Option<f64>;
    #[doc = "Get the `endTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    fn end_time(&self) -> f64;
    #[doc = "Get the `localTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    fn local_time(&self) -> Option<f64>;
    #[doc = "Get the `progress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    fn progress(&self) -> Option<f64>;
}
impl ComputedEffectTimingGetters for ComputedEffectTiming {
    fn delay(&self) -> f64 {
        self.delay_shim()
    }
    #[cfg(feature = "PlaybackDirection")]
    fn direction(&self) -> PlaybackDirection {
        self.direction_shim()
    }
    fn duration(&self) -> ::wasm_bindgen::JsValue {
        self.duration_shim()
    }
    fn easing(&self) -> String {
        self.easing_shim()
    }
    fn end_delay(&self) -> f64 {
        self.end_delay_shim()
    }
    #[cfg(feature = "FillMode")]
    fn fill(&self) -> FillMode {
        self.fill_shim()
    }
    fn iteration_start(&self) -> f64 {
        self.iteration_start_shim()
    }
    fn iterations(&self) -> f64 {
        self.iterations_shim()
    }
    fn active_duration(&self) -> f64 {
        self.active_duration_shim()
    }
    fn current_iteration(&self) -> Option<f64> {
        self.current_iteration_shim()
    }
    fn end_time(&self) -> f64 {
        self.end_time_shim()
    }
    fn local_time(&self) -> Option<f64> {
        self.local_time_shim()
    }
    fn progress(&self) -> Option<f64> {
        self.progress_shim()
    }
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
        self.set_delay_shim(val);
        self
    }
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`, `PlaybackDirection`*"]
    pub fn direction(&mut self, val: PlaybackDirection) -> &mut Self {
        self.set_direction_shim(val);
        self
    }
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn duration(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_duration_shim(val);
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.set_easing_shim(val);
        self
    }
    #[doc = "Change the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn end_delay(&mut self, val: f64) -> &mut Self {
        self.set_end_delay_shim(val);
        self
    }
    #[cfg(feature = "FillMode")]
    #[doc = "Change the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`, `FillMode`*"]
    pub fn fill(&mut self, val: FillMode) -> &mut Self {
        self.set_fill_shim(val);
        self
    }
    #[doc = "Change the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn iteration_start(&mut self, val: f64) -> &mut Self {
        self.set_iteration_start_shim(val);
        self
    }
    #[doc = "Change the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn iterations(&mut self, val: f64) -> &mut Self {
        self.set_iterations_shim(val);
        self
    }
    #[doc = "Change the `activeDuration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn active_duration(&mut self, val: f64) -> &mut Self {
        self.set_active_duration_shim(val);
        self
    }
    #[doc = "Change the `currentIteration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn current_iteration(&mut self, val: Option<f64>) -> &mut Self {
        self.set_current_iteration_shim(val);
        self
    }
    #[doc = "Change the `endTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn end_time(&mut self, val: f64) -> &mut Self {
        self.set_end_time_shim(val);
        self
    }
    #[doc = "Change the `localTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn local_time(&mut self, val: Option<f64>) -> &mut Self {
        self.set_local_time_shim(val);
        self
    }
    #[doc = "Change the `progress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn progress(&mut self, val: Option<f64>) -> &mut Self {
        self.set_progress_shim(val);
        self
    }
}
impl Default for ComputedEffectTiming {
    fn default() -> Self {
        Self::new()
    }
}
