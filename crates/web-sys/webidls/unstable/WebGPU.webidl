interface mixin GPUObjectBase {
    attribute DOMString? label;
};

dictionary GPUObjectDescriptorBase {
    DOMString label;
};

[Exposed=Window]
partial interface Navigator {
    [SameObject] readonly attribute GPU gpu;
};

[Exposed=DedicatedWorker]
partial interface WorkerNavigator {
    [SameObject] readonly attribute GPU gpu;
};

[Exposed=(Window, DedicatedWorker)]
interface GPU {
    Promise<GPUAdapter> requestAdapter(optional GPURequestAdapterOptions options = {});
};

dictionary GPURequestAdapterOptions {
    GPUPowerPreference powerPreference;
};

enum GPUPowerPreference {
    "low-power",
    "high-performance"
};

interface GPUAdapter {
    readonly attribute DOMString name;
    readonly attribute FrozenArray<GPUExtensionName> extensions;
    //readonly attribute GPULimits limits; Don’t expose higher limits for now.

    Promise<GPUDevice> requestDevice(optional GPUDeviceDescriptor descriptor = {});
};

dictionary GPUDeviceDescriptor : GPUObjectDescriptorBase {
    sequence<GPUExtensionName> extensions = [];
    GPULimits limits = {};
};

enum GPUExtensionName {
    "texture-compression-bc"
};

dictionary GPULimits {
    GPUSize32 maxBindGroups = 4;
    GPUSize32 maxDynamicUniformBuffersPerPipelineLayout = 8;
    GPUSize32 maxDynamicStorageBuffersPerPipelineLayout = 4;
    GPUSize32 maxSampledTexturesPerShaderStage = 16;
    GPUSize32 maxSamplersPerShaderStage = 16;
    GPUSize32 maxStorageBuffersPerShaderStage = 4;
    GPUSize32 maxStorageTexturesPerShaderStage = 4;
    GPUSize32 maxUniformBuffersPerShaderStage = 12;
};

[Exposed=(Window, DedicatedWorker), Serializable]
interface GPUDevice : EventTarget {
    [SameObject] readonly attribute GPUAdapter adapter;
    readonly attribute FrozenArray<GPUExtensionName> extensions;
    readonly attribute object limits;

    [SameObject] readonly attribute GPUQueue defaultQueue;

    GPUBuffer createBuffer(GPUBufferDescriptor descriptor);
    GPUMappedBuffer createBufferMapped(GPUBufferDescriptor descriptor);
    GPUTexture createTexture(GPUTextureDescriptor descriptor);
    GPUSampler createSampler(optional GPUSamplerDescriptor descriptor = {});

    GPUBindGroupLayout createBindGroupLayout(GPUBindGroupLayoutDescriptor descriptor);
    GPUPipelineLayout createPipelineLayout(GPUPipelineLayoutDescriptor descriptor);
    GPUBindGroup createBindGroup(GPUBindGroupDescriptor descriptor);

    GPUShaderModule createShaderModule(GPUShaderModuleDescriptor descriptor);
    GPUComputePipeline createComputePipeline(GPUComputePipelineDescriptor descriptor);
    GPURenderPipeline createRenderPipeline(GPURenderPipelineDescriptor descriptor);

    GPUCommandEncoder createCommandEncoder(optional GPUCommandEncoderDescriptor descriptor = {});
    GPURenderBundleEncoder createRenderBundleEncoder(GPURenderBundleEncoderDescriptor descriptor);

    GPUQuerySet createQuerySet(GPUQuerySetDescriptor descriptor);
};
GPUDevice includes GPUObjectBase;

[Serializable]
interface GPUBuffer {
    Promise<ArrayBuffer> mapReadAsync();
    Promise<ArrayBuffer> mapWriteAsync();
    void unmap();

    void destroy();
};
GPUBuffer includes GPUObjectBase;

dictionary GPUBufferDescriptor : GPUObjectDescriptorBase {
    required GPUSize64 size;
    required GPUBufferUsageFlags usage;
};

