use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUCommandEncoder , typescript_name = GPUCommandEncoder ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuCommandEncoder` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder)
    ///
    ///*This API requires the following crate features to be activated: `GpuCommandEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuCommandEncoder;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUCommandEncoder" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuCommandEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn label(this: &GpuCommandEncoder) -> Option<String>;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPUCommandEncoder" , js_name = label ) ]
    ///Setter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuCommandEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_label(this: &GpuCommandEncoder, value: Option<&str>);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuComputePassEncoder")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = beginComputePass ) ]
    ///The `beginComputePass()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/beginComputePass)
    ///
    ///*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn begin_compute_pass(this: &GpuCommandEncoder) -> GpuComputePassEncoder;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuComputePassDescriptor",
        feature = "GpuComputePassEncoder",
    ))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = beginComputePass ) ]
    ///The `beginComputePass()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/beginComputePass)
    ///
    ///*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuComputePassDescriptor`, `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn begin_compute_pass_with_descriptor(
        this: &GpuCommandEncoder,
        descriptor: &GpuComputePassDescriptor,
    ) -> GpuComputePassEncoder;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuRenderPassDescriptor", feature = "GpuRenderPassEncoder",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = beginRenderPass ) ]
    ///The `beginRenderPass()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/beginRenderPass)
    ///
    ///*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuRenderPassDescriptor`, `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn begin_render_pass(
        this: &GpuCommandEncoder,
        descriptor: &GpuRenderPassDescriptor,
    ) -> GpuRenderPassEncoder;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer ) ]
    ///The `copyBufferToBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn copy_buffer_to_buffer_with_u32_and_u32_and_u32(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: u32,
        destination: &GpuBuffer,
        destination_offset: u32,
        size: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer ) ]
    ///The `copyBufferToBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn copy_buffer_to_buffer_with_f64_and_u32_and_u32(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: f64,
        destination: &GpuBuffer,
        destination_offset: u32,
        size: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer ) ]
    ///The `copyBufferToBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn copy_buffer_to_buffer_with_u32_and_f64_and_u32(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: u32,
        destination: &GpuBuffer,
        destination_offset: f64,
        size: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer ) ]
    ///The `copyBufferToBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn copy_buffer_to_buffer_with_f64_and_f64_and_u32(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: f64,
        destination: &GpuBuffer,
        destination_offset: f64,
        size: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer ) ]
    ///The `copyBufferToBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn copy_buffer_to_buffer_with_u32_and_u32_and_f64(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: u32,
        destination: &GpuBuffer,
        destination_offset: u32,
        size: f64,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer ) ]
    ///The `copyBufferToBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn copy_buffer_to_buffer_with_f64_and_u32_and_f64(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: f64,
        destination: &GpuBuffer,
        destination_offset: u32,
        size: f64,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer ) ]
    ///The `copyBufferToBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn copy_buffer_to_buffer_with_u32_and_f64_and_f64(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: u32,
        destination: &GpuBuffer,
        destination_offset: f64,
        size: f64,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer ) ]
    ///The `copyBufferToBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn copy_buffer_to_buffer_with_f64_and_f64_and_f64(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: f64,
        destination: &GpuBuffer,
        destination_offset: f64,
        size: f64,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBufferCopyView", feature = "GpuTextureCopyView",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToTexture ) ]
    ///The `copyBufferToTexture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToTexture)
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferCopyView`, `GpuCommandEncoder`, `GpuTextureCopyView`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn copy_buffer_to_texture_with_u32_sequence(
        this: &GpuCommandEncoder,
        source: &GpuBufferCopyView,
        destination: &GpuTextureCopyView,
        copy_size: &::wasm_bindgen::JsValue,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuBufferCopyView",
        feature = "GpuExtent3dDict",
        feature = "GpuTextureCopyView",
    ))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToTexture ) ]
    ///The `copyBufferToTexture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToTexture)
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferCopyView`, `GpuCommandEncoder`, `GpuExtent3dDict`, `GpuTextureCopyView`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn copy_buffer_to_texture_with_gpu_extent_3d_dict(
        this: &GpuCommandEncoder,
        source: &GpuBufferCopyView,
        destination: &GpuTextureCopyView,
        copy_size: &GpuExtent3dDict,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBufferCopyView", feature = "GpuTextureCopyView",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyTextureToBuffer ) ]
    ///The `copyTextureToBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferCopyView`, `GpuCommandEncoder`, `GpuTextureCopyView`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn copy_texture_to_buffer_with_u32_sequence(
        this: &GpuCommandEncoder,
        source: &GpuTextureCopyView,
        destination: &GpuBufferCopyView,
        copy_size: &::wasm_bindgen::JsValue,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuBufferCopyView",
        feature = "GpuExtent3dDict",
        feature = "GpuTextureCopyView",
    ))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyTextureToBuffer ) ]
    ///The `copyTextureToBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferCopyView`, `GpuCommandEncoder`, `GpuExtent3dDict`, `GpuTextureCopyView`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn copy_texture_to_buffer_with_gpu_extent_3d_dict(
        this: &GpuCommandEncoder,
        source: &GpuTextureCopyView,
        destination: &GpuBufferCopyView,
        copy_size: &GpuExtent3dDict,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureCopyView")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyTextureToTexture ) ]
    ///The `copyTextureToTexture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToTexture)
    ///
    ///*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuTextureCopyView`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn copy_texture_to_texture_with_u32_sequence(
        this: &GpuCommandEncoder,
        source: &GpuTextureCopyView,
        destination: &GpuTextureCopyView,
        copy_size: &::wasm_bindgen::JsValue,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuExtent3dDict", feature = "GpuTextureCopyView",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyTextureToTexture ) ]
    ///The `copyTextureToTexture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToTexture)
    ///
    ///*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuExtent3dDict`, `GpuTextureCopyView`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn copy_texture_to_texture_with_gpu_extent_3d_dict(
        this: &GpuCommandEncoder,
        source: &GpuTextureCopyView,
        destination: &GpuTextureCopyView,
        copy_size: &GpuExtent3dDict,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCommandBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = finish ) ]
    ///The `finish()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/finish)
    ///
    ///*This API requires the following crate features to be activated: `GpuCommandBuffer`, `GpuCommandEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn finish(this: &GpuCommandEncoder) -> GpuCommandBuffer;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuCommandBuffer", feature = "GpuCommandBufferDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = finish ) ]
    ///The `finish()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/finish)
    ///
    ///*This API requires the following crate features to be activated: `GpuCommandBuffer`, `GpuCommandBufferDescriptor`, `GpuCommandEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn finish_with_descriptor(
        this: &GpuCommandEncoder,
        descriptor: &GpuCommandBufferDescriptor,
    ) -> GpuCommandBuffer;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = insertDebugMarker ) ]
    ///The `insertDebugMarker()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/insertDebugMarker)
    ///
    ///*This API requires the following crate features to be activated: `GpuCommandEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn insert_debug_marker(this: &GpuCommandEncoder, marker_label: &str);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = popDebugGroup ) ]
    ///The `popDebugGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/popDebugGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuCommandEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn pop_debug_group(this: &GpuCommandEncoder);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = pushDebugGroup ) ]
    ///The `pushDebugGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/pushDebugGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuCommandEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn push_debug_group(this: &GpuCommandEncoder, group_label: &str);

}
