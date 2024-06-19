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
    #[doc = "Get the `delay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "delay")]
    pub fn get_delay(this: &ComputedEffectTiming) -> Option<f64>;
    #[wasm_bindgen(method, setter = "delay")]
    fn set_delay(this: &ComputedEffectTiming, val: f64);
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Get the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`, `PlaybackDirection`*"]
    #[wasm_bindgen(method, getter = "direction")]
    pub fn get_direction(this: &ComputedEffectTiming) -> Option<PlaybackDirection>;
    #[cfg(feature = "PlaybackDirection")]
    #[wasm_bindgen(method, setter = "direction")]
    fn set_direction(this: &ComputedEffectTiming, val: PlaybackDirection);
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "duration")]
    pub fn get_duration(this: &ComputedEffectTiming) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "duration")]
    fn set_duration(this: &ComputedEffectTiming, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "easing")]
    pub fn get_easing(this: &ComputedEffectTiming) -> Option<String>;
    #[wasm_bindgen(method, setter = "easing")]
    fn set_easing(this: &ComputedEffectTiming, val: &str);
    #[doc = "Get the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "endDelay")]
    pub fn get_end_delay(this: &ComputedEffectTiming) -> Option<f64>;
    #[wasm_bindgen(method, setter = "endDelay")]
    fn set_end_delay(this: &ComputedEffectTiming, val: f64);
    #[cfg(feature = "FillMode")]
    #[doc = "Get the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`, `FillMode`*"]
    #[wasm_bindgen(method, getter = "fill")]
    pub fn get_fill(this: &ComputedEffectTiming) -> Option<FillMode>;
    #[cfg(feature = "FillMode")]
    #[wasm_bindgen(method, setter = "fill")]
    fn set_fill(this: &ComputedEffectTiming, val: FillMode);
    #[doc = "Get the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "iterationStart")]
    pub fn get_iteration_start(this: &ComputedEffectTiming) -> Option<f64>;
    #[wasm_bindgen(method, setter = "iterationStart")]
    fn set_iteration_start(this: &ComputedEffectTiming, val: f64);
    #[doc = "Get the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "iterations")]
    pub fn get_iterations(this: &ComputedEffectTiming) -> Option<f64>;
    #[wasm_bindgen(method, setter = "iterations")]
    fn set_iterations(this: &ComputedEffectTiming, val: f64);
    #[doc = "Get the `activeDuration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "activeDuration")]
    pub fn get_active_duration(this: &ComputedEffectTiming) -> Option<f64>;
    #[wasm_bindgen(method, setter = "activeDuration")]
    fn set_active_duration(this: &ComputedEffectTiming, val: f64);
    #[doc = "Get the `currentIteration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "currentIteration")]
    pub fn get_current_iteration(this: &ComputedEffectTiming) -> Option<f64>;
    #[wasm_bindgen(method, setter = "currentIteration")]
    fn set_current_iteration(this: &ComputedEffectTiming, val: Option<f64>);
    #[doc = "Get the `endTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "endTime")]
    pub fn get_end_time(this: &ComputedEffectTiming) -> Option<f64>;
    #[wasm_bindgen(method, setter = "endTime")]
    fn set_end_time(this: &ComputedEffectTiming, val: f64);
    #[doc = "Get the `localTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "localTime")]
    pub fn get_local_time(this: &ComputedEffectTiming) -> Option<f64>;
    #[wasm_bindgen(method, setter = "localTime")]
    fn set_local_time(this: &ComputedEffectTiming, val: Option<f64>);
    #[doc = "Get the `progress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "progress")]
    pub fn get_progress(this: &ComputedEffectTiming) -> Option<f64>;
    #[wasm_bindgen(method, setter = "progress")]
    fn set_progress(this: &ComputedEffectTiming, val: Option<f64>);
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
        self.set_delay(val);
        self
    }
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`, `PlaybackDirection`*"]
    pub fn direction(&mut self, val: PlaybackDirection) -> &mut Self {
        self.set_direction(val);
        self
    }
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn duration(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_duration(val);
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.set_easing(val);
        self
    }
    #[doc = "Change the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn end_delay(&mut self, val: f64) -> &mut Self {
        self.set_end_delay(val);
        self
    }
    #[cfg(feature = "FillMode")]
    #[doc = "Change the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`, `FillMode`*"]
    pub fn fill(&mut self, val: FillMode) -> &mut Self {
        self.set_fill(val);
        self
    }
    #[doc = "Change the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn iteration_start(&mut self, val: f64) -> &mut Self {
        self.set_iteration_start(val);
        self
    }
    #[doc = "Change the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn iterations(&mut self, val: f64) -> &mut Self {
        self.set_iterations(val);
        self
    }
    #[doc = "Change the `activeDuration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn active_duration(&mut self, val: f64) -> &mut Self {
        self.set_active_duration(val);
        self
    }
    #[doc = "Change the `currentIteration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn current_iteration(&mut self, val: Option<f64>) -> &mut Self {
        self.set_current_iteration(val);
        self
    }
    #[doc = "Change the `endTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn end_time(&mut self, val: f64) -> &mut Self {
        self.set_end_time(val);
        self
    }
    #[doc = "Change the `localTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn local_time(&mut self, val: Option<f64>) -> &mut Self {
        self.set_local_time(val);
        self
    }
    #[doc = "Change the `progress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn progress(&mut self, val: Option<f64>) -> &mut Self {
        self.set_progress(val);
        self
    }
}
impl Default for ComputedEffectTiming {
    fn default() -> Self {
        Self::new()
    }
}