typedef [EnforceRange] unsigned long GPUBufferUsageFlags;
interface GPUBufferUsage {
    const GPUBufferUsageFlags MAP_READ      = 0x0001;
    const GPUBufferUsageFlags MAP_WRITE     = 0x0002;
    const GPUBufferUsageFlags COPY_SRC      = 0x0004;
    const GPUBufferUsageFlags COPY_DST      = 0x0008;
    const GPUBufferUsageFlags INDEX         = 0x0010;
    const GPUBufferUsageFlags VERTEX        = 0x0020;
    const GPUBufferUsageFlags UNIFORM       = 0x0040;
    const GPUBufferUsageFlags STORAGE       = 0x0080;
    const GPUBufferUsageFlags INDIRECT      = 0x0100;
    const GPUBufferUsageFlags QUERY_RESOLVE = 0x0200;
};

[Serializable]
interface GPUTexture {
    GPUTextureView createView(optional GPUTextureViewDescriptor descriptor = {});

    void destroy();
};
GPUTexture includes GPUObjectBase;

dictionary GPUTextureDescriptor : GPUObjectDescriptorBase {
    required GPUExtent3D size;
    GPUIntegerCoordinate mipLevelCount = 1;
    GPUSize32 sampleCount = 1;
    GPUTextureDimension dimension = "2d";
    required GPUTextureFormat format;
    required GPUTextureUsageFlags usage;
};

enum GPUTextureDimension {
    "1d",
    "2d",
    "3d"
};

typedef [EnforceRange] unsigned long GPUTextureUsageFlags;
interface GPUTextureUsage {
    const GPUTextureUsageFlags COPY_SRC          = 0x01;
    const GPUTextureUsageFlags COPY_DST          = 0x02;
    const GPUTextureUsageFlags SAMPLED           = 0x04;
    const GPUTextureUsageFlags STORAGE           = 0x08;
    const GPUTextureUsageFlags OUTPUT_ATTACHMENT = 0x10;
};

interface GPUTextureView {
};
GPUTextureView includes GPUObjectBase;

dictionary GPUTextureViewDescriptor : GPUObjectDescriptorBase {
    GPUTextureFormat format;
    GPUTextureViewDimension dimension;
    GPUTextureAspect aspect = "all";
    GPUIntegerCoordinate baseMipLevel = 0;
    GPUIntegerCoordinate mipLevelCount = 0;
    GPUIntegerCoordinate baseArrayLayer = 0;
    GPUIntegerCoordinate arrayLayerCount = 0;
};

enum GPUTextureViewDimension {
    "1d",
    "2d",
    "2d-array",
    "cube",
    "cube-array",
    "3d"
};

enum GPUTextureAspect {
    "all",
    "stencil-only",
    "depth-only"
};

enum GPUTextureFormat {
    // 8-bit formats
    "r8unorm",
    "r8snorm",
    "r8uint",
    "r8sint",

    // 16-bit formats
    "r16uint",
    "r16sint",
    "r16float",
    "rg8unorm",
    "rg8snorm",
    "rg8uint",
    "rg8sint",

    // 32-bit formats
    "r32uint",
    "r32sint",
    "r32float",
    "rg16uint",
    "rg16sint",
    "rg16float",
    "rgba8unorm",
    "rgba8unorm-srgb",
    "rgba8snorm",
    "rgba8uint",
    "rgba8sint",
    "bgra8unorm",
    "bgra8unorm-srgb",
    // Packed 32-bit formats
    "rgb10a2unorm",
    "rg11b10float",

    // 64-bit formats
    "rg32uint",
    "rg32sint",
    "rg32float",
    "rgba16uint",
    "rgba16sint",
    "rgba16float",

    // 128-bit formats
    "rgba32uint",
    "rgba32sint",
    "rgba32float",

    // Depth and stencil formats
    "depth32float",
    "depth24plus",
    "depth24plus-stencil8"
};

enum GPUTextureComponentType {
    "float",
    "sint",
    "uint"
};

interface GPUSampler {
};
GPUSampler includes GPUObjectBase;

