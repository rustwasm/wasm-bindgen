use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUShaderModule , typescript_name = GPUShaderModule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuShaderModule` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUShaderModule)\n\n*This API requires the following crate features to be activated: `GpuShaderModule`*"]
    pub type GpuShaderModule;
    # [ wasm_bindgen ( structural , method , getter , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUShaderModule/label)\n\n*This API requires the following crate features to be activated: `GpuShaderModule`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuShaderModule) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUShaderModule/label)\n\n*This API requires the following crate features to be activated: `GpuShaderModule`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuShaderModule, value: Option<&str>);
}
