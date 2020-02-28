use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUPipelineLayout , typescript_name = GPUPipelineLayout ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuPipelineLayout` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUPipelineLayout)\n\n*This API requires the following crate features to be activated: `GpuPipelineLayout`*"]
    pub type GpuPipelineLayout;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUPipelineLayout" , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUPipelineLayout/label)\n\n*This API requires the following crate features to be activated: `GpuPipelineLayout`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuPipelineLayout) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPUPipelineLayout" , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUPipelineLayout/label)\n\n*This API requires the following crate features to be activated: `GpuPipelineLayout`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuPipelineLayout, value: Option<&str>);
}
