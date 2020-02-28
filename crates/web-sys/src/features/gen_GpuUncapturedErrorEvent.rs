use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = GPUUncapturedErrorEvent , typescript_name = GPUUncapturedErrorEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuUncapturedErrorEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUUncapturedErrorEvent)\n\n*This API requires the following crate features to be activated: `GpuUncapturedErrorEvent`*"]
    pub type GpuUncapturedErrorEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUUncapturedErrorEvent" , js_name = error ) ]
    #[doc = "Getter for the `error` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUUncapturedErrorEvent/error)\n\n*This API requires the following crate features to be activated: `GpuUncapturedErrorEvent`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn error(this: &GpuUncapturedErrorEvent) -> ::js_sys::Object;
}
