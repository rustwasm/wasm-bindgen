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
    #[doc = "Get the `delay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    #[wasm_bindgen(method, getter = "delay")]
    pub fn get_delay(this: &KeyframeEffectOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter = "delay")]
    fn set_delay(this: &KeyframeEffectOptions, val: f64);
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Get the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`, `PlaybackDirection`*"]
    #[wasm_bindgen(method, getter = "direction")]
    pub fn get_direction(this: &KeyframeEffectOptions) -> Option<PlaybackDirection>;
    #[cfg(feature = "PlaybackDirection")]
    #[wasm_bindgen(method, setter = "direction")]
    fn set_direction(this: &KeyframeEffectOptions, val: PlaybackDirection);
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    #[wasm_bindgen(method, getter = "duration")]
    pub fn get_duration(this: &KeyframeEffectOptions) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "duration")]
    fn set_duration(this: &KeyframeEffectOptions, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    #[wasm_bindgen(method, getter = "easing")]
    pub fn get_easing(this: &KeyframeEffectOptions) -> Option<String>;
    #[wasm_bindgen(method, setter = "easing")]
    fn set_easing(this: &KeyframeEffectOptions, val: &str);
    #[doc = "Get the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    #[wasm_bindgen(method, getter = "endDelay")]
    pub fn get_end_delay(this: &KeyframeEffectOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter = "endDelay")]
    fn set_end_delay(this: &KeyframeEffectOptions, val: f64);
    #[cfg(feature = "FillMode")]
    #[doc = "Get the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FillMode`, `KeyframeEffectOptions`*"]
    #[wasm_bindgen(method, getter = "fill")]
    pub fn get_fill(this: &KeyframeEffectOptions) -> Option<FillMode>;
    #[cfg(feature = "FillMode")]
    #[wasm_bindgen(method, setter = "fill")]
    fn set_fill(this: &KeyframeEffectOptions, val: FillMode);
    #[doc = "Get the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    #[wasm_bindgen(method, getter = "iterationStart")]
    pub fn get_iteration_start(this: &KeyframeEffectOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter = "iterationStart")]
    fn set_iteration_start(this: &KeyframeEffectOptions, val: f64);
    #[doc = "Get the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    #[wasm_bindgen(method, getter = "iterations")]
    pub fn get_iterations(this: &KeyframeEffectOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter = "iterations")]
    fn set_iterations(this: &KeyframeEffectOptions, val: f64);
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Get the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeEffectOptions`*"]
    #[wasm_bindgen(method, getter = "composite")]
    pub fn get_composite(this: &KeyframeEffectOptions) -> Option<CompositeOperation>;
    #[cfg(feature = "CompositeOperation")]
    #[wasm_bindgen(method, setter = "composite")]
    fn set_composite(this: &KeyframeEffectOptions, val: CompositeOperation);
    #[cfg(feature = "IterationCompositeOperation")]
    #[doc = "Get the `iterationComposite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeEffectOptions`*"]
    #[wasm_bindgen(method, getter = "iterationComposite")]
    pub fn get_iteration_composite(
        this: &KeyframeEffectOptions,
    ) -> Option<IterationCompositeOperation>;
    #[cfg(feature = "IterationCompositeOperation")]
    #[wasm_bindgen(method, setter = "iterationComposite")]
    fn set_iteration_composite(this: &KeyframeEffectOptions, val: IterationCompositeOperation);
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
        self.set_delay(val);
        self
    }
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`, `PlaybackDirection`*"]
    pub fn direction(&mut self, val: PlaybackDirection) -> &mut Self {
        self.set_direction(val);
        self
    }
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn duration(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_duration(val);
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.set_easing(val);
        self
    }
    #[doc = "Change the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn end_delay(&mut self, val: f64) -> &mut Self {
        self.set_end_delay(val);
        self
    }
    #[cfg(feature = "FillMode")]
    #[doc = "Change the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FillMode`, `KeyframeEffectOptions`*"]
    pub fn fill(&mut self, val: FillMode) -> &mut Self {
        self.set_fill(val);
        self
    }
    #[doc = "Change the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn iteration_start(&mut self, val: f64) -> &mut Self {
        self.set_iteration_start(val);
        self
    }
    #[doc = "Change the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffectOptions`*"]
    pub fn iterations(&mut self, val: f64) -> &mut Self {
        self.set_iterations(val);
        self
    }
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeEffectOptions`*"]
    pub fn composite(&mut self, val: CompositeOperation) -> &mut Self {
        self.set_composite(val);
        self
    }
    #[cfg(feature = "IterationCompositeOperation")]
    #[doc = "Change the `iterationComposite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeEffectOptions`*"]
    pub fn iteration_composite(&mut self, val: IterationCompositeOperation) -> &mut Self {
        self.set_iteration_composite(val);
        self
    }
}
impl Default for KeyframeEffectOptions {
    fn default() -> Self {
        Self::new()
    }
}
