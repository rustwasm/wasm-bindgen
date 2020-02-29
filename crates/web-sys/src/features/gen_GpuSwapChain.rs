use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUSwapChain , typescript_name = GPUSwapChain ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuSwapChain` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSwapChain)
    ///
    ///*This API requires the following crate features to be activated: `GpuSwapChain`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuSwapChain;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUSwapChain" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSwapChain/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuSwapChain`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn label(this: &GpuSwapChain) -> Option<String>;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPUSwapChain" , js_name = label ) ]
    ///Setter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSwapChain/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuSwapChain`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_label(this: &GpuSwapChain, value: Option<&str>);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTexture")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUSwapChain" , js_name = getCurrentTexture ) ]
    ///The `getCurrentTexture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSwapChain/getCurrentTexture)
    ///
    ///*This API requires the following crate features to be activated: `GpuSwapChain`, `GpuTexture`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn get_current_texture(this: &GpuSwapChain) -> GpuTexture;

}