dictionary GPUSamplerDescriptor : GPUObjectDescriptorBase {
    GPUAddressMode addressModeU = "clamp-to-edge";
    GPUAddressMode addressModeV = "clamp-to-edge";
    GPUAddressMode addressModeW = "clamp-to-edge";
    GPUFilterMode magFilter = "nearest";
    GPUFilterMode minFilter = "nearest";
    GPUFilterMode mipmapFilter = "nearest";
    float lodMinClamp = 0;
    float lodMaxClamp = 0xffffffff; // TODO: What should this be? Was Number.MAX_VALUE.
    GPUCompareFunction compare;
};

enum GPUAddressMode {
    "clamp-to-edge",
    "repeat",
    "mirror-repeat"
};

enum GPUFilterMode {
    "nearest",
    "linear"
};

enum GPUCompareFunction {
    "never",
    "less",
    "equal",
    "less-equal",
    "greater",
    "not-equal",
    "greater-equal",
    "always"
};

[Serializable]
interface GPUBindGroupLayout {
};
GPUBindGroupLayout includes GPUObjectBase;

dictionary GPUBindGroupLayoutDescriptor : GPUObjectDescriptorBase {
    required sequence<GPUBindGroupLayoutEntry> entries;
};

dictionary GPUBindGroupLayoutEntry {
    required GPUIndex32 binding;
    required GPUShaderStageFlags visibility;
    required GPUBindingType type;
    GPUTextureViewDimension viewDimension = "2d";
    GPUTextureComponentType textureComponentType = "float";
    GPUTextureFormat storageTextureFormat;
    boolean multisampled = false;
    boolean hasDynamicOffset = false;
};

typedef [EnforceRange] unsigned long GPUShaderStageFlags;
interface GPUShaderStage {
    const GPUShaderStageFlags VERTEX   = 0x1;
    const GPUShaderStageFlags FRAGMENT = 0x2;
    const GPUShaderStageFlags COMPUTE  = 0x4;
};

enum GPUBindingType {
    "uniform-buffer",
    "storage-buffer",
    "readonly-storage-buffer",
    "sampler",
    "comparison-sampler",
    "sampled-texture",
    "readonly-storage-texture",
    "writeonly-storage-texture"
    // TODO: other binding types
};

interface GPUBindGroup {
};
GPUBindGroup includes GPUObjectBase;

dictionary GPUBindGroupDescriptor : GPUObjectDescriptorBase {
    required GPUBindGroupLayout layout;
    required sequence<GPUBindGroupEntry> entries;
};

typedef (GPUSampler or GPUTextureView or GPUBufferBinding) GPUBindingResource;

dictionary GPUBindGroupEntry {
    required GPUIndex32 binding;
    required GPUBindingResource resource;
};

dictionary GPUBufferBinding {
    required GPUBuffer buffer;
    GPUSize64 offset = 0;
    GPUSize64 size;
};

interface GPUPipelineLayout {
};
GPUPipelineLayout includes GPUObjectBase;

dictionary GPUPipelineLayoutDescriptor : GPUObjectDescriptorBase {
    required sequence<GPUBindGroupLayout> bindGroupLayouts;
};

[Serializable]
interface GPUShaderModule {
};
GPUShaderModule includes GPUObjectBase;

typedef (DOMString or Uint32Array) GPUCode;

dictionary GPUShaderModuleDescriptor : GPUObjectDescriptorBase {
    required GPUCode code;
};

dictionary GPUPipelineDescriptorBase : GPUObjectDescriptorBase {
    required GPUPipelineLayout layout;
};

dictionary GPUProgrammableStageDescriptor {
    required GPUShaderModule module;
    required DOMString entryPoint;
    // TODO: other stuff like specialization constants?
};

[Serializable]
interface GPUComputePipeline {
};
GPUComputePipeline includes GPUObjectBase;

dictionary GPUComputePipelineDescriptor : GPUPipelineDescriptorBase {
    required GPUProgrammableStageDescriptor computeStage;
};

[Serializable]
interface GPURenderPipeline {
};
GPURenderPipeline includes GPUObjectBase;

