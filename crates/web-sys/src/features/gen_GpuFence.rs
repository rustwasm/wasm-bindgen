use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUFence , typescript_name = GPUFence ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuFence` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUFence)\n\n*This API requires the following crate features to be activated: `GpuFence`*"]
    pub type GpuFence;
    # [ wasm_bindgen ( structural , method , getter , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUFence/label)\n\n*This API requires the following crate features to be activated: `GpuFence`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuFence) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUFence/label)\n\n*This API requires the following crate features to be activated: `GpuFence`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuFence, value: Option<&str>);
    # [ wasm_bindgen ( method , structural , js_name = getCompletedValue ) ]
    #[doc = "The `getCompletedValue()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUFence/getCompletedValue)\n\n*This API requires the following crate features to be activated: `GpuFence`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_completed_value(this: &GpuFence) -> f64;
    # [ wasm_bindgen ( method , structural , js_name = onCompletion ) ]
    #[doc = "The `onCompletion()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUFence/onCompletion)\n\n*This API requires the following crate features to be activated: `GpuFence`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn on_completion_with_u32(this: &GpuFence, completion_value: u32) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_name = onCompletion ) ]
    #[doc = "The `onCompletion()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUFence/onCompletion)\n\n*This API requires the following crate features to be activated: `GpuFence`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn on_completion_with_f64(this: &GpuFence, completion_value: f64) -> ::js_sys::Promise;
}
