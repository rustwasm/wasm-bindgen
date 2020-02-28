use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUTextureView , typescript_name = GPUTextureView ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuTextureView` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTextureView)\n\n*This API requires the following crate features to be activated: `GpuTextureView`*"]
    pub type GpuTextureView;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUTextureView" , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTextureView/label)\n\n*This API requires the following crate features to be activated: `GpuTextureView`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuTextureView) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPUTextureView" , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTextureView/label)\n\n*This API requires the following crate features to be activated: `GpuTextureView`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuTextureView, value: Option<&str>);
}
