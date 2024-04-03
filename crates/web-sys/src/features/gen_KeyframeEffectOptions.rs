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
    #[wasm_bindgen(method, setter = "delay")]
    fn delay_shim(this: &KeyframeEffectOptions, val: f64);
    #[cfg(feature = "PlaybackDirection")]
    #[wasm_bindgen(method, setter = "direction")]
    fn direction_shim(this: &KeyframeEffectOptions, val: PlaybackDirection);
    #[wasm_bindgen(method, setter = "duration")]
    fn duration_shim(this: &KeyframeEffectOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "easing")]
    fn easing_shim(this: &KeyframeEffectOptions, val: &str);
    #[wasm_bindgen(method, setter = "endDelay")]
    fn end_delay_shim(this: &KeyframeEffectOptions, val: f64);
    #[cfg(feature = "FillMode")]
    #[wasm_bindgen(method, setter = "fill")]
    fn fill_shim(this: &KeyframeEffectOptions, val: FillMode);
    #[wasm_bindgen(method, setter = "iterationStart")]
    fn iteration_start_shim(this: &KeyframeEffectOptions, val: f64);
    #[wasm_bindgen(method, setter = "iterations")]
    fn iterations_shim(this: &KeyframeEffectOptions, val: f64);
    #[cfg(feature = "CompositeOperation")]
    #[wasm_bindgen(method, setter = "composite")]
    fn composite_shim(this: &KeyframeEffectOptions, val: CompositeOperation);
    #[cfg(feature = "IterationCompositeOperation")]
    #[wasm_bindgen(method, setter = "iterationComposite")]
    fn iteration_composite_shim(this: &KeyframeEffectOptions, val: IterationCompositeOperation);
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
        self.delay_shim(val);
        self
    }
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`, `PlaybackDirection`*"]
    pub fn direction(&mut self, val: PlaybackDirection) -> &mut Self {
        self.direction_shim(val);
        self
    }
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn duration(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.duration_shim(val);
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.easing_shim(val);
        self
    }
    #[doc = "Change the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn end_delay(&mut self, val: f64) -> &mut Self {
        self.end_delay_shim(val);
        self
    }
    #[cfg(feature = "FillMode")]
    #[doc = "Change the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FillMode`, `KeyframeEffectOptions`*"]
    pub fn fill(&mut self, val: FillMode) -> &mut Self {
        self.fill_shim(val);
        self
    }
    #[doc = "Change the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn iteration_start(&mut self, val: f64) -> &mut Self {
        self.iteration_start_shim(val);
        self
    }
    #[doc = "Change the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn iterations(&mut self, val: f64) -> &mut Self {
        self.iterations_shim(val);
        self
    }
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeEffectOptions`*"]
    pub fn composite(&mut self, val: CompositeOperation) -> &mut Self {
        self.composite_shim(val);
        self
    }
    #[cfg(feature = "IterationCompositeOperation")]
    #[doc = "Change the `iterationComposite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeEffectOptions`*"]
    pub fn iteration_composite(&mut self, val: IterationCompositeOperation) -> &mut Self {
        self.iteration_composite_shim(val);
        self
    }
}
impl Default for KeyframeEffectOptions {
    fn default() -> Self {
        Self::new()
    }
}
