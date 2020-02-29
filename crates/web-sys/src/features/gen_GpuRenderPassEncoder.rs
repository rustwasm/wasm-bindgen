use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPURenderPassEncoder , typescript_type = "GPURenderPassEncoder" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuRenderPassEncoder` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuRenderPassEncoder;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPURenderPassEncoder" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn label(this: &GpuRenderPassEncoder) -> Option<String>;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPURenderPassEncoder" , js_name = label ) ]
    ///Setter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_label(this: &GpuRenderPassEncoder, value: Option<&str>);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = endPass ) ]
    ///The `endPass()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/endPass)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn end_pass(this: &GpuRenderPassEncoder);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = executeBundles ) ]
    ///The `executeBundles()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/executeBundles)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn execute_bundles(this: &GpuRenderPassEncoder, bundles: &::wasm_bindgen::JsValue);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBlendColor ) ]
    ///The `setBlendColor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBlendColor)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_blend_color_with_f64_sequence(
        this: &GpuRenderPassEncoder,
        color: &::wasm_bindgen::JsValue,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuColorDict")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBlendColor ) ]
    ///The `setBlendColor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBlendColor)
    ///
    ///*This API requires the following crate features to be activated: `GpuColorDict`, `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_blend_color_with_gpu_color_dict(this: &GpuRenderPassEncoder, color: &GpuColorDict);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setScissorRect ) ]
    ///The `setScissorRect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setScissorRect)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_scissor_rect(this: &GpuRenderPassEncoder, x: u32, y: u32, width: u32, height: u32);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setStencilReference ) ]
    ///The `setStencilReference()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setStencilReference)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_stencil_reference(this: &GpuRenderPassEncoder, reference: u32);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setViewport ) ]
    ///The `setViewport()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setViewport)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_viewport(
        this: &GpuRenderPassEncoder,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    );

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = insertDebugMarker ) ]
    ///The `insertDebugMarker()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/insertDebugMarker)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn insert_debug_marker(this: &GpuRenderPassEncoder, marker_label: &str);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = popDebugGroup ) ]
    ///The `popDebugGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/popDebugGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn pop_debug_group(this: &GpuRenderPassEncoder);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = pushDebugGroup ) ]
    ///The `pushDebugGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/pushDebugGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn push_debug_group(this: &GpuRenderPassEncoder, group_label: &str);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBindGroup ) ]
    ///The `setBindGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_bind_group(this: &GpuRenderPassEncoder, index: u32, bind_group: &GpuBindGroup);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBindGroup ) ]
    ///The `setBindGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_bind_group_with_u32_sequence(
        this: &GpuRenderPassEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets: &::wasm_bindgen::JsValue,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBindGroup ) ]
    ///The `setBindGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_bind_group_with_u32_array_and_u32_and_dynamic_offsets_data_length(
        this: &GpuRenderPassEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets_data: &mut [u32],
        dynamic_offsets_data_start: u32,
        dynamic_offsets_data_length: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBindGroup ) ]
    ///The `setBindGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_bind_group_with_u32_array_and_f64_and_dynamic_offsets_data_length(
        this: &GpuRenderPassEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets_data: &mut [u32],
        dynamic_offsets_data_start: f64,
        dynamic_offsets_data_length: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = draw ) ]
    ///The `draw()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/draw)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn draw(
        this: &GpuRenderPassEncoder,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = drawIndexed ) ]
    ///The `drawIndexed()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexed)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn draw_indexed(
        this: &GpuRenderPassEncoder,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        base_vertex: i32,
        first_instance: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = drawIndexedIndirect ) ]
    ///The `drawIndexedIndirect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexedIndirect)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn draw_indexed_indirect_with_u32(
        this: &GpuRenderPassEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = drawIndexedIndirect ) ]
    ///The `drawIndexedIndirect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexedIndirect)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn draw_indexed_indirect_with_f64(
        this: &GpuRenderPassEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: f64,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = drawIndirect ) ]
    ///The `drawIndirect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndirect)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn draw_indirect_with_u32(
        this: &GpuRenderPassEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = drawIndirect ) ]
    ///The `drawIndirect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndirect)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn draw_indirect_with_f64(
        this: &GpuRenderPassEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: f64,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setIndexBuffer ) ]
    ///The `setIndexBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_index_buffer(this: &GpuRenderPassEncoder, buffer: &GpuBuffer);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setIndexBuffer ) ]
    ///The `setIndexBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_index_buffer_with_u32(this: &GpuRenderPassEncoder, buffer: &GpuBuffer, offset: u32);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setIndexBuffer ) ]
    ///The `setIndexBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_index_buffer_with_f64(this: &GpuRenderPassEncoder, buffer: &GpuBuffer, offset: f64);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPipeline")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setPipeline ) ]
    ///The `setPipeline()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setPipeline)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderPassEncoder`, `GpuRenderPipeline`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_pipeline(this: &GpuRenderPassEncoder, pipeline: &GpuRenderPipeline);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setVertexBuffer ) ]
    ///The `setVertexBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_vertex_buffer(this: &GpuRenderPassEncoder, slot: u32, buffer: &GpuBuffer);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setVertexBuffer ) ]
    ///The `setVertexBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_vertex_buffer_with_u32(
        this: &GpuRenderPassEncoder,
        slot: u32,
        buffer: &GpuBuffer,
        offset: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setVertexBuffer ) ]
    ///The `setVertexBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_vertex_buffer_with_f64(
        this: &GpuRenderPassEncoder,
        slot: u32,
        buffer: &GpuBuffer,
        offset: f64,
    );

}