dictionary GPURenderPipelineDescriptor : GPUPipelineDescriptorBase {
    required GPUProgrammableStageDescriptor vertexStage;
    GPUProgrammableStageDescriptor fragmentStage;

    required GPUPrimitiveTopology primitiveTopology;
    GPURasterizationStateDescriptor rasterizationState = {};
    required sequence<GPUColorStateDescriptor> colorStates;
    GPUDepthStencilStateDescriptor depthStencilState;
    GPUVertexStateDescriptor vertexState = {};

    GPUSize32 sampleCount = 1;
    GPUSampleMask sampleMask = 0xFFFFFFFF;
    boolean alphaToCoverageEnabled = false;
    // TODO: other properties
};

enum GPUPrimitiveTopology {
    "point-list",
    "line-list",
    "line-strip",
    "triangle-list",
    "triangle-strip"
};

dictionary GPURasterizationStateDescriptor {
    GPUFrontFace frontFace = "ccw";
    GPUCullMode cullMode = "none";

    GPUDepthBias depthBias = 0;
    float depthBiasSlopeScale = 0;
    float depthBiasClamp = 0;
};

enum GPUFrontFace {
    "ccw",
    "cw"
};

enum GPUCullMode {
    "none",
    "front",
    "back"
};

dictionary GPUColorStateDescriptor {
    required GPUTextureFormat format;

    GPUBlendDescriptor alphaBlend = {};
    GPUBlendDescriptor colorBlend = {};
    GPUColorWriteFlags writeMask = 0xF;  // GPUColorWrite.ALL
};

typedef [EnforceRange] unsigned long GPUColorWriteFlags;
interface GPUColorWrite {
    const GPUColorWriteFlags RED   = 0x1;
    const GPUColorWriteFlags GREEN = 0x2;
    const GPUColorWriteFlags BLUE  = 0x4;
    const GPUColorWriteFlags ALPHA = 0x8;
    const GPUColorWriteFlags ALL   = 0xF;
};

dictionary GPUBlendDescriptor {
    GPUBlendFactor srcFactor = "one";
    GPUBlendFactor dstFactor = "zero";
    GPUBlendOperation operation = "add";
};

enum GPUBlendFactor {
    "zero",
    "one",
    "src-color",
    "one-minus-src-color",
    "src-alpha",
    "one-minus-src-alpha",
    "dst-color",
    "one-minus-dst-color",
    "dst-alpha",
    "one-minus-dst-alpha",
    "src-alpha-saturated",
    "blend-color",
    "one-minus-blend-color"
};

enum GPUBlendOperation {
    "add",
    "subtract",
    "reverse-subtract",
    "min",
    "max"
};

enum GPUStencilOperation {
    "keep",
    "zero",
    "replace",
    "invert",
    "increment-clamp",
    "decrement-clamp",
    "increment-wrap",
    "decrement-wrap"
};

dictionary GPUDepthStencilStateDescriptor {
    required GPUTextureFormat format;

    boolean depthWriteEnabled = false;
    GPUCompareFunction depthCompare = "always";

    GPUStencilStateFaceDescriptor stencilFront = {};
    GPUStencilStateFaceDescriptor stencilBack = {};

    GPUStencilValue stencilReadMask = 0xFFFFFFFF;
    GPUStencilValue stencilWriteMask = 0xFFFFFFFF;
};

dictionary GPUStencilStateFaceDescriptor {
    GPUCompareFunction compare = "always";
    GPUStencilOperation failOp = "keep";
    GPUStencilOperation depthFailOp = "keep";
    GPUStencilOperation passOp = "keep";
};

enum GPUIndexFormat {
    "uint16",
    "uint32"
};

enum GPUVertexFormat {
    "uchar2",
    "uchar4",
    "char2",
    "char4",
    "uchar2norm",
    "uchar4norm",
    "char2norm",
    "char4norm",
    "ushort2",
    "ushort4",
    "short2",
    "short4",
    "ushort2norm",
    "ushort4norm",
    "short2norm",
    "short4norm",
    "half2",
    "half4",
    "float",
    "float2",
    "float3",
    "float4",
    "uint",
    "uint2",
    "uint3",
    "uint4",
    "int",
    "int2",
    "int3",
    "int4"
};

enum GPUInputStepMode {
    "vertex",
    "instance"
};

