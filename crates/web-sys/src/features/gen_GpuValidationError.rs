use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUValidationError , typescript_name = GPUValidationError ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuValidationError` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUValidationError)\n\n*This API requires the following crate features to be activated: `GpuValidationError`*"]
    pub type GpuValidationError;
    # [ wasm_bindgen ( structural , method , getter , js_name = message ) ]
    #[doc = "Getter for the `message` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUValidationError/message)\n\n*This API requires the following crate features to be activated: `GpuValidationError`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn message(this: &GpuValidationError) -> String;
}
