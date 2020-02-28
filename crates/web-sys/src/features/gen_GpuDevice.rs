use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = GPUDevice , typescript_name = GPUDevice ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuDevice` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice)\n\n*This API requires the following crate features to be activated: `GpuDevice`*"]
    pub type GpuDevice;
    # [ wasm_bindgen ( structural , method , getter , js_name = adapter ) ]
    #[cfg(feature = "GpuAdapter")]
    #[doc = "Getter for the `adapter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/adapter)\n\n*This API requires the following crate features to be activated: `GpuAdapter`, `GpuDevice`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn adapter(this: &GpuDevice) -> GpuAdapter;
    # [ wasm_bindgen ( structural , method , getter , js_name = limits ) ]
    #[doc = "Getter for the `limits` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/limits)\n\n*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn limits(this: &GpuDevice) -> ::js_sys::Object;
    # [ wasm_bindgen ( structural , method , getter , js_name = defaultQueue ) ]
    #[cfg(feature = "GpuQueue")]
    #[doc = "Getter for the `defaultQueue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/defaultQueue)\n\n*This API requires the following crate features to be activated: `GpuDevice`, `GpuQueue`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn default_queue(this: &GpuDevice) -> GpuQueue;
    # [ wasm_bindgen ( structural , method , getter , js_name = lost ) ]
    #[doc = "Getter for the `lost` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/lost)\n\n*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn lost(this: &GpuDevice) -> ::js_sys::Promise;
    # [ wasm_bindgen ( structural , method , getter , js_name = onuncapturederror ) ]
    #[doc = "Getter for the `onuncapturederror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/onuncapturederror)\n\n*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onuncapturederror(this: &GpuDevice) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onuncapturederror ) ]
    #[doc = "Setter for the `onuncapturederror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/onuncapturederror)\n\n*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onuncapturederror(this: &GpuDevice, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/label)\n\n*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuDevice) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/label)\n\n*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuDevice, value: Option<&str>);
    #[cfg(all(feature = "GpuBindGroup", feature = "GpuBindGroupDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_name = createBindGroup ) ]
    #[doc = "The `createBindGroup()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBindGroup)\n\n*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuBindGroupDescriptor`, `GpuDevice`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_bind_group(this: &GpuDevice, descriptor: &GpuBindGroupDescriptor)
        -> GpuBindGroup;
    #[cfg(all(
        feature = "GpuBindGroupLayout",
        feature = "GpuBindGroupLayoutDescriptor",
    ))]
    # [ wasm_bindgen ( method , structural , js_name = createBindGroupLayout ) ]
    #[doc = "The `createBindGroupLayout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBindGroupLayout)\n\n*This API requires the following crate features to be activated: `GpuBindGroupLayout`, `GpuBindGroupLayoutDescriptor`, `GpuDevice`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_bind_group_layout(
        this: &GpuDevice,
        descriptor: &GpuBindGroupLayoutDescriptor,
    ) -> GpuBindGroupLayout;
    #[cfg(all(feature = "GpuBuffer", feature = "GpuBufferDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_name = createBuffer ) ]
    #[doc = "The `createBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`, `GpuBufferDescriptor`, `GpuDevice`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_buffer(this: &GpuDevice, descriptor: &GpuBufferDescriptor) -> GpuBuffer;
    #[cfg(feature = "GpuBufferDescriptor")]
    # [ wasm_bindgen ( method , structural , js_name = createBufferMapped ) ]
    #[doc = "The `createBufferMapped()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBufferMapped)\n\n*This API requires the following crate features to be activated: `GpuBufferDescriptor`, `GpuDevice`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_buffer_mapped(
        this: &GpuDevice,
        descriptor: &GpuBufferDescriptor,
    ) -> ::js_sys::Array;
    #[cfg(feature = "GpuCommandEncoder")]
    # [ wasm_bindgen ( method , structural , js_name = createCommandEncoder ) ]
    #[doc = "The `createCommandEncoder()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createCommandEncoder)\n\n*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuDevice`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_command_encoder(this: &GpuDevice) -> GpuCommandEncoder;
    #[cfg(all(feature = "GpuCommandEncoder", feature = "GpuCommandEncoderDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_name = createCommandEncoder ) ]
    #[doc = "The `createCommandEncoder()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createCommandEncoder)\n\n*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuCommandEncoderDescriptor`, `GpuDevice`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_command_encoder_with_descriptor(
        this: &GpuDevice,
        descriptor: &GpuCommandEncoderDescriptor,
    ) -> GpuCommandEncoder;
    #[cfg(all(
        feature = "GpuComputePipeline",
        feature = "GpuComputePipelineDescriptor",
    ))]
    # [ wasm_bindgen ( method , structural , js_name = createComputePipeline ) ]
    #[doc = "The `createComputePipeline()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createComputePipeline)\n\n*This API requires the following crate features to be activated: `GpuComputePipeline`, `GpuComputePipelineDescriptor`, `GpuDevice`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_compute_pipeline(
        this: &GpuDevice,
        descriptor: &GpuComputePipelineDescriptor,
    ) -> GpuComputePipeline;
    #[cfg(all(feature = "GpuPipelineLayout", feature = "GpuPipelineLayoutDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_name = createPipelineLayout ) ]
    #[doc = "The `createPipelineLayout()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createPipelineLayout)\n\n*This API requires the following crate features to be activated: `GpuDevice`, `GpuPipelineLayout`, `GpuPipelineLayoutDescriptor`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_pipeline_layout(
        this: &GpuDevice,
        descriptor: &GpuPipelineLayoutDescriptor,
    ) -> GpuPipelineLayout;
    #[cfg(all(
        feature = "GpuRenderBundleEncoder",
        feature = "GpuRenderBundleEncoderDescriptor",
    ))]
    # [ wasm_bindgen ( method , structural , js_name = createRenderBundleEncoder ) ]
    #[doc = "The `createRenderBundleEncoder()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createRenderBundleEncoder)\n\n*This API requires the following crate features to be activated: `GpuDevice`, `GpuRenderBundleEncoder`, `GpuRenderBundleEncoderDescriptor`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_render_bundle_encoder(
        this: &GpuDevice,
        descriptor: &GpuRenderBundleEncoderDescriptor,
    ) -> GpuRenderBundleEncoder;
    #[cfg(all(feature = "GpuRenderPipeline", feature = "GpuRenderPipelineDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_name = createRenderPipeline ) ]
    #[doc = "The `createRenderPipeline()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createRenderPipeline)\n\n*This API requires the following crate features to be activated: `GpuDevice`, `GpuRenderPipeline`, `GpuRenderPipelineDescriptor`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_render_pipeline(
        this: &GpuDevice,
        descriptor: &GpuRenderPipelineDescriptor,
    ) -> GpuRenderPipeline;
    #[cfg(feature = "GpuSampler")]
    # [ wasm_bindgen ( method , structural , js_name = createSampler ) ]
    #[doc = "The `createSampler()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createSampler)\n\n*This API requires the following crate features to be activated: `GpuDevice`, `GpuSampler`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_sampler(this: &GpuDevice) -> GpuSampler;
    #[cfg(all(feature = "GpuSampler", feature = "GpuSamplerDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_name = createSampler ) ]
    #[doc = "The `createSampler()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createSampler)\n\n*This API requires the following crate features to be activated: `GpuDevice`, `GpuSampler`, `GpuSamplerDescriptor`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_sampler_with_descriptor(
        this: &GpuDevice,
        descriptor: &GpuSamplerDescriptor,
    ) -> GpuSampler;
    #[cfg(all(feature = "GpuShaderModule", feature = "GpuShaderModuleDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_name = createShaderModule ) ]
    #[doc = "The `createShaderModule()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createShaderModule)\n\n*This API requires the following crate features to be activated: `GpuDevice`, `GpuShaderModule`, `GpuShaderModuleDescriptor`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_shader_module(
        this: &GpuDevice,
        descriptor: &GpuShaderModuleDescriptor,
    ) -> GpuShaderModule;
    #[cfg(all(feature = "GpuTexture", feature = "GpuTextureDescriptor",))]
    # [ wasm_bindgen ( method , structural , js_name = createTexture ) ]
    #[doc = "The `createTexture()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createTexture)\n\n*This API requires the following crate features to be activated: `GpuDevice`, `GpuTexture`, `GpuTextureDescriptor`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_texture(this: &GpuDevice, descriptor: &GpuTextureDescriptor) -> GpuTexture;
    # [ wasm_bindgen ( method , structural , js_name = popErrorScope ) ]
    #[doc = "The `popErrorScope()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/popErrorScope)\n\n*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn pop_error_scope(this: &GpuDevice) -> ::js_sys::Promise;
    #[cfg(feature = "GpuErrorFilter")]
    # [ wasm_bindgen ( method , structural , js_name = pushErrorScope ) ]
    #[doc = "The `pushErrorScope()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/pushErrorScope)\n\n*This API requires the following crate features to be activated: `GpuDevice`, `GpuErrorFilter`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn push_error_scope(this: &GpuDevice, filter: GpuErrorFilter);
}
