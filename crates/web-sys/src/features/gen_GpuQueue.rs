#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUQueue , typescript_type = "GPUQueue" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuQueue` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuQueue;
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUQueue" , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuQueue) -> Option<String>;
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPUQueue" , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuQueue, value: Option<&str>);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuImageBitmapCopyView", feature = "GpuTextureCopyView",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUQueue" , js_name = copyImageBitmapToTexture ) ]
    #[doc = "The `copyImageBitmapToTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/copyImageBitmapToTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageBitmapCopyView`, `GpuQueue`, `GpuTextureCopyView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_image_bitmap_to_texture_with_u32_sequence(
        this: &GpuQueue,
        source: &GpuImageBitmapCopyView,
        destination: &GpuTextureCopyView,
        copy_size: &::wasm_bindgen::JsValue,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuExtent3dDict",
        feature = "GpuImageBitmapCopyView",
        feature = "GpuTextureCopyView",
    ))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUQueue" , js_name = copyImageBitmapToTexture ) ]
    #[doc = "The `copyImageBitmapToTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/copyImageBitmapToTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`, `GpuImageBitmapCopyView`, `GpuQueue`, `GpuTextureCopyView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_image_bitmap_to_texture_with_gpu_extent_3d_dict(
        this: &GpuQueue,
        source: &GpuImageBitmapCopyView,
        destination: &GpuTextureCopyView,
        copy_size: &GpuExtent3dDict,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFence")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUQueue" , js_name = createFence ) ]
    #[doc = "The `createFence()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/createFence)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFence`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_fence(this: &GpuQueue) -> GpuFence;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuFence", feature = "GpuFenceDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUQueue" , js_name = createFence ) ]
    #[doc = "The `createFence()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/createFence)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFence`, `GpuFenceDescriptor`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_fence_with_descriptor(
        this: &GpuQueue,
        descriptor: &GpuFenceDescriptor,
    ) -> GpuFence;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFence")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUQueue" , js_name = signal ) ]
    #[doc = "The `signal()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/signal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFence`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn signal_with_u32(this: &GpuQueue, fence: &GpuFence, signal_value: u32);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFence")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUQueue" , js_name = signal ) ]
    #[doc = "The `signal()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/signal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFence`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn signal_with_f64(this: &GpuQueue, fence: &GpuFence, signal_value: f64);
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUQueue" , js_name = submit ) ]
    #[doc = "The `submit()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/submit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn submit(this: &GpuQueue, command_buffers: &::wasm_bindgen::JsValue);
}