dictionary GPUVertexStateDescriptor {
    GPUIndexFormat indexFormat = "uint32";
    sequence<GPUVertexBufferLayoutDescriptor?> vertexBuffers = [];
};

dictionary GPUVertexBufferLayoutDescriptor {
    required GPUSize64 arrayStride;
    GPUInputStepMode stepMode = "vertex";
    required sequence<GPUVertexAttributeDescriptor> attributes;
};

dictionary GPUVertexAttributeDescriptor {
    required GPUVertexFormat format;
    required GPUSize64 offset;

    required GPUIndex32 shaderLocation;
};

interface GPUCommandBuffer {
};
GPUCommandBuffer includes GPUObjectBase;

dictionary GPUCommandBufferDescriptor : GPUObjectDescriptorBase {
};

interface GPUCommandEncoder {
    GPURenderPassEncoder beginRenderPass(GPURenderPassDescriptor descriptor);
    GPUComputePassEncoder beginComputePass(optional GPUComputePassDescriptor descriptor = {});

    void copyBufferToBuffer(
        GPUBuffer source,
        GPUSize64 sourceOffset,
        GPUBuffer destination,
        GPUSize64 destinationOffset,
        GPUSize64 size);

    void copyBufferToTexture(
        GPUBufferCopyView source,
        GPUTextureCopyView destination,
        GPUExtent3D copySize);

    void copyTextureToBuffer(
        GPUTextureCopyView source,
        GPUBufferCopyView destination,
        GPUExtent3D copySize);

    void copyTextureToTexture(
        GPUTextureCopyView source,
        GPUTextureCopyView destination,
        GPUExtent3D copySize);

    void pushDebugGroup(DOMString groupLabel);
    void popDebugGroup();
    void insertDebugMarker(DOMString markerLabel);

    void resolveQuerySet(
        GPUQuerySet querySet,
        GPUSize32 firstQuery,
        GPUSize32 queryCount,
        GPUBuffer destination,
        GPUSize64 destinationOffset);

    GPUCommandBuffer finish(optional GPUCommandBufferDescriptor descriptor = {});
};
GPUCommandEncoder includes GPUObjectBase;

dictionary GPUCommandEncoderDescriptor : GPUObjectDescriptorBase {
    // TODO: reusability flag?
};

dictionary GPUBufferCopyView {
    required GPUBuffer buffer;
    GPUSize64 offset = 0;
    required GPUSize32 bytesPerRow;
    GPUSize32 rowsPerImage = 0;
};

dictionary GPUTextureCopyView {
    required GPUTexture texture;
    GPUIntegerCoordinate mipLevel = 0;
    GPUIntegerCoordinate arrayLayer = 0;
    GPUOrigin3D origin = {};
};

dictionary GPUImageBitmapCopyView {
    required ImageBitmap imageBitmap;
    GPUOrigin2D origin = {};
};

interface mixin GPUProgrammablePassEncoder {
    void setBindGroup(GPUIndex32 index, GPUBindGroup bindGroup,
                      optional sequence<GPUBufferDynamicOffset> dynamicOffsets = []);

    void setBindGroup(GPUIndex32 index, GPUBindGroup bindGroup,
                      Uint32Array dynamicOffsetsData,
                      GPUSize64 dynamicOffsetsDataStart,
                      GPUSize32 dynamicOffsetsDataLength);

    void pushDebugGroup(DOMString groupLabel);
    void popDebugGroup();
    void insertDebugMarker(DOMString markerLabel);
};

interface GPUComputePassEncoder {
    void setPipeline(GPUComputePipeline pipeline);
    void dispatch(GPUSize32 x, optional GPUSize32 y = 1, optional GPUSize32 z = 1);
    void dispatchIndirect(GPUBuffer indirectBuffer, GPUSize64 indirectOffset);

    void endPass();
};
GPUComputePassEncoder includes GPUObjectBase;
GPUComputePassEncoder includes GPUProgrammablePassEncoder;

dictionary GPUComputePassDescriptor : GPUObjectDescriptorBase {
};

interface mixin GPURenderEncoderBase {
    void setPipeline(GPURenderPipeline pipeline);

