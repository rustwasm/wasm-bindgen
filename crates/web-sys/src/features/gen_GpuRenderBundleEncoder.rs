use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPURenderBundleEncoder , typescript_type = "GPURenderBundleEncoder" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuRenderBundleEncoder` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuRenderBundleEncoder;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPURenderBundleEncoder" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn label(this: &GpuRenderBundleEncoder) -> Option<String>;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPURenderBundleEncoder" , js_name = label ) ]
    ///Setter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_label(this: &GpuRenderBundleEncoder, value: Option<&str>);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderBundle")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = finish ) ]
    ///The `finish()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/finish)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderBundle`, `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn finish(this: &GpuRenderBundleEncoder) -> GpuRenderBundle;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuRenderBundle", feature = "GpuRenderBundleDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = finish ) ]
    ///The `finish()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/finish)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderBundle`, `GpuRenderBundleDescriptor`, `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn finish_with_descriptor(
        this: &GpuRenderBundleEncoder,
        descriptor: &GpuRenderBundleDescriptor,
    ) -> GpuRenderBundle;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = insertDebugMarker ) ]
    ///The `insertDebugMarker()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/insertDebugMarker)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn insert_debug_marker(this: &GpuRenderBundleEncoder, marker_label: &str);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = popDebugGroup ) ]
    ///The `popDebugGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/popDebugGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn pop_debug_group(this: &GpuRenderBundleEncoder);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = pushDebugGroup ) ]
    ///The `pushDebugGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/pushDebugGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn push_debug_group(this: &GpuRenderBundleEncoder, group_label: &str);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setBindGroup ) ]
    ///The `setBindGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setBindGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_bind_group(this: &GpuRenderBundleEncoder, index: u32, bind_group: &GpuBindGroup);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setBindGroup ) ]
    ///The `setBindGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setBindGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_bind_group_with_u32_sequence(
        this: &GpuRenderBundleEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets: &::wasm_bindgen::JsValue,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setBindGroup ) ]
    ///The `setBindGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setBindGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_bind_group_with_u32_array_and_u32_and_dynamic_offsets_data_length(
        this: &GpuRenderBundleEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets_data: &mut [u32],
        dynamic_offsets_data_start: u32,
        dynamic_offsets_data_length: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setBindGroup ) ]
    ///The `setBindGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setBindGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_bind_group_with_u32_array_and_f64_and_dynamic_offsets_data_length(
        this: &GpuRenderBundleEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets_data: &mut [u32],
        dynamic_offsets_data_start: f64,
        dynamic_offsets_data_length: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = draw ) ]
    ///The `draw()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/draw)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn draw(
        this: &GpuRenderBundleEncoder,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = drawIndexed ) ]
    ///The `drawIndexed()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndexed)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn draw_indexed(
        this: &GpuRenderBundleEncoder,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        base_vertex: i32,
        first_instance: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = drawIndexedIndirect ) ]
    ///The `drawIndexedIndirect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndexedIndirect)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn draw_indexed_indirect_with_u32(
        this: &GpuRenderBundleEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = drawIndexedIndirect ) ]
    ///The `drawIndexedIndirect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndexedIndirect)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn draw_indexed_indirect_with_f64(
        this: &GpuRenderBundleEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: f64,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = drawIndirect ) ]
    ///The `drawIndirect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndirect)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn draw_indirect_with_u32(
        this: &GpuRenderBundleEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = drawIndirect ) ]
    ///The `drawIndirect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndirect)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn draw_indirect_with_f64(
        this: &GpuRenderBundleEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: f64,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setIndexBuffer ) ]
    ///The `setIndexBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setIndexBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_index_buffer(this: &GpuRenderBundleEncoder, buffer: &GpuBuffer);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setIndexBuffer ) ]
    ///The `setIndexBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setIndexBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_index_buffer_with_u32(
        this: &GpuRenderBundleEncoder,
        buffer: &GpuBuffer,
        offset: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setIndexBuffer ) ]
    ///The `setIndexBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setIndexBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_index_buffer_with_f64(
        this: &GpuRenderBundleEncoder,
        buffer: &GpuBuffer,
        offset: f64,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPipeline")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setPipeline ) ]
    ///The `setPipeline()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setPipeline)
    ///
    ///*This API requires the following crate features to be activated: `GpuRenderBundleEncoder`, `GpuRenderPipeline`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_pipeline(this: &GpuRenderBundleEncoder, pipeline: &GpuRenderPipeline);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setVertexBuffer ) ]
    ///The `setVertexBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setVertexBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_vertex_buffer(this: &GpuRenderBundleEncoder, slot: u32, buffer: &GpuBuffer);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setVertexBuffer ) ]
    ///The `setVertexBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setVertexBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_vertex_buffer_with_u32(
        this: &GpuRenderBundleEncoder,
        slot: u32,
        buffer: &GpuBuffer,
        offset: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPURenderBundleEncoder" , js_name = setVertexBuffer ) ]
    ///The `setVertexBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setVertexBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderBundleEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_vertex_buffer_with_f64(
        this: &GpuRenderBundleEncoder,
        slot: u32,
        buffer: &GpuBuffer,
        offset: f64,
    );

}
