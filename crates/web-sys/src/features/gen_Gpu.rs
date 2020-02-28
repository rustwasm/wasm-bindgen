use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPU , typescript_name = GPU ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Gpu` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPU)\n\n*This API requires the following crate features to be activated: `Gpu`*"]
    pub type Gpu;
    # [ wasm_bindgen ( method , structural , js_name = requestAdapter ) ]
    #[doc = "The `requestAdapter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPU/requestAdapter)\n\n*This API requires the following crate features to be activated: `Gpu`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_adapter(this: &Gpu) -> ::js_sys::Promise;
    #[cfg(feature = "GpuRequestAdapterOptions")]
    # [ wasm_bindgen ( method , structural , js_name = requestAdapter ) ]
    #[doc = "The `requestAdapter()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPU/requestAdapter)\n\n*This API requires the following crate features to be activated: `Gpu`, `GpuRequestAdapterOptions`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_adapter_with_options(
        this: &Gpu,
        options: &GpuRequestAdapterOptions,
    ) -> ::js_sys::Promise;
}
