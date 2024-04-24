#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = KeyframeAnimationOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `KeyframeAnimationOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type KeyframeAnimationOptions;
    #[wasm_bindgen(method, getter = "delay")]
    fn delay_shim(this: &KeyframeAnimationOptions) -> f64;
    #[wasm_bindgen(method, setter = "delay")]
    fn set_delay_shim(this: &KeyframeAnimationOptions, val: f64);
    #[cfg(feature = "PlaybackDirection")]
    #[wasm_bindgen(method, getter = "direction")]
    fn direction_shim(this: &KeyframeAnimationOptions) -> PlaybackDirection;
    #[cfg(feature = "PlaybackDirection")]
    #[wasm_bindgen(method, setter = "direction")]
    fn set_direction_shim(this: &KeyframeAnimationOptions, val: PlaybackDirection);
    #[wasm_bindgen(method, getter = "duration")]
    fn duration_shim(this: &KeyframeAnimationOptions) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "duration")]
    fn set_duration_shim(this: &KeyframeAnimationOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "easing")]
    fn easing_shim(this: &KeyframeAnimationOptions) -> String;
    #[wasm_bindgen(method, setter = "easing")]
    fn set_easing_shim(this: &KeyframeAnimationOptions, val: &str);
    #[wasm_bindgen(method, getter = "endDelay")]
    fn end_delay_shim(this: &KeyframeAnimationOptions) -> f64;
    #[wasm_bindgen(method, setter = "endDelay")]
    fn set_end_delay_shim(this: &KeyframeAnimationOptions, val: f64);
    #[cfg(feature = "FillMode")]
    #[wasm_bindgen(method, getter = "fill")]
    fn fill_shim(this: &KeyframeAnimationOptions) -> FillMode;
    #[cfg(feature = "FillMode")]
    #[wasm_bindgen(method, setter = "fill")]
    fn set_fill_shim(this: &KeyframeAnimationOptions, val: FillMode);
    #[wasm_bindgen(method, getter = "iterationStart")]
    fn iteration_start_shim(this: &KeyframeAnimationOptions) -> f64;
    #[wasm_bindgen(method, setter = "iterationStart")]
    fn set_iteration_start_shim(this: &KeyframeAnimationOptions, val: f64);
    #[wasm_bindgen(method, getter = "iterations")]
    fn iterations_shim(this: &KeyframeAnimationOptions) -> f64;
    #[wasm_bindgen(method, setter = "iterations")]
    fn set_iterations_shim(this: &KeyframeAnimationOptions, val: f64);
    #[cfg(feature = "CompositeOperation")]
    #[wasm_bindgen(method, getter = "composite")]
    fn composite_shim(this: &KeyframeAnimationOptions) -> CompositeOperation;
    #[cfg(feature = "CompositeOperation")]
    #[wasm_bindgen(method, setter = "composite")]
    fn set_composite_shim(this: &KeyframeAnimationOptions, val: CompositeOperation);
    #[cfg(feature = "IterationCompositeOperation")]
    #[wasm_bindgen(method, getter = "iterationComposite")]
    fn iteration_composite_shim(this: &KeyframeAnimationOptions) -> IterationCompositeOperation;
    #[cfg(feature = "IterationCompositeOperation")]
    #[wasm_bindgen(method, setter = "iterationComposite")]
    fn set_iteration_composite_shim(
        this: &KeyframeAnimationOptions,
        val: IterationCompositeOperation,
    );
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &KeyframeAnimationOptions) -> String;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &KeyframeAnimationOptions, val: &str);
    #[cfg(feature = "AnimationTimeline")]
    #[wasm_bindgen(method, getter = "timeline")]
    fn timeline_shim(this: &KeyframeAnimationOptions) -> Option<AnimationTimeline>;
    #[cfg(feature = "AnimationTimeline")]
    #[wasm_bindgen(method, setter = "timeline")]
    fn set_timeline_shim(this: &KeyframeAnimationOptions, val: Option<&AnimationTimeline>);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `KeyframeAnimationOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
pub trait KeyframeAnimationOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `delay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn delay(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Get the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`, `PlaybackDirection`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn direction(&self) -> PlaybackDirection;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn duration(&self) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn easing(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn end_delay(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FillMode")]
    #[doc = "Get the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FillMode`, `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn fill(&self) -> FillMode;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn iteration_start(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn iterations(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Get the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn composite(&self) -> CompositeOperation;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "IterationCompositeOperation")]
    #[doc = "Get the `iterationComposite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn iteration_composite(&self) -> IterationCompositeOperation;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn id(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AnimationTimeline")]
    #[doc = "Get the `timeline` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationTimeline`, `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn timeline(&self) -> Option<AnimationTimeline>;
}
#[cfg(web_sys_unstable_apis)]
impl KeyframeAnimationOptionsGetters for KeyframeAnimationOptions {
    #[cfg(web_sys_unstable_apis)]
    fn delay(&self) -> f64 {
        self.delay_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PlaybackDirection")]
    fn direction(&self) -> PlaybackDirection {
        self.direction_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn duration(&self) -> ::wasm_bindgen::JsValue {
        self.duration_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn easing(&self) -> String {
        self.easing_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn end_delay(&self) -> f64 {
        self.end_delay_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FillMode")]
    fn fill(&self) -> FillMode {
        self.fill_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn iteration_start(&self) -> f64 {
        self.iteration_start_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn iterations(&self) -> f64 {
        self.iterations_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CompositeOperation")]
    fn composite(&self) -> CompositeOperation {
        self.composite_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "IterationCompositeOperation")]
    fn iteration_composite(&self) -> IterationCompositeOperation {
        self.iteration_composite_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn id(&self) -> String {
        self.id_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AnimationTimeline")]
    fn timeline(&self) -> Option<AnimationTimeline> {
        self.timeline_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl KeyframeAnimationOptions {
    #[doc = "Construct a new `KeyframeAnimationOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `delay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn delay(&mut self, val: f64) -> &mut Self {
        self.set_delay_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`, `PlaybackDirection`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn direction(&mut self, val: PlaybackDirection) -> &mut Self {
        self.set_direction_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn duration(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_duration_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.set_easing_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn end_delay(&mut self, val: f64) -> &mut Self {
        self.set_end_delay_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FillMode")]
    #[doc = "Change the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FillMode`, `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn fill(&mut self, val: FillMode) -> &mut Self {
        self.set_fill_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn iteration_start(&mut self, val: f64) -> &mut Self {
        self.set_iteration_start_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn iterations(&mut self, val: f64) -> &mut Self {
        self.set_iterations_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn composite(&mut self, val: CompositeOperation) -> &mut Self {
        self.set_composite_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "IterationCompositeOperation")]
    #[doc = "Change the `iterationComposite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn iteration_composite(&mut self, val: IterationCompositeOperation) -> &mut Self {
        self.set_iteration_composite_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AnimationTimeline")]
    #[doc = "Change the `timeline` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationTimeline`, `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn timeline(&mut self, val: Option<&AnimationTimeline>) -> &mut Self {
        self.set_timeline_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for KeyframeAnimationOptions {
    fn default() -> Self {
        Self::new()
    }
}
