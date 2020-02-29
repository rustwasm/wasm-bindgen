use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = GPUDevice , typescript_name = GPUDevice ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuDevice` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuDevice;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAdapter")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUDevice" , js_name = adapter ) ]
    ///Getter for the `adapter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/adapter)
    ///
    ///*This API requires the following crate features to be activated: `GpuAdapter`, `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn adapter(this: &GpuDevice) -> GpuAdapter;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUDevice" , js_name = limits ) ]
    ///Getter for the `limits` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/limits)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn limits(this: &GpuDevice) -> ::js_sys::Object;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQueue")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUDevice" , js_name = defaultQueue ) ]
    ///Getter for the `defaultQueue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/defaultQueue)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`, `GpuQueue`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn default_queue(this: &GpuDevice) -> GpuQueue;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUDevice" , js_name = lost ) ]
    ///Getter for the `lost` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/lost)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn lost(this: &GpuDevice) -> ::js_sys::Promise;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUDevice" , js_name = onuncapturederror ) ]
    ///Getter for the `onuncapturederror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/onuncapturederror)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn onuncapturederror(this: &GpuDevice) -> Option<::js_sys::Function>;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPUDevice" , js_name = onuncapturederror ) ]
    ///Setter for the `onuncapturederror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/onuncapturederror)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_onuncapturederror(this: &GpuDevice, value: Option<&::js_sys::Function>);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUDevice" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn label(this: &GpuDevice) -> Option<String>;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPUDevice" , js_name = label ) ]
    ///Setter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_label(this: &GpuDevice, value: Option<&str>);

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBindGroup", feature = "GpuBindGroupDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUDevice" , js_name = createBindGroup ) ]
    ///The `createBindGroup()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBindGroup)
    ///
    ///*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuBindGroupDescriptor`, `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn create_bind_group(this: &GpuDevice, descriptor: &GpuBindGroupDescriptor)
        -> GpuBindGroup;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuBindGroupLayout",
        feature = "GpuBindGroupLayoutDescriptor",
    ))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUDevice" , js_name = createBindGroupLayout ) ]
    ///The `createBindGroupLayout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBindGroupLayout)
    ///
    ///*This API requires the following crate features to be activated: `GpuBindGroupLayout`, `GpuBindGroupLayoutDescriptor`, `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn create_bind_group_layout(
        this: &GpuDevice,
        descriptor: &GpuBindGroupLayoutDescriptor,
    ) -> GpuBindGroupLayout;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBuffer", feature = "GpuBufferDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUDevice" , js_name = createBuffer ) ]
    ///The `createBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`, `GpuBufferDescriptor`, `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn create_buffer(this: &GpuDevice, descriptor: &GpuBufferDescriptor) -> GpuBuffer;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBufferDescriptor")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUDevice" , js_name = createBufferMapped ) ]
    ///The `createBufferMapped()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBufferMapped)
    ///
    ///*This API requires the following crate features to be activated: `GpuBufferDescriptor`, `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn create_buffer_mapped(
        this: &GpuDevice,
        descriptor: &GpuBufferDescriptor,
    ) -> ::js_sys::Array;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCommandEncoder")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUDevice" , js_name = createCommandEncoder ) ]
    ///The `createCommandEncoder()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createCommandEncoder)
    ///
    ///*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn create_command_encoder(this: &GpuDevice) -> GpuCommandEncoder;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuCommandEncoder", feature = "GpuCommandEncoderDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUDevice" , js_name = createCommandEncoder ) ]
    ///The `createCommandEncoder()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createCommandEncoder)
    ///
    ///*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuCommandEncoderDescriptor`, `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn create_command_encoder_with_descriptor(
        this: &GpuDevice,
        descriptor: &GpuCommandEncoderDescriptor,
    ) -> GpuCommandEncoder;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuComputePipeline",
        feature = "GpuComputePipelineDescriptor",
    ))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUDevice" , js_name = createComputePipeline ) ]
    ///The `createComputePipeline()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createComputePipeline)
    ///
    ///*This API requires the following crate features to be activated: `GpuComputePipeline`, `GpuComputePipelineDescriptor`, `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn create_compute_pipeline(
        this: &GpuDevice,
        descriptor: &GpuComputePipelineDescriptor,
    ) -> GpuComputePipeline;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuPipelineLayout", feature = "GpuPipelineLayoutDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUDevice" , js_name = createPipelineLayout ) ]
    ///The `createPipelineLayout()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createPipelineLayout)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`, `GpuPipelineLayout`, `GpuPipelineLayoutDescriptor`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn create_pipeline_layout(
        this: &GpuDevice,
        descriptor: &GpuPipelineLayoutDescriptor,
    ) -> GpuPipelineLayout;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuRenderBundleEncoder",
        feature = "GpuRenderBundleEncoderDescriptor",
    ))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUDevice" , js_name = createRenderBundleEncoder ) ]
    ///The `createRenderBundleEncoder()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createRenderBundleEncoder)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`, `GpuRenderBundleEncoder`, `GpuRenderBundleEncoderDescriptor`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn create_render_bundle_encoder(
        this: &GpuDevice,
        descriptor: &GpuRenderBundleEncoderDescriptor,
    ) -> GpuRenderBundleEncoder;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuRenderPipeline", feature = "GpuRenderPipelineDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUDevice" , js_name = createRenderPipeline ) ]
    ///The `createRenderPipeline()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createRenderPipeline)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`, `GpuRenderPipeline`, `GpuRenderPipelineDescriptor`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn create_render_pipeline(
        this: &GpuDevice,
        descriptor: &GpuRenderPipelineDescriptor,
    ) -> GpuRenderPipeline;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSampler")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUDevice" , js_name = createSampler ) ]
    ///The `createSampler()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createSampler)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`, `GpuSampler`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn create_sampler(this: &GpuDevice) -> GpuSampler;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuSampler", feature = "GpuSamplerDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUDevice" , js_name = createSampler ) ]
    ///The `createSampler()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createSampler)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`, `GpuSampler`, `GpuSamplerDescriptor`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn create_sampler_with_descriptor(
        this: &GpuDevice,
        descriptor: &GpuSamplerDescriptor,
    ) -> GpuSampler;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuShaderModule", feature = "GpuShaderModuleDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUDevice" , js_name = createShaderModule ) ]
    ///The `createShaderModule()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createShaderModule)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`, `GpuShaderModule`, `GpuShaderModuleDescriptor`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn create_shader_module(
        this: &GpuDevice,
        descriptor: &GpuShaderModuleDescriptor,
    ) -> GpuShaderModule;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuTexture", feature = "GpuTextureDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_class = "GPUDevice" , js_name = createTexture ) ]
    ///The `createTexture()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createTexture)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`, `GpuTexture`, `GpuTextureDescriptor`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn create_texture(this: &GpuDevice, descriptor: &GpuTextureDescriptor) -> GpuTexture;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUDevice" , js_name = popErrorScope ) ]
    ///The `popErrorScope()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/popErrorScope)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn pop_error_scope(this: &GpuDevice) -> ::js_sys::Promise;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuErrorFilter")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUDevice" , js_name = pushErrorScope ) ]
    ///The `pushErrorScope()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/pushErrorScope)
    ///
    ///*This API requires the following crate features to be activated: `GpuDevice`, `GpuErrorFilter`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn push_error_scope(this: &GpuDevice, filter: GpuErrorFilter);

}
