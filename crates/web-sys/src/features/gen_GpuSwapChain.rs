use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUSwapChain , typescript_name = GPUSwapChain ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuSwapChain` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSwapChain)\n\n*This API requires the following crate features to be activated: `GpuSwapChain`*"]
    pub type GpuSwapChain;
    # [ wasm_bindgen ( structural , method , getter , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSwapChain/label)\n\n*This API requires the following crate features to be activated: `GpuSwapChain`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuSwapChain) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSwapChain/label)\n\n*This API requires the following crate features to be activated: `GpuSwapChain`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuSwapChain, value: Option<&str>);
    #[cfg(feature = "GpuTexture")]
    # [ wasm_bindgen ( method , structural , js_name = getCurrentTexture ) ]
    #[doc = "The `getCurrentTexture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSwapChain/getCurrentTexture)\n\n*This API requires the following crate features to be activated: `GpuSwapChain`, `GpuTexture`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_current_texture(this: &GpuSwapChain) -> GpuTexture;
}
