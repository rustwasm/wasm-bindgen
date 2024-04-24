#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SchedulerPostTaskOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SchedulerPostTaskOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SchedulerPostTaskOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SchedulerPostTaskOptions;
    #[wasm_bindgen(method, getter = "delay")]
    fn delay_shim(this: &SchedulerPostTaskOptions) -> f64;
    #[wasm_bindgen(method, setter = "delay")]
    fn set_delay_shim(this: &SchedulerPostTaskOptions, val: f64);
    #[cfg(feature = "TaskPriority")]
    #[wasm_bindgen(method, getter = "priority")]
    fn priority_shim(this: &SchedulerPostTaskOptions) -> TaskPriority;
    #[cfg(feature = "TaskPriority")]
    #[wasm_bindgen(method, setter = "priority")]
    fn set_priority_shim(this: &SchedulerPostTaskOptions, val: TaskPriority);
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(method, getter = "signal")]
    fn signal_shim(this: &SchedulerPostTaskOptions) -> &AbortSignal;
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(method, setter = "signal")]
    fn set_signal_shim(this: &SchedulerPostTaskOptions, val: &AbortSignal);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `SchedulerPostTaskOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SchedulerPostTaskOptions`*"]
pub trait SchedulerPostTaskOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `delay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SchedulerPostTaskOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn delay(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskPriority")]
    #[doc = "Get the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SchedulerPostTaskOptions`, `TaskPriority`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn priority(&self) -> TaskPriority;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `SchedulerPostTaskOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn signal(&self) -> &AbortSignal;
}
#[cfg(web_sys_unstable_apis)]
impl SchedulerPostTaskOptionsGetters for SchedulerPostTaskOptions {
    #[cfg(web_sys_unstable_apis)]
    fn delay(&self) -> f64 {
        self.delay_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskPriority")]
    fn priority(&self) -> TaskPriority {
        self.priority_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AbortSignal")]
    fn signal(&self) -> &AbortSignal {
        self.signal_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl SchedulerPostTaskOptions {
    #[doc = "Construct a new `SchedulerPostTaskOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SchedulerPostTaskOptions`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `SchedulerPostTaskOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn delay(&mut self, val: f64) -> &mut Self {
        self.set_delay_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskPriority")]
    #[doc = "Change the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SchedulerPostTaskOptions`, `TaskPriority`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn priority(&mut self, val: TaskPriority) -> &mut Self {
        self.set_priority_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `SchedulerPostTaskOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn signal(&mut self, val: &AbortSignal) -> &mut Self {
        self.set_signal_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for SchedulerPostTaskOptions {
    fn default() -> Self {
        Self::new()
    }
}
