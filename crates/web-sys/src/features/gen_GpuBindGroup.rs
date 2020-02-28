use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUBindGroup , typescript_name = GPUBindGroup ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuBindGroup` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBindGroup)\n\n*This API requires the following crate features to be activated: `GpuBindGroup`*"]
    pub type GpuBindGroup;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUBindGroup" , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBindGroup/label)\n\n*This API requires the following crate features to be activated: `GpuBindGroup`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuBindGroup) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPUBindGroup" , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBindGroup/label)\n\n*This API requires the following crate features to be activated: `GpuBindGroup`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuBindGroup, value: Option<&str>);
}
