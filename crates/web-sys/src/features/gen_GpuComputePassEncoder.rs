use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUComputePassEncoder , typescript_name = GPUComputePassEncoder ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuComputePassEncoder` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder)\n\n*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    pub type GpuComputePassEncoder;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUComputePassEncoder" , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/label)\n\n*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuComputePassEncoder) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPUComputePassEncoder" , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/label)\n\n*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuComputePassEncoder, value: Option<&str>);
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = dispatch ) ]
    #[doc = "The `dispatch()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatch)\n\n*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn dispatch(this: &GpuComputePassEncoder, x: u32);
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = dispatch ) ]
    #[doc = "The `dispatch()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatch)\n\n*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn dispatch_with_y(this: &GpuComputePassEncoder, x: u32, y: u32);
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = dispatch ) ]
    #[doc = "The `dispatch()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatch)\n\n*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn dispatch_with_y_and_z(this: &GpuComputePassEncoder, x: u32, y: u32, z: u32);
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = dispatchIndirect ) ]
    #[doc = "The `dispatchIndirect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatchIndirect)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn dispatch_indirect_with_u32(
        this: &GpuComputePassEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: u32,
    );
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = dispatchIndirect ) ]
    #[doc = "The `dispatchIndirect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatchIndirect)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn dispatch_indirect_with_f64(
        this: &GpuComputePassEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: f64,
    );
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = endPass ) ]
    #[doc = "The `endPass()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/endPass)\n\n*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn end_pass(this: &GpuComputePassEncoder);
    #[cfg(feature = "GpuComputePipeline")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = setPipeline ) ]
    #[doc = "The `setPipeline()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setPipeline)\n\n*This API requires the following crate features to be activated: `GpuComputePassEncoder`, `GpuComputePipeline`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_pipeline(this: &GpuComputePassEncoder, pipeline: &GpuComputePipeline);
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = insertDebugMarker ) ]
    #[doc = "The `insertDebugMarker()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/insertDebugMarker)\n\n*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn insert_debug_marker(this: &GpuComputePassEncoder, marker_label: &str);
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = popDebugGroup ) ]
    #[doc = "The `popDebugGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/popDebugGroup)\n\n*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn pop_debug_group(this: &GpuComputePassEncoder);
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = pushDebugGroup ) ]
    #[doc = "The `pushDebugGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/pushDebugGroup)\n\n*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn push_debug_group(this: &GpuComputePassEncoder, group_label: &str);
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = setBindGroup ) ]
    #[doc = "The `setBindGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setBindGroup)\n\n*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_bind_group(this: &GpuComputePassEncoder, index: u32, bind_group: &GpuBindGroup);
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = setBindGroup ) ]
    #[doc = "The `setBindGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setBindGroup)\n\n*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_bind_group_with_u32_sequence(
        this: &GpuComputePassEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = setBindGroup ) ]
    #[doc = "The `setBindGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setBindGroup)\n\n*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_bind_group_with_u32_array_and_u32_and_dynamic_offsets_data_length(
        this: &GpuComputePassEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets_data: &mut [u32],
        dynamic_offsets_data_start: u32,
        dynamic_offsets_data_length: u32,
    );
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = setBindGroup ) ]
    #[doc = "The `setBindGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setBindGroup)\n\n*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuComputePassEncoder`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_bind_group_with_u32_array_and_f64_and_dynamic_offsets_data_length(
        this: &GpuComputePassEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets_data: &mut [u32],
        dynamic_offsets_data_start: f64,
        dynamic_offsets_data_length: u32,
    );
}
