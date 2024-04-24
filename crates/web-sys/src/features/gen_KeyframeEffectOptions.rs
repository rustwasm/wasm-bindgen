#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = KeyframeEffectOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `KeyframeEffectOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub type KeyframeEffectOptions;
    #[wasm_bindgen(method, getter = "delay")]
    fn delay_shim(this: &KeyframeEffectOptions) -> f64;
    #[wasm_bindgen(method, setter = "delay")]
    fn set_delay_shim(this: &KeyframeEffectOptions, val: f64);
    #[cfg(feature = "PlaybackDirection")]
    #[wasm_bindgen(method, getter = "direction")]
    fn direction_shim(this: &KeyframeEffectOptions) -> PlaybackDirection;
    #[cfg(feature = "PlaybackDirection")]
    #[wasm_bindgen(method, setter = "direction")]
    fn set_direction_shim(this: &KeyframeEffectOptions, val: PlaybackDirection);
    #[wasm_bindgen(method, getter = "duration")]
    fn duration_shim(this: &KeyframeEffectOptions) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "duration")]
    fn set_duration_shim(this: &KeyframeEffectOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "easing")]
    fn easing_shim(this: &KeyframeEffectOptions) -> String;
    #[wasm_bindgen(method, setter = "easing")]
    fn set_easing_shim(this: &KeyframeEffectOptions, val: &str);
    #[wasm_bindgen(method, getter = "endDelay")]
    fn end_delay_shim(this: &KeyframeEffectOptions) -> f64;
    #[wasm_bindgen(method, setter = "endDelay")]
    fn set_end_delay_shim(this: &KeyframeEffectOptions, val: f64);
    #[cfg(feature = "FillMode")]
    #[wasm_bindgen(method, getter = "fill")]
    fn fill_shim(this: &KeyframeEffectOptions) -> FillMode;
    #[cfg(feature = "FillMode")]
    #[wasm_bindgen(method, setter = "fill")]
    fn set_fill_shim(this: &KeyframeEffectOptions, val: FillMode);
    #[wasm_bindgen(method, getter = "iterationStart")]
    fn iteration_start_shim(this: &KeyframeEffectOptions) -> f64;
    #[wasm_bindgen(method, setter = "iterationStart")]
    fn set_iteration_start_shim(this: &KeyframeEffectOptions, val: f64);
    #[wasm_bindgen(method, getter = "iterations")]
    fn iterations_shim(this: &KeyframeEffectOptions) -> f64;
    #[wasm_bindgen(method, setter = "iterations")]
    fn set_iterations_shim(this: &KeyframeEffectOptions, val: f64);
    #[cfg(feature = "CompositeOperation")]
    #[wasm_bindgen(method, getter = "composite")]
    fn composite_shim(this: &KeyframeEffectOptions) -> CompositeOperation;
    #[cfg(feature = "CompositeOperation")]
    #[wasm_bindgen(method, setter = "composite")]
    fn set_composite_shim(this: &KeyframeEffectOptions, val: CompositeOperation);
    #[cfg(feature = "IterationCompositeOperation")]
    #[wasm_bindgen(method, getter = "iterationComposite")]
    fn iteration_composite_shim(this: &KeyframeEffectOptions) -> IterationCompositeOperation;
    #[cfg(feature = "IterationCompositeOperation")]
    #[wasm_bindgen(method, setter = "iterationComposite")]
    fn set_iteration_composite_shim(this: &KeyframeEffectOptions, val: IterationCompositeOperation);
}
#[doc = "The trait to access properties on the `KeyframeEffectOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
pub trait KeyframeEffectOptionsGetters {
    #[doc = "Get the `delay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    fn delay(&self) -> f64;
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Get the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`, `PlaybackDirection`*"]
    fn direction(&self) -> PlaybackDirection;
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    fn duration(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    fn easing(&self) -> String;
    #[doc = "Get the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    fn end_delay(&self) -> f64;
    #[cfg(feature = "FillMode")]
    #[doc = "Get the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FillMode`, `KeyframeEffectOptions`*"]
    fn fill(&self) -> FillMode;
    #[doc = "Get the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    fn iteration_start(&self) -> f64;
    #[doc = "Get the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    fn iterations(&self) -> f64;
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Get the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeEffectOptions`*"]
    fn composite(&self) -> CompositeOperation;
    #[cfg(feature = "IterationCompositeOperation")]
    #[doc = "Get the `iterationComposite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeEffectOptions`*"]
    fn iteration_composite(&self) -> IterationCompositeOperation;
}
impl KeyframeEffectOptionsGetters for KeyframeEffectOptions {
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
    #[cfg(feature = "CompositeOperation")]
    fn composite(&self) -> CompositeOperation {
        self.composite_shim()
    }
    #[cfg(feature = "IterationCompositeOperation")]
    fn iteration_composite(&self) -> IterationCompositeOperation {
        self.iteration_composite_shim()
    }
}
impl KeyframeEffectOptions {
    #[doc = "Construct a new `KeyframeEffectOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `delay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn delay(&mut self, val: f64) -> &mut Self {
        self.set_delay_shim(val);
        self
    }
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`, `PlaybackDirection`*"]
    pub fn direction(&mut self, val: PlaybackDirection) -> &mut Self {
        self.set_direction_shim(val);
        self
    }
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn duration(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_duration_shim(val);
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.set_easing_shim(val);
        self
    }
    #[doc = "Change the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn end_delay(&mut self, val: f64) -> &mut Self {
        self.set_end_delay_shim(val);
        self
    }
    #[cfg(feature = "FillMode")]
    #[doc = "Change the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FillMode`, `KeyframeEffectOptions`*"]
    pub fn fill(&mut self, val: FillMode) -> &mut Self {
        self.set_fill_shim(val);
        self
    }
    #[doc = "Change the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn iteration_start(&mut self, val: f64) -> &mut Self {
        self.set_iteration_start_shim(val);
        self
    }
    #[doc = "Change the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn iterations(&mut self, val: f64) -> &mut Self {
        self.set_iterations_shim(val);
        self
    }
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeEffectOptions`*"]
    pub fn composite(&mut self, val: CompositeOperation) -> &mut Self {
        self.set_composite_shim(val);
        self
    }
    #[cfg(feature = "IterationCompositeOperation")]
    #[doc = "Change the `iterationComposite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeEffectOptions`*"]
    pub fn iteration_composite(&mut self, val: IterationCompositeOperation) -> &mut Self {
        self.set_iteration_composite_shim(val);
        self
    }
}
impl Default for KeyframeEffectOptions {
    fn default() -> Self {
        Self::new()
    }
}
