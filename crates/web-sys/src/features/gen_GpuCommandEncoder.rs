#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUCommandEncoder , typescript_type = "GPUCommandEncoder")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuCommandEncoder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuCommandEncoder;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUCommandEncoder" , js_name = label)]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuCommandEncoder) -> Option<String>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , setter , js_class = "GPUCommandEncoder" , js_name = label)]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuCommandEncoder, value: Option<&str>);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuComputePassEncoder")]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = beginComputePass)]
    #[doc = "The `beginComputePass()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/beginComputePass)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn begin_compute_pass(this: &GpuCommandEncoder) -> GpuComputePassEncoder;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuComputePassDescriptor",
        feature = "GpuComputePassEncoder",
    ))]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = beginComputePass)]
    #[doc = "The `beginComputePass()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/beginComputePass)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuComputePassDescriptor`, `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn begin_compute_pass_with_descriptor(
        this: &GpuCommandEncoder,
        descriptor: &GpuComputePassDescriptor,
    ) -> GpuComputePassEncoder;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuRenderPassDescriptor", feature = "GpuRenderPassEncoder",))]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = beginRenderPass)]
    #[doc = "The `beginRenderPass()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/beginRenderPass)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuRenderPassDescriptor`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn begin_render_pass(
        this: &GpuCommandEncoder,
        descriptor: &GpuRenderPassDescriptor,
    ) -> GpuRenderPassEncoder;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer)]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
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
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer)]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
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
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer)]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
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
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer)]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
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
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer)]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
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
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer)]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
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
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer)]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
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
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToBuffer)]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_buffer_to_buffer_with_f64_and_f64_and_f64(
        this: &GpuCommandEncoder,
        source: &GpuBuffer,
        source_offset: f64,
        destination: &GpuBuffer,
        destination_offset: f64,
        size: f64,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuImageCopyBuffer", feature = "GpuImageCopyTexture",))]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToTexture)]
    #[doc = "The `copyBufferToTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuImageCopyBuffer`, `GpuImageCopyTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_buffer_to_texture_with_u32_sequence(
        this: &GpuCommandEncoder,
        source: &GpuImageCopyBuffer,
        destination: &GpuImageCopyTexture,
        copy_size: &::wasm_bindgen::JsValue,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuExtent3dDict",
        feature = "GpuImageCopyBuffer",
        feature = "GpuImageCopyTexture",
    ))]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = copyBufferToTexture)]
    #[doc = "The `copyBufferToTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuExtent3dDict`, `GpuImageCopyBuffer`, `GpuImageCopyTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_buffer_to_texture_with_gpu_extent_3d_dict(
        this: &GpuCommandEncoder,
        source: &GpuImageCopyBuffer,
        destination: &GpuImageCopyTexture,
        copy_size: &GpuExtent3dDict,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuImageCopyBuffer", feature = "GpuImageCopyTexture",))]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = copyTextureToBuffer)]
    #[doc = "The `copyTextureToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuImageCopyBuffer`, `GpuImageCopyTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_texture_to_buffer_with_u32_sequence(
        this: &GpuCommandEncoder,
        source: &GpuImageCopyTexture,
        destination: &GpuImageCopyBuffer,
        copy_size: &::wasm_bindgen::JsValue,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuExtent3dDict",
        feature = "GpuImageCopyBuffer",
        feature = "GpuImageCopyTexture",
    ))]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = copyTextureToBuffer)]
    #[doc = "The `copyTextureToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuExtent3dDict`, `GpuImageCopyBuffer`, `GpuImageCopyTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_texture_to_buffer_with_gpu_extent_3d_dict(
        this: &GpuCommandEncoder,
        source: &GpuImageCopyTexture,
        destination: &GpuImageCopyBuffer,
        copy_size: &GpuExtent3dDict,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuImageCopyTexture")]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = copyTextureToTexture)]
    #[doc = "The `copyTextureToTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuImageCopyTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_texture_to_texture_with_u32_sequence(
        this: &GpuCommandEncoder,
        source: &GpuImageCopyTexture,
        destination: &GpuImageCopyTexture,
        copy_size: &::wasm_bindgen::JsValue,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuExtent3dDict", feature = "GpuImageCopyTexture",))]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = copyTextureToTexture)]
    #[doc = "The `copyTextureToTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuExtent3dDict`, `GpuImageCopyTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_texture_to_texture_with_gpu_extent_3d_dict(
        this: &GpuCommandEncoder,
        source: &GpuImageCopyTexture,
        destination: &GpuImageCopyTexture,
        copy_size: &GpuExtent3dDict,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCommandBuffer")]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = finish)]
    #[doc = "The `finish()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/finish)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn finish(this: &GpuCommandEncoder) -> GpuCommandBuffer;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuCommandBuffer", feature = "GpuCommandBufferDescriptor",))]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = finish)]
    #[doc = "The `finish()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/finish)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandBuffer`, `GpuCommandBufferDescriptor`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn finish_with_descriptor(
        this: &GpuCommandEncoder,
        descriptor: &GpuCommandBufferDescriptor,
    ) -> GpuCommandBuffer;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = insertDebugMarker)]
    #[doc = "The `insertDebugMarker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/insertDebugMarker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn insert_debug_marker(this: &GpuCommandEncoder, marker_label: &str);
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = popDebugGroup)]
    #[doc = "The `popDebugGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/popDebugGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn pop_debug_group(this: &GpuCommandEncoder);
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = pushDebugGroup)]
    #[doc = "The `pushDebugGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/pushDebugGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn push_debug_group(this: &GpuCommandEncoder, group_label: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBuffer", feature = "GpuQuerySet",))]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = resolveQuerySet)]
    #[doc = "The `resolveQuerySet()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/resolveQuerySet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`, `GpuQuerySet`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn resolve_query_set_with_u32(
        this: &GpuCommandEncoder,
        query_set: &GpuQuerySet,
        first_query: u32,
        query_count: u32,
        destination: &GpuBuffer,
        destination_offset: u32,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBuffer", feature = "GpuQuerySet",))]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = resolveQuerySet)]
    #[doc = "The `resolveQuerySet()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/resolveQuerySet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`, `GpuQuerySet`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn resolve_query_set_with_f64(
        this: &GpuCommandEncoder,
        query_set: &GpuQuerySet,
        first_query: u32,
        query_count: u32,
        destination: &GpuBuffer,
        destination_offset: f64,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQuerySet")]
    # [wasm_bindgen (method , structural , js_class = "GPUCommandEncoder" , js_name = writeTimestamp)]
    #[doc = "The `writeTimestamp()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/writeTimestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuQuerySet`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_timestamp(this: &GpuCommandEncoder, query_set: &GpuQuerySet, query_index: u32);
}
