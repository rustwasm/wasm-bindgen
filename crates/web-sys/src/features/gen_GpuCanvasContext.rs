use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUCanvasContext , typescript_name = GPUCanvasContext ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuCanvasContext` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext)\n\n*This API requires the following crate features to be activated: `GpuCanvasContext`*"]
    pub type GpuCanvasContext;
    #[cfg(all(feature = "GpuSwapChain", feature = "GpuSwapChainDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCanvasContext" , js_name = configureSwapChain ) ]
    #[doc = "The `configureSwapChain()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/configureSwapChain)\n\n*This API requires the following crate features to be activated: `GpuCanvasContext`, `GpuSwapChain`, `GpuSwapChainDescriptor`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn configure_swap_chain(
        this: &GpuCanvasContext,
        descriptor: &GpuSwapChainDescriptor,
    ) -> GpuSwapChain;
    #[cfg(feature = "GpuDevice")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCanvasContext" , js_name = getSwapChainPreferredFormat ) ]
    #[doc = "The `getSwapChainPreferredFormat()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/getSwapChainPreferredFormat)\n\n*This API requires the following crate features to be activated: `GpuCanvasContext`, `GpuDevice`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_swap_chain_preferred_format(
        this: &GpuCanvasContext,
        device: &GpuDevice,
    ) -> ::js_sys::Promise;
}