    void setIndexBuffer(GPUBuffer buffer, optional GPUSize64 offset = 0, optional GPUSize64 size = 0);
    void setVertexBuffer(GPUIndex32 slot, GPUBuffer buffer, optional GPUSize64 offset = 0, optional GPUSize64 size = 0);

    void draw(GPUSize32 vertexCount, optional GPUSize32 instanceCount = 1,
              optional GPUSize32 firstVertex = 0, optional GPUSize32 firstInstance = 0);
    void drawIndexed(GPUSize32 indexCount, optional GPUSize32 instanceCount = 1,
                     optional GPUSize32 firstIndex = 0,
                     optional GPUSignedOffset32 baseVertex = 0,
                     optional GPUSize32 firstInstance = 0);

    void drawIndirect(GPUBuffer indirectBuffer, GPUSize64 indirectOffset);
    void drawIndexedIndirect(GPUBuffer indirectBuffer, GPUSize64 indirectOffset);
};

interface GPURenderPassEncoder {
    void setViewport(float x, float y,
                     float width, float height,
                     float minDepth, float maxDepth);

    void setScissorRect(GPUIntegerCoordinate x, GPUIntegerCoordinate y,
                        GPUIntegerCoordinate width, GPUIntegerCoordinate height);

    void setBlendColor(GPUColor color);
    void setStencilReference(GPUStencilValue reference);

    void beginOcclusionQuery(GPUSize32 queryIndex);
    void endOcclusionQuery(GPUSize32 queryIndex);

    void executeBundles(sequence<GPURenderBundle> bundles);
    void endPass();
};
GPURenderPassEncoder includes GPUObjectBase;
GPURenderPassEncoder includes GPUProgrammablePassEncoder;
GPURenderPassEncoder includes GPURenderEncoderBase;

dictionary GPURenderPassDescriptor : GPUObjectDescriptorBase {
    required sequence<GPURenderPassColorAttachmentDescriptor> colorAttachments;
    GPURenderPassDepthStencilAttachmentDescriptor depthStencilAttachment;
    GPUQuerySet occlusionQuerySet;
};

dictionary GPURenderPassColorAttachmentDescriptor {
    required GPUTextureView attachment;
    GPUTextureView resolveTarget;

    required (GPULoadOp or GPUColor) loadValue;
    GPUStoreOp storeOp = "store";
};

dictionary GPURenderPassDepthStencilAttachmentDescriptor {
    required GPUTextureView attachment;

    required (GPULoadOp or float) depthLoadValue;
    required GPUStoreOp depthStoreOp;

    required (GPULoadOp or GPUStencilValue) stencilLoadValue;
    required GPUStoreOp stencilStoreOp;
};

enum GPULoadOp {
    "load"
};

enum GPUStoreOp {
    "store",
    "clear"
};

interface GPURenderBundle {
};
GPURenderBundle includes GPUObjectBase;

dictionary GPURenderBundleDescriptor : GPUObjectDescriptorBase {
};

interface GPURenderBundleEncoder {
    GPURenderBundle finish(optional GPURenderBundleDescriptor descriptor = {});
};
GPURenderBundleEncoder includes GPUObjectBase;
GPURenderBundleEncoder includes GPUProgrammablePassEncoder;
GPURenderBundleEncoder includes GPURenderEncoderBase;

dictionary GPURenderBundleEncoderDescriptor : GPUObjectDescriptorBase {
    required sequence<GPUTextureFormat> colorFormats;
    GPUTextureFormat depthStencilFormat;
    GPUSize32 sampleCount = 1;
};

interface GPUQueue {
    void submit(sequence<GPUCommandBuffer> commandBuffers);

    GPUFence createFence(optional GPUFenceDescriptor descriptor = {});
    void signal(GPUFence fence, GPUFenceValue signalValue);

    void copyImageBitmapToTexture(
        GPUImageBitmapCopyView source,
        GPUTextureCopyView destination,
        GPUExtent3D copySize);
};
GPUQueue includes GPUObjectBase;

