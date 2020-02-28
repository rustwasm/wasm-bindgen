use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUQueue , typescript_name = GPUQueue ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuQueue` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue)\n\n*This API requires the following crate features to be activated: `GpuQueue`*"]
    pub type GpuQueue;
    # [ wasm_bindgen ( structural , method , getter , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/label)\n\n*This API requires the following crate features to be activated: `GpuQueue`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuQueue) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/label)\n\n*This API requires the following crate features to be activated: `GpuQueue`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuQueue, value: Option<&str>);
    #[cfg(all(feature = "GpuImageBitmapCopyView", feature = "GpuTextureCopyView",))]
    # [ wasm_bindgen ( method , structural , js_name = copyImageBitmapToTexture ) ]
    #[doc = "The `copyImageBitmapToTexture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/copyImageBitmapToTexture)\n\n*This API requires the following crate features to be activated: `GpuImageBitmapCopyView`, `GpuQueue`, `GpuTextureCopyView`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_image_bitmap_to_texture_with_u32_sequence(
        this: &GpuQueue,
        source: &GpuImageBitmapCopyView,
        destination: &GpuTextureCopyView,
        copy_size: &::wasm_bindgen::JsValue,
    );
    #[cfg(all(
        feature = "GpuExtent3dDict",
        feature = "GpuImageBitmapCopyView",
        feature = "GpuTextureCopyView",
    ))]
    # [ wasm_bindgen ( method , structural , js_name = copyImageBitmapToTexture ) ]
    #[doc = "The `copyImageBitmapToTexture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/copyImageBitmapToTexture)\n\n*This API requires the following crate features to be activated: `GpuExtent3dDict`, `GpuImageBitmapCopyView`, `GpuQueue`, `GpuTextureCopyView`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_image_bitmap_to_texture_with_gpu_extent_3d_dict(
        this: &GpuQueue,
        source: &GpuImageBitmapCopyView,
        destination: &GpuTextureCopyView,
        copy_size: &GpuExtent3dDict,
    );
    #[cfg(feature = "GpuFence")]
    # [ wasm_bindgen ( method , structural , js_name = createFence ) ]
    #[doc = "The `createFence()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/createFence)\n\n*This API requires the following crate features to be activated: `GpuFence`, `GpuQueue`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_fence(this: &GpuQueue) -> GpuFence;
    #[cfg(all(feature = "GpuFence", feature = "GpuFenceDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_name = createFence ) ]
    #[doc = "The `createFence()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/createFence)\n\n*This API requires the following crate features to be activated: `GpuFence`, `GpuFenceDescriptor`, `GpuQueue`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_fence_with_descriptor(
        this: &GpuQueue,
        descriptor: &GpuFenceDescriptor,
    ) -> GpuFence;
    #[cfg(feature = "GpuFence")]
    # [ wasm_bindgen ( method , structural , js_name = signal ) ]
    #[doc = "The `signal()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/signal)\n\n*This API requires the following crate features to be activated: `GpuFence`, `GpuQueue`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn signal_with_u32(this: &GpuQueue, fence: &GpuFence, signal_value: u32);
    #[cfg(feature = "GpuFence")]
    # [ wasm_bindgen ( method , structural , js_name = signal ) ]
    #[doc = "The `signal()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/signal)\n\n*This API requires the following crate features to be activated: `GpuFence`, `GpuQueue`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn signal_with_f64(this: &GpuQueue, fence: &GpuFence, signal_value: f64);
    # [ wasm_bindgen ( method , structural , js_name = submit ) ]
    #[doc = "The `submit()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/submit)\n\n*This API requires the following crate features to be activated: `GpuQueue`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn submit(this: &GpuQueue, command_buffers: &::wasm_bindgen::JsValue);
}
