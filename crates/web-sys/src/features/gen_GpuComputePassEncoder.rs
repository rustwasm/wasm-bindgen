use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUComputePassEncoder , typescript_name = GPUComputePassEncoder ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuComputePassEncoder` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder)
    ///
    ///*This API requires the following crate features to be activated: `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuComputePassEncoder;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUComputePassEncoder" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn label(this: &GpuComputePassEncoder) -> Option<String>;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPUComputePassEncoder" , js_name = label ) ]
    ///Setter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_label(this: &GpuComputePassEncoder, value: Option<&str>);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = dispatch ) ]
    ///The `dispatch()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatch)
    ///
    ///*This API requires the following crate features to be activated: `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn dispatch(this: &GpuComputePassEncoder, x: u32);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = dispatch ) ]
    ///The `dispatch()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatch)
    ///
    ///*This API requires the following crate features to be activated: `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn dispatch_with_y(this: &GpuComputePassEncoder, x: u32, y: u32);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = dispatch ) ]
    ///The `dispatch()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatch)
    ///
    ///*This API requires the following crate features to be activated: `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn dispatch_with_y_and_z(this: &GpuComputePassEncoder, x: u32, y: u32, z: u32);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = dispatchIndirect ) ]
    ///The `dispatchIndirect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatchIndirect)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn dispatch_indirect_with_u32(
        this: &GpuComputePassEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = dispatchIndirect ) ]
    ///The `dispatchIndirect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatchIndirect)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn dispatch_indirect_with_f64(
        this: &GpuComputePassEncoder,
        indirect_buffer: &GpuBuffer,
        indirect_offset: f64,
    );

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = endPass ) ]
    ///The `endPass()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/endPass)
    ///
    ///*This API requires the following crate features to be activated: `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn end_pass(this: &GpuComputePassEncoder);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuComputePipeline")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = setPipeline ) ]
    ///The `setPipeline()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setPipeline)
    ///
    ///*This API requires the following crate features to be activated: `GpuComputePassEncoder`, `GpuComputePipeline`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_pipeline(this: &GpuComputePassEncoder, pipeline: &GpuComputePipeline);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = insertDebugMarker ) ]
    ///The `insertDebugMarker()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/insertDebugMarker)
    ///
    ///*This API requires the following crate features to be activated: `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn insert_debug_marker(this: &GpuComputePassEncoder, marker_label: &str);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = popDebugGroup ) ]
    ///The `popDebugGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/popDebugGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn pop_debug_group(this: &GpuComputePassEncoder);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = pushDebugGroup ) ]
    ///The `pushDebugGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/pushDebugGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn push_debug_group(this: &GpuComputePassEncoder, group_label: &str);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = setBindGroup ) ]
    ///The `setBindGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setBindGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_bind_group(this: &GpuComputePassEncoder, index: u32, bind_group: &GpuBindGroup);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = setBindGroup ) ]
    ///The `setBindGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setBindGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_bind_group_with_u32_sequence(
        this: &GpuComputePassEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets: &::wasm_bindgen::JsValue,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = setBindGroup ) ]
    ///The `setBindGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setBindGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_bind_group_with_u32_array_and_u32_and_dynamic_offsets_data_length(
        this: &GpuComputePassEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets_data: &mut [u32],
        dynamic_offsets_data_start: u32,
        dynamic_offsets_data_length: u32,
    );

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUComputePassEncoder" , js_name = setBindGroup ) ]
    ///The `setBindGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setBindGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuComputePassEncoder`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_bind_group_with_u32_array_and_f64_and_dynamic_offsets_data_length(
        this: &GpuComputePassEncoder,
        index: u32,
        bind_group: &GpuBindGroup,
        dynamic_offsets_data: &mut [u32],
        dynamic_offsets_data_start: f64,
        dynamic_offsets_data_length: u32,
    );

}
