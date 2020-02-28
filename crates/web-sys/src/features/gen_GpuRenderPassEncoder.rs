use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPURenderPassEncoder , typescript_name = GPURenderPassEncoder ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuRenderPassEncoder` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder)\n\n*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    pub type GpuRenderPassEncoder;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPURenderPassEncoder" , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/label)\n\n*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuRenderPassEncoder) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPURenderPassEncoder" , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/label)\n\n*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuRenderPassEncoder, value: Option<&str>);
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = endPass ) ]
    #[doc = "The `endPass()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/endPass)\n\n*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn end_pass(this: &GpuRenderPassEncoder);
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = executeBundles ) ]
    #[doc = "The `executeBundles()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/executeBundles)\n\n*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn execute_bundles(this: &GpuRenderPassEncoder, bundles: &::wasm_bindgen::JsValue);
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBlendColor ) ]
    #[doc = "The `setBlendColor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBlendColor)\n\n*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_blend_color_with_f64_sequence(
        this: &GpuRenderPassEncoder,
        color: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "GpuColorDict")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBlendColor ) ]
    #[doc = "The `setBlendColor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBlendColor)\n\n*This API requires the following crate features to be activated: `GpuColorDict`, `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_blend_color_with_gpu_color_dict(this: &GpuRenderPassEncoder, color: &GpuColorDict);
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setScissorRect ) ]
    #[doc = "The `setScissorRect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setScissorRect)\n\n*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_scissor_rect(this: &GpuRenderPassEncoder, x: u32, y: u32, width: u32, height: u32);
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setStencilReference ) ]
    #[doc = "The `setStencilReference()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setStencilReference)\n\n*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_stencil_reference(this: &GpuRenderPassEncoder, reference: u32);
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setViewport ) ]
    #[doc = "The `setViewport()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setViewport)\n\n*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_viewport(
        this: &GpuRenderPassEncoder,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    );
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = insertDebugMarker ) ]
    #[doc = "The `insertDebugMarker()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/insertDebugMarker)\n\n*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn insert_debug_marker(this: &GpuRenderPassEncoder, marker_label: &str);
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = popDebugGroup ) ]
    #[doc = "The `popDebugGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/popDebugGroup)\n\n*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn pop_debug_group(this: &GpuRenderPassEncoder);
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = pushDebugGroup ) ]
    #[doc = "The `pushDebugGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/pushDebugGroup)\n\n*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn push_debug_group(this: &GpuRenderPassEncoder, group_label: &str);
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBindGroup ) ]
    #[doc = "The `setBindGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)\n\n*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_bind_group(this: &GpuRenderPassEncoder, index: u32, bind_group: &GpuBindGroup);
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBindGroup ) ]
    #[doc = "The `setBindGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)\n\n*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_bind_group_with_u32_sequence(
        this: &GpuRenderPassEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBindGroup ) ]
    #[doc = "The `setBindGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)\n\n*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_bind_group_with_u32_array_and_u32_and_dynamic_offsets_data_length(
        this: &GpuRenderPassEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets_data: &mut [u32],
        dynamic_offsets_data_start: u32,
        dynamic_offsets_data_length: u32,
    );
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBindGroup ) ]
    #[doc = "The `setBindGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)\n\n*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_bind_group_with_u32_array_and_f64_and_dynamic_offsets_data_length(
        this: &GpuRenderPassEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets_data: &mut [u32],
        dynamic_offsets_data_start: f64,
        dynamic_offsets_data_length: u32,
    );
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = draw ) ]
    #[doc = "The `draw()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/draw)\n\n*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw(
        this: &GpuRenderPassEncoder,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    );
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = drawIndexed ) ]
    #[doc = "The `drawIndexed()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexed)\n\n*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw_indexed(
        this: &GpuRenderPassEncoder,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        base_vertex: i32,
        first_instance: u32,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = drawIndexedIndirect ) ]
    #[doc = "The `drawIndexedIndirect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexedIndirect)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw_indexed_indirect_with_u32(
        this: &GpuRenderPassEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: u32,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = drawIndexedIndirect ) ]
    #[doc = "The `drawIndexedIndirect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexedIndirect)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw_indexed_indirect_with_f64(
        this: &GpuRenderPassEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: f64,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = drawIndirect ) ]
    #[doc = "The `drawIndirect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndirect)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw_indirect_with_u32(
        this: &GpuRenderPassEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: u32,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = drawIndirect ) ]
    #[doc = "The `drawIndirect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndirect)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw_indirect_with_f64(
        this: &GpuRenderPassEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: f64,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setIndexBuffer ) ]
    #[doc = "The `setIndexBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_index_buffer(this: &GpuRenderPassEncoder, buffer: &GpuBuffer);
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setIndexBuffer ) ]
    #[doc = "The `setIndexBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_index_buffer_with_u32(this: &GpuRenderPassEncoder, buffer: &GpuBuffer, offset: u32);
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setIndexBuffer ) ]
    #[doc = "The `setIndexBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_index_buffer_with_f64(this: &GpuRenderPassEncoder, buffer: &GpuBuffer, offset: f64);
    #[cfg(feature = "GpuRenderPipeline")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setPipeline ) ]
    #[doc = "The `setPipeline()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setPipeline)\n\n*This API requires the following crate features to be activated: `GpuRenderPassEncoder`, `GpuRenderPipeline`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_pipeline(this: &GpuRenderPassEncoder, pipeline: &GpuRenderPipeline);
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setVertexBuffer ) ]
    #[doc = "The `setVertexBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_vertex_buffer(this: &GpuRenderPassEncoder, slot: u32, buffer: &GpuBuffer);
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setVertexBuffer ) ]
    #[doc = "The `setVertexBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_vertex_buffer_with_u32(
        this: &GpuRenderPassEncoder,
        slot: u32,
        buffer: &GpuBuffer,
        offset: u32,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setVertexBuffer ) ]
    #[doc = "The `setVertexBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_vertex_buffer_with_f64(
        this: &GpuRenderPassEncoder,
        slot: u32,
        buffer: &GpuBuffer,
        offset: f64,
    );
}
