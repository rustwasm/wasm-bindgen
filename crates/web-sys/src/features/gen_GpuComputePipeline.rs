use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUComputePipeline , typescript_type = "GPUComputePipeline" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuComputePipeline` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePipeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePipeline`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuComputePipeline;
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUComputePipeline" , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePipeline/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePipeline`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuComputePipeline) -> Option<String>;
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPUComputePipeline" , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePipeline/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePipeline`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuComputePipeline, value: Option<&str>);
}
