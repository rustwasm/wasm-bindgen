use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUTexture , typescript_name = GPUTexture ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuTexture` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture)\n\n*This API requires the following crate features to be activated: `GpuTexture`*"]
    pub type GpuTexture;
    # [ wasm_bindgen ( structural , method , getter , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/label)\n\n*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuTexture) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/label)\n\n*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuTexture, value: Option<&str>);
    #[cfg(feature = "GpuTextureView")]
    # [ wasm_bindgen ( method , structural , js_name = createView ) ]
    #[doc = "The `createView()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/createView)\n\n*This API requires the following crate features to be activated: `GpuTexture`, `GpuTextureView`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_view(this: &GpuTexture) -> GpuTextureView;
    #[cfg(all(feature = "GpuTextureView", feature = "GpuTextureViewDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_name = createView ) ]
    #[doc = "The `createView()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/createView)\n\n*This API requires the following crate features to be activated: `GpuTexture`, `GpuTextureView`, `GpuTextureViewDescriptor`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_view_with_descriptor(
        this: &GpuTexture,
        descriptor: &GpuTextureViewDescriptor,
    ) -> GpuTextureView;
    # [ wasm_bindgen ( method , structural , js_name = destroy ) ]
    #[doc = "The `destroy()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/destroy)\n\n*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn destroy(this: &GpuTexture);
}