interface GPUFence {
    GPUFenceValue getCompletedValue();
    Promise<void> onCompletion(GPUFenceValue completionValue);
};
GPUFence includes GPUObjectBase;

dictionary GPUFenceDescriptor : GPUObjectDescriptorBase {
    GPUFenceValue initialValue = 0;
};

interface GPUQuerySet {
    void destroy();
};
GPUQuerySet includes GPUObjectBase;

dictionary GPUQuerySetDescriptor : GPUObjectDescriptorBase {
    required GPUQueryType type;
    required GPUSize32 count;
};

enum GPUQueryType {
    "occlusion"
};

interface GPUCanvasContext {
    GPUSwapChain configureSwapChain(GPUSwapChainDescriptor descriptor);

    Promise<GPUTextureFormat> getSwapChainPreferredFormat(GPUDevice device);
};

dictionary GPUSwapChainDescriptor : GPUObjectDescriptorBase {
    required GPUDevice device;
    required GPUTextureFormat format;
    GPUTextureUsageFlags usage = 0x10;  // GPUTextureUsage.OUTPUT_ATTACHMENT
};

interface GPUSwapChain {
    GPUTexture getCurrentTexture();
};
GPUSwapChain includes GPUObjectBase;

interface GPUDeviceLostInfo {
    readonly attribute DOMString message;
};

partial interface GPUDevice {
    readonly attribute Promise<GPUDeviceLostInfo> lost;
};

enum GPUErrorFilter {
    "none",
    "out-of-memory",
    "validation"
};

interface GPUOutOfMemoryError {
    constructor();
};

interface GPUValidationError {
    constructor(DOMString message);
    readonly attribute DOMString message;
};

typedef (GPUOutOfMemoryError or GPUValidationError) GPUError;

partial interface GPUDevice {
    void pushErrorScope(GPUErrorFilter filter);
    Promise<GPUError?> popErrorScope();
};

[
    Exposed=(Window, DedicatedWorker)
]
interface GPUUncapturedErrorEvent : Event {
    constructor(
        DOMString type,
        GPUUncapturedErrorEventInit gpuUncapturedErrorEventInitDict
    );
    [SameObject] readonly attribute GPUError error;
};

dictionary GPUUncapturedErrorEventInit : EventInit {
    required GPUError error;
};

partial interface GPUDevice {
    [Exposed=(Window, DedicatedWorker)]
    attribute EventHandler onuncapturederror;
};

typedef [EnforceRange] unsigned long GPUBufferDynamicOffset;
typedef [EnforceRange] unsigned long long GPUFenceValue;
typedef [EnforceRange] unsigned long GPUStencilValue;
typedef [EnforceRange] unsigned long GPUSampleMask;
typedef [EnforceRange] long GPUDepthBias;

typedef [EnforceRange] unsigned long long GPUSize64;
typedef [EnforceRange] unsigned long GPUIntegerCoordinate;
typedef [EnforceRange] unsigned long GPUIndex32;
typedef [EnforceRange] unsigned long GPUSize32;
typedef [EnforceRange] long GPUSignedOffset32;

dictionary GPUColorDict {
    required double r;
    required double g;
    required double b;
    required double a;
};
typedef (sequence<double> or GPUColorDict) GPUColor;

dictionary GPUOrigin2DDict {
    GPUIntegerCoordinate x = 0;
    GPUIntegerCoordinate y = 0;
};
typedef (sequence<GPUIntegerCoordinate> or GPUOrigin2DDict) GPUOrigin2D;

dictionary GPUOrigin3DDict {
    GPUIntegerCoordinate x = 0;
    GPUIntegerCoordinate y = 0;
    GPUIntegerCoordinate z = 0;
};
typedef (sequence<GPUIntegerCoordinate> or GPUOrigin3DDict) GPUOrigin3D;

dictionary GPUExtent3DDict {
    required GPUIntegerCoordinate width;
    required GPUIntegerCoordinate height;
    required GPUIntegerCoordinate depth;
};
typedef (sequence<GPUIntegerCoordinate> or GPUExtent3DDict) GPUExtent3D;

typedef sequence<(GPUBuffer or ArrayBuffer)> GPUMappedBuffer;
