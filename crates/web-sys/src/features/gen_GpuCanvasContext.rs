use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUCanvasContext , typescript_name = GPUCanvasContext ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuCanvasContext` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext)
    ///
    ///*This API requires the following crate features to be activated: `GpuCanvasContext`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuCanvasContext;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuSwapChain", feature = "GpuSwapChainDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCanvasContext" , js_name = configureSwapChain ) ]
    ///The `configureSwapChain()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/configureSwapChain)
    ///
    ///*This API requires the following crate features to be activated: `GpuCanvasContext`, `GpuSwapChain`, `GpuSwapChainDescriptor`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn configure_swap_chain(
        this: &GpuCanvasContext,
        descriptor: &GpuSwapChainDescriptor,
    ) -> GpuSwapChain;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDevice")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCanvasContext" , js_name = getSwapChainPreferredFormat ) ]
    ///The `getSwapChainPreferredFormat()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/getSwapChainPreferredFormat)
    ///
    ///*This API requires the following crate features to be activated: `GpuCanvasContext`, `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn get_swap_chain_preferred_format(
        this: &GpuCanvasContext,
        device: &GpuDevice,
    ) -> ::js_sys::Promise;

}
