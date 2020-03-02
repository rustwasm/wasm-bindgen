#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPURenderPassEncoder , typescript_type = "GPURenderPassEncoder" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuRenderPassEncoder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuRenderPassEncoder;
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPURenderPassEncoder" , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuRenderPassEncoder) -> Option<String>;
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPURenderPassEncoder" , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuRenderPassEncoder, value: Option<&str>);
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = endPass ) ]
    #[doc = "The `endPass()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/endPass)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn end_pass(this: &GpuRenderPassEncoder);
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = executeBundles ) ]
    #[doc = "The `executeBundles()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/executeBundles)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn execute_bundles(this: &GpuRenderPassEncoder, bundles: &::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBlendColor ) ]
    #[doc = "The `setBlendColor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBlendColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_blend_color_with_f64_sequence(
        this: &GpuRenderPassEncoder,
        color: &::wasm_bindgen::JsValue,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuColorDict")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBlendColor ) ]
    #[doc = "The `setBlendColor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBlendColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorDict`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_blend_color_with_gpu_color_dict(this: &GpuRenderPassEncoder, color: &GpuColorDict);
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setScissorRect ) ]
    #[doc = "The `setScissorRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setScissorRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_scissor_rect(this: &GpuRenderPassEncoder, x: u32, y: u32, width: u32, height: u32);
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setStencilReference ) ]
    #[doc = "The `setStencilReference()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setStencilReference)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_stencil_reference(this: &GpuRenderPassEncoder, reference: u32);
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setViewport ) ]
    #[doc = "The `setViewport()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setViewport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
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
    #[doc = "The `insertDebugMarker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/insertDebugMarker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn insert_debug_marker(this: &GpuRenderPassEncoder, marker_label: &str);
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = popDebugGroup ) ]
    #[doc = "The `popDebugGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/popDebugGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn pop_debug_group(this: &GpuRenderPassEncoder);
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = pushDebugGroup ) ]
    #[doc = "The `pushDebugGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/pushDebugGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn push_debug_group(this: &GpuRenderPassEncoder, group_label: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBindGroup ) ]
    #[doc = "The `setBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_bind_group(this: &GpuRenderPassEncoder, index: u32, bind_group: &GpuBindGroup);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBindGroup ) ]
    #[doc = "The `setBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_bind_group_with_u32_sequence(
        this: &GpuRenderPassEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets: &::wasm_bindgen::JsValue,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setBindGroup ) ]
    #[doc = "The `setBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
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
    #[doc = "The `setBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
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
    #[doc = "The `draw()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/draw)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw(
        this: &GpuRenderPassEncoder,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    );
    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = drawIndexed ) ]
    #[doc = "The `drawIndexed()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
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
    #[doc = "The `drawIndexedIndirect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexedIndirect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw_indexed_indirect_with_u32(
        this: &GpuRenderPassEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: u32,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = drawIndexedIndirect ) ]
    #[doc = "The `drawIndexedIndirect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexedIndirect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw_indexed_indirect_with_f64(
        this: &GpuRenderPassEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: f64,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = drawIndirect ) ]
    #[doc = "The `drawIndirect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndirect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw_indirect_with_u32(
        this: &GpuRenderPassEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: u32,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = drawIndirect ) ]
    #[doc = "The `drawIndirect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndirect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw_indirect_with_f64(
        this: &GpuRenderPassEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: f64,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setIndexBuffer ) ]
    #[doc = "The `setIndexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_index_buffer(this: &GpuRenderPassEncoder, buffer: &GpuBuffer);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setIndexBuffer ) ]
    #[doc = "The `setIndexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_index_buffer_with_u32(this: &GpuRenderPassEncoder, buffer: &GpuBuffer, offset: u32);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setIndexBuffer ) ]
    #[doc = "The `setIndexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_index_buffer_with_f64(this: &GpuRenderPassEncoder, buffer: &GpuBuffer, offset: f64);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPipeline")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setPipeline ) ]
    #[doc = "The `setPipeline()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setPipeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`, `GpuRenderPipeline`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_pipeline(this: &GpuRenderPassEncoder, pipeline: &GpuRenderPipeline);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setVertexBuffer ) ]
    #[doc = "The `setVertexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_vertex_buffer(this: &GpuRenderPassEncoder, slot: u32, buffer: &GpuBuffer);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setVertexBuffer ) ]
    #[doc = "The `setVertexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_vertex_buffer_with_u32(
        this: &GpuRenderPassEncoder,
        slot: u32,
        buffer: &GpuBuffer,
        offset: u32,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderPassEncoder" , js_name = setVertexBuffer ) ]
    #[doc = "The `setVertexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_vertex_buffer_with_f64(
        this: &GpuRenderPassEncoder,
        slot: u32,
        buffer: &GpuBuffer,
        offset: f64,
    );
}
