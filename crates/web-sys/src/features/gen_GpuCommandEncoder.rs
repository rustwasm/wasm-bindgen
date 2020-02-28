use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUCommandEncoder , typescript_name = GPUCommandEncoder ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuCommandEncoder` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder)\n\n*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    pub type GpuCommandEncoder;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUCommandEncoder" , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/label)\n\n*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuCommandEncoder) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPUCommandEncoder" , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/label)\n\n*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuCommandEncoder, value: Option<&str>);
    #[cfg(feature = "GpuComputePassEncoder")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = beginComputePass ) ]
    #[doc = "The `beginComputePass()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/beginComputePass)\n\n*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn begin_compute_pass(this: &GpuCommandEncoder) -> GpuComputePassEncoder;
    #[cfg(all(
        feature = "GpuComputePassDescriptor",
        feature = "GpuComputePassEncoder",
    ))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = beginComputePass ) ]
    #[doc = "The `beginComputePass()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/beginComputePass)\n\n*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuComputePassDescriptor`, `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn begin_compute_pass_with_descriptor(
        this: &GpuCommandEncoder,
        descriptor: &GpuComputePassDescriptor,
    ) -> GpuComputePassEncoder;
    #[cfg(all(feature = "GpuRenderPassDescriptor", feature = "GpuRenderPassEncoder",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = beginRenderPass ) ]
    #[doc = "The `beginRenderPass()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/beginRenderPass)\n\n*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuRenderPassDescriptor`, `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn begin_render_pass(
        this: &GpuCommandEncoder,
        descriptor: &GpuRenderPassDescriptor,
    ) -> GpuRenderPassEncoder;
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer ) ]
    #[doc = "The `copyBufferToBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_buffer_to_buffer_with_u32_and_u32_and_u32(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: u32,
        destination: &GpuBuffer,
        destination_offset: u32,
        size: u32,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer ) ]
    #[doc = "The `copyBufferToBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_buffer_to_buffer_with_f64_and_u32_and_u32(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: f64,
        destination: &GpuBuffer,
        destination_offset: u32,
        size: u32,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer ) ]
    #[doc = "The `copyBufferToBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_buffer_to_buffer_with_u32_and_f64_and_u32(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: u32,
        destination: &GpuBuffer,
        destination_offset: f64,
        size: u32,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer ) ]
    #[doc = "The `copyBufferToBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_buffer_to_buffer_with_f64_and_f64_and_u32(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: f64,
        destination: &GpuBuffer,
        destination_offset: f64,
        size: u32,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer ) ]
    #[doc = "The `copyBufferToBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_buffer_to_buffer_with_u32_and_u32_and_f64(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: u32,
        destination: &GpuBuffer,
        destination_offset: u32,
        size: f64,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer ) ]
    #[doc = "The `copyBufferToBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_buffer_to_buffer_with_f64_and_u32_and_f64(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: f64,
        destination: &GpuBuffer,
        destination_offset: u32,
        size: f64,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer ) ]
    #[doc = "The `copyBufferToBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_buffer_to_buffer_with_u32_and_f64_and_f64(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: u32,
        destination: &GpuBuffer,
        destination_offset: f64,
        size: f64,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer ) ]
    #[doc = "The `copyBufferToBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_buffer_to_buffer_with_f64_and_f64_and_f64(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: f64,
        destination: &GpuBuffer,
        destination_offset: f64,
        size: f64,
    );
    #[cfg(all(feature = "GpuBufferCopyView", feature = "GpuTextureCopyView",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToTexture ) ]
    #[doc = "The `copyBufferToTexture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToTexture)\n\n*This API requires the following crate features to be activated: `GpuBufferCopyView`, `GpuCommandEncoder`, `GpuTextureCopyView`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_buffer_to_texture_with_u32_sequence(
        this: &GpuCommandEncoder,
        source: &GpuBufferCopyView,
        destination: &GpuTextureCopyView,
        copy_size: &::wasm_bindgen::JsValue,
    );
    #[cfg(all(
        feature = "GpuBufferCopyView",
        feature = "GpuExtent3dDict",
        feature = "GpuTextureCopyView",
    ))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToTexture ) ]
    #[doc = "The `copyBufferToTexture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToTexture)\n\n*This API requires the following crate features to be activated: `GpuBufferCopyView`, `GpuCommandEncoder`, `GpuExtent3dDict`, `GpuTextureCopyView`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_buffer_to_texture_with_gpu_extent_3d_dict(
        this: &GpuCommandEncoder,
        source: &GpuBufferCopyView,
        destination: &GpuTextureCopyView,
        copy_size: &GpuExtent3dDict,
    );
    #[cfg(all(feature = "GpuBufferCopyView", feature = "GpuTextureCopyView",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyTextureToBuffer ) ]
    #[doc = "The `copyTextureToBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToBuffer)\n\n*This API requires the following crate features to be activated: `GpuBufferCopyView`, `GpuCommandEncoder`, `GpuTextureCopyView`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_texture_to_buffer_with_u32_sequence(
        this: &GpuCommandEncoder,
        source: &GpuTextureCopyView,
        destination: &GpuBufferCopyView,
        copy_size: &::wasm_bindgen::JsValue,
    );
    #[cfg(all(
        feature = "GpuBufferCopyView",
        feature = "GpuExtent3dDict",
        feature = "GpuTextureCopyView",
    ))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyTextureToBuffer ) ]
    #[doc = "The `copyTextureToBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToBuffer)\n\n*This API requires the following crate features to be activated: `GpuBufferCopyView`, `GpuCommandEncoder`, `GpuExtent3dDict`, `GpuTextureCopyView`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_texture_to_buffer_with_gpu_extent_3d_dict(
        this: &GpuCommandEncoder,
        source: &GpuTextureCopyView,
        destination: &GpuBufferCopyView,
        copy_size: &GpuExtent3dDict,
    );
    #[cfg(feature = "GpuTextureCopyView")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyTextureToTexture ) ]
    #[doc = "The `copyTextureToTexture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToTexture)\n\n*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuTextureCopyView`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_texture_to_texture_with_u32_sequence(
        this: &GpuCommandEncoder,
        source: &GpuTextureCopyView,
        destination: &GpuTextureCopyView,
        copy_size: &::wasm_bindgen::JsValue,
    );
    #[cfg(all(feature = "GpuExtent3dDict", feature = "GpuTextureCopyView",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = copyTextureToTexture ) ]
    #[doc = "The `copyTextureToTexture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToTexture)\n\n*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuExtent3dDict`, `GpuTextureCopyView`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_texture_to_texture_with_gpu_extent_3d_dict(
        this: &GpuCommandEncoder,
        source: &GpuTextureCopyView,
        destination: &GpuTextureCopyView,
        copy_size: &GpuExtent3dDict,
    );
    #[cfg(feature = "GpuCommandBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = finish ) ]
    #[doc = "The `finish()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/finish)\n\n*This API requires the following crate features to be activated: `GpuCommandBuffer`, `GpuCommandEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn finish(this: &GpuCommandEncoder) -> GpuCommandBuffer;
    #[cfg(all(feature = "GpuCommandBuffer", feature = "GpuCommandBufferDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = finish ) ]
    #[doc = "The `finish()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/finish)\n\n*This API requires the following crate features to be activated: `GpuCommandBuffer`, `GpuCommandBufferDescriptor`, `GpuCommandEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn finish_with_descriptor(
        this: &GpuCommandEncoder,
        descriptor: &GpuCommandBufferDescriptor,
    ) -> GpuCommandBuffer;
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = insertDebugMarker ) ]
    #[doc = "The `insertDebugMarker()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/insertDebugMarker)\n\n*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn insert_debug_marker(this: &GpuCommandEncoder, marker_label: &str);
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = popDebugGroup ) ]
    #[doc = "The `popDebugGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/popDebugGroup)\n\n*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn pop_debug_group(this: &GpuCommandEncoder);
    # [ wasm_bindgen ( method , structural , js_class = "GPUCommandEncoder" , js_name = pushDebugGroup ) ]
    #[doc = "The `pushDebugGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/pushDebugGroup)\n\n*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn push_debug_group(this: &GpuCommandEncoder, group_label: &str);
}
