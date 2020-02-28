use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPURenderBundleEncoder , typescript_name = GPURenderBundleEncoder ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuRenderBundleEncoder` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder)\n\n*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`*"]
    pub type GpuRenderBundleEncoder;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPURenderBundleEncoder" , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/label)\n\n*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuRenderBundleEncoder) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPURenderBundleEncoder" , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/label)\n\n*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuRenderBundleEncoder, value: Option<&str>);
    #[cfg(feature = "GpuRenderBundle")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = finish ) ]
    #[doc = "The `finish()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/finish)\n\n*This API requires the following crate features to be activated: `GpuRenderBundle`, `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn finish(this: &GpuRenderBundleEncoder) -> GpuRenderBundle;
    #[cfg(all(feature = "GpuRenderBundle", feature = "GpuRenderBundleDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = finish ) ]
    #[doc = "The `finish()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/finish)\n\n*This API requires the following crate features to be activated: `GpuRenderBundle`, `GpuRenderBundleDescriptor`, `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn finish_with_descriptor(
        this: &GpuRenderBundleEncoder,
        descriptor: &GpuRenderBundleDescriptor,
    ) -> GpuRenderBundle;
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = insertDebugMarker ) ]
    #[doc = "The `insertDebugMarker()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/insertDebugMarker)\n\n*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn insert_debug_marker(this: &GpuRenderBundleEncoder, marker_label: &str);
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = popDebugGroup ) ]
    #[doc = "The `popDebugGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/popDebugGroup)\n\n*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn pop_debug_group(this: &GpuRenderBundleEncoder);
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = pushDebugGroup ) ]
    #[doc = "The `pushDebugGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/pushDebugGroup)\n\n*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn push_debug_group(this: &GpuRenderBundleEncoder, group_label: &str);
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setBindGroup ) ]
    #[doc = "The `setBindGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setBindGroup)\n\n*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_bind_group(this: &GpuRenderBundleEncoder, index: u32, bind_group: &GpuBindGroup);
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setBindGroup ) ]
    #[doc = "The `setBindGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setBindGroup)\n\n*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_bind_group_with_u32_sequence(
        this: &GpuRenderBundleEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setBindGroup ) ]
    #[doc = "The `setBindGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setBindGroup)\n\n*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_bind_group_with_u32_array_and_u32_and_dynamic_offsets_data_length(
        this: &GpuRenderBundleEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets_data: &mut [u32],
        dynamic_offsets_data_start: u32,
        dynamic_offsets_data_length: u32,
    );
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setBindGroup ) ]
    #[doc = "The `setBindGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setBindGroup)\n\n*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_bind_group_with_u32_array_and_f64_and_dynamic_offsets_data_length(
        this: &GpuRenderBundleEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets_data: &mut [u32],
        dynamic_offsets_data_start: f64,
        dynamic_offsets_data_length: u32,
    );
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = draw ) ]
    #[doc = "The `draw()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/draw)\n\n*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw(
        this: &GpuRenderBundleEncoder,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    );
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = drawIndexed ) ]
    #[doc = "The `drawIndexed()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndexed)\n\n*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw_indexed(
        this: &GpuRenderBundleEncoder,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        base_vertex: i32,
        first_instance: u32,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = drawIndexedIndirect ) ]
    #[doc = "The `drawIndexedIndirect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndexedIndirect)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw_indexed_indirect_with_u32(
        this: &GpuRenderBundleEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: u32,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = drawIndexedIndirect ) ]
    #[doc = "The `drawIndexedIndirect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndexedIndirect)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw_indexed_indirect_with_f64(
        this: &GpuRenderBundleEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: f64,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = drawIndirect ) ]
    #[doc = "The `drawIndirect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndirect)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw_indirect_with_u32(
        this: &GpuRenderBundleEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: u32,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = drawIndirect ) ]
    #[doc = "The `drawIndirect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndirect)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn draw_indirect_with_f64(
        this: &GpuRenderBundleEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: f64,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setIndexBuffer ) ]
    #[doc = "The `setIndexBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setIndexBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_index_buffer(this: &GpuRenderBundleEncoder, buffer: &GpuBuffer);
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setIndexBuffer ) ]
    #[doc = "The `setIndexBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setIndexBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_index_buffer_with_u32(
        this: &GpuRenderBundleEncoder,
        buffer: &GpuBuffer,
        offset: u32,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setIndexBuffer ) ]
    #[doc = "The `setIndexBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setIndexBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_index_buffer_with_f64(
        this: &GpuRenderBundleEncoder,
        buffer: &GpuBuffer,
        offset: f64,
    );
    #[cfg(feature = "GpuRenderPipeline")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setPipeline ) ]
    #[doc = "The `setPipeline()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setPipeline)\n\n*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`, `GpuRenderPipeline`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_pipeline(this: &GpuRenderBundleEncoder, pipeline: &GpuRenderPipeline);
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setVertexBuffer ) ]
    #[doc = "The `setVertexBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setVertexBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_vertex_buffer(this: &GpuRenderBundleEncoder, slot: u32, buffer: &GpuBuffer);
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setVertexBuffer ) ]
    #[doc = "The `setVertexBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setVertexBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_vertex_buffer_with_u32(
        this: &GpuRenderBundleEncoder,
        slot: u32,
        buffer: &GpuBuffer,
        offset: u32,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setVertexBuffer ) ]
    #[doc = "The `setVertexBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setVertexBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_vertex_buffer_with_f64(
        this: &GpuRenderBundleEncoder,
        slot: u32,
        buffer: &GpuBuffer,
        offset: f64,
    );
}
