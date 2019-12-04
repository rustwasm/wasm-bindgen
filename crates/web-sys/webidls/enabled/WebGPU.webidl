/* -*- Mode: IDL; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 4 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * https://gpuweb.github.io/gpuweb/
 */

typedef long i32;
typedef unsigned long u32;
typedef unsigned long long u64;

dictionary GPUColorDict {
    required double r;
    required double g;
    required double b;
    required double a;
};

dictionary GPUOrigin2DDict {
    u32 x = 0;
    u32 y = 0;
};

dictionary GPUOrigin3DDict {
    u32 x = 0;
    u32 y = 0;
    u32 z = 0;
};

dictionary GPUExtent3DDict {
    required u32 width;
    required u32 height;
    required u32 depth;
};

typedef (sequence<double> or GPUColorDict) GPUColor;
typedef (sequence<u32> or GPUOrigin2DDict) GPUOrigin2D;
typedef (sequence<u32> or GPUOrigin3DDict) GPUOrigin3D;
typedef (sequence<u32> or GPUExtent3DDict) GPUExtent3D;

interface mixin GPUObjectBase {
    attribute DOMString? label;
};

dictionary GPUObjectDescriptorBase {
    DOMString? label;
};

// ****************************************************************************
// INITIALIZATION
// ****************************************************************************

[
    Pref="dom.webgpu.enable",
    Exposed=Window
]
interface GPU {
    // May reject with DOMException
    [NewObject]
    Promise<GPUAdapter> requestAdapter(optional GPURequestAdapterOptions options = {});
};

// Add a "webgpu" member to Navigator/Worker that contains the global instance of a "WebGPU"
interface mixin GPUProvider {
    [SameObject, Replaceable, Pref="dom.webgpu.enable", Exposed=Window] readonly attribute GPU gpu;
};

enum GPUPowerPreference {
    "low-power",
    "high-performance"
};

dictionary GPURequestAdapterOptions {
    GPUPowerPreference powerPreference;
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUAdapter {
    readonly attribute DOMString name;
    //GPUExtensions getExtensions();
    //readonly attribute GPULimits limits; Don't expose higher limits for now.

    // May reject with DOMException
    [NewObject]
    Promise<GPUDevice> requestDevice(optional GPUDeviceDescriptor descriptor = {});
};
GPUAdapter includes GPUObjectBase;

dictionary GPUExtensions {
    boolean anisotropicFiltering = false;
};

dictionary GPULimits {
    u32 maxBindGroups = 4;
};

// Device
[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUDevice {
    //GPUExtensions getExtensions();
    //GPULimits getLimits();
    //readonly attribute GPUAdapter adapter;

    //GPUBuffer createBuffer(GPUBufferDescriptor descriptor);
    //GPUMappedBuffer createBufferMapped(GPUBufferDescriptor descriptor);
    //Promise<GPUMappedBuffer> createBufferMappedAsync(GPUBufferDescriptor descriptor);
    //GPUTexture createTexture(GPUTextureDescriptor descriptor);
    //GPUSampler createSampler(optional GPUSamplerDescriptor descriptor = {});

    //GPUBindGroupLayout createBindGroupLayout(GPUBindGroupLayoutDescriptor descriptor);
    //GPUPipelineLayout createPipelineLayout(GPUPipelineLayoutDescriptor descriptor);
    //GPUBindGroup createBindGroup(GPUBindGroupDescriptor descriptor);

    //GPUShaderModule createShaderModule(GPUShaderModuleDescriptor descriptor);
    //GPUComputePipeline createComputePipeline(GPUComputePipelineDescriptor descriptor);
    //GPURenderPipeline createRenderPipeline(GPURenderPipelineDescriptor descriptor);

    //GPUCommandEncoder createCommandEncoder(optional GPUCommandEncoderDescriptor descriptor = {});
    //GPURenderBundleEncoder createRenderBundleEncoder(GPURenderBundleEncoderDescriptor descriptor);

    //GPUQueue getQueue();
};
GPUDevice includes GPUObjectBase;

dictionary GPUDeviceDescriptor {
    GPUExtensions extensions;
    GPULimits limits;

    // TODO are other things configurable like queues?
};


// ****************************************************************************
// ERROR HANDLING
// ****************************************************************************

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUDeviceLostInfo {
    readonly attribute DOMString message;
};

enum GPUErrorFilter {
    "none",
    "out-of-memory",
    "validation"
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUOutOfMemoryError {
    //constructor();
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUValidationError {
    //constructor(DOMString message);
    //readonly attribute DOMString message;
};

typedef (GPUOutOfMemoryError or GPUValidationError) GPUError;

partial interface GPUDevice {
    //readonly attribute Promise<GPUDeviceLostInfo> lost;
    //void pushErrorScope(GPUErrorFilter filter);
    //Promise<GPUError?> popErrorScope();
    //[Exposed=Window]
    //attribute EventHandler onuncapturederror;
};

// ****************************************************************************
// SHADER RESOURCES (buffer, textures, texture views, samples)
// ****************************************************************************

// Buffer
typedef u32 GPUBufferUsageFlags;
[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUBufferUsage {
    const u32 NONE      = 0x0000;
    const u32 MAP_READ  = 0x0001;
    const u32 MAP_WRITE = 0x0002;
    const u32 COPY_SRC  = 0x0004;
    const u32 COPY_DST  = 0x0008;
    const u32 INDEX     = 0x0010;
    const u32 VERTEX    = 0x0020;
    const u32 UNIFORM   = 0x0040;
    const u32 STORAGE   = 0x0080;
    const u32 INDIRECT  = 0x0100;
};

dictionary GPUBufferDescriptor {
    required u64 size;
    required GPUBufferUsageFlags usage;
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUBuffer {
    //Promise<ArrayBuffer> mapReadAsync();
    //Promise<ArrayBuffer> mapWriteAsync();
    //void unmap();

    //void destroy();
};
GPUBuffer includes GPUObjectBase;

typedef sequence<any> GPUMappedBuffer;

// Texture
enum GPUTextureDimension {
    "1d",
    "2d",
    "3d",
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

typedef u32 GPUTextureUsageFlags;
[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUTextureUsage {
    const u32 NONE              = 0x00;
    const u32 COPY_SRC          = 0x01;
    const u32 COPY_DST          = 0x02;
    const u32 SAMPLED           = 0x04;
    const u32 STORAGE           = 0x08;
    const u32 OUTPUT_ATTACHMENT = 0x10;
};

dictionary GPUTextureDescriptor {
    required GPUExtent3D size;
    u32 arrayLayerCount = 1;
    u32 mipLevelCount = 1;
    u32 sampleCount = 1;
    GPUTextureDimension dimension = "2d";
    required GPUTextureFormat format;
    required GPUTextureUsageFlags usage;
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUTexture {
    //GPUTextureView createView(GPUTextureViewDescriptor descriptor);

    //void destroy();
};
GPUTexture includes GPUObjectBase;

// Texture view
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

dictionary GPUTextureViewDescriptor : GPUObjectDescriptorBase {
    required GPUTextureFormat format;
    required GPUTextureViewDimension dimension;
    required GPUTextureAspect aspect;
    u32 baseMipLevel = 0;
    u32 mipLevelCount = 1;
    u32 baseArrayLayer = 0;
    u32 arrayLayerCount = 1;
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUTextureView {
};
GPUTextureView includes GPUObjectBase;

// Sampler
enum GPUAddressMode {
    "clamp-to-edge",
    "repeat",
    "mirror-repeat"
};

enum GPUFilterMode {
    "nearest",
    "linear",
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

dictionary GPUSamplerDescriptor : GPUObjectDescriptorBase {
    GPUAddressMode addressModeU = "clamp-to-edge";
    GPUAddressMode addressModeV = "clamp-to-edge";
    GPUAddressMode addressModeW = "clamp-to-edge";
    GPUFilterMode magFilter = "nearest";
    GPUFilterMode minFilter = "nearest";
    GPUFilterMode mipmapFilter = "nearest";
    float lodMinClamp = 0;
    float lodMaxClamp = 1000.0; //TODO?
    GPUCompareFunction compare = "never";
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUSampler {
};
GPUSampler includes GPUObjectBase;

enum GPUTextureComponentType {
    "float",
    "sint",
    "uint"
};

// ****************************************************************************
// BINDING MODEL (bindgroup layout, bindgroup)
// ****************************************************************************

// PipelineLayout
dictionary GPUPipelineLayoutDescriptor : GPUObjectDescriptorBase {
    required sequence<GPUBindGroupLayout> bindGroupLayouts;
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUPipelineLayout {
};
GPUPipelineLayout includes GPUObjectBase;

// BindGroupLayout
typedef u32 GPUShaderStageFlags;
[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUShaderStage {
    const u32 NONE = 0;
    const u32 VERTEX = 1;
    const u32 FRAGMENT = 2;
    const u32 COMPUTE = 4;
};

enum GPUBindingType {
    "uniform-buffer",
    "storage-buffer",
    "readonly-storage-buffer",
    "sampler",
    "sampled-texture",
    "storage-texture",
};

dictionary GPUBindGroupLayoutBinding {
    required u32 binding;
    required GPUShaderStageFlags visibility;
    required GPUBindingType type;
    GPUTextureViewDimension textureDimension = "2d";
    GPUTextureComponentType textureComponentType = "float";
    boolean multisampled = false;
    boolean dynamic = false;
};

dictionary GPUBindGroupLayoutDescriptor : GPUObjectDescriptorBase {
    required sequence<GPUBindGroupLayoutBinding> bindings;
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUBindGroupLayout {
};
GPUBindGroupLayout includes GPUObjectBase;

// BindGroup
dictionary GPUBufferBinding {
    required GPUBuffer buffer;
    u64 offset = 0;
    u64 size;
};

typedef (GPUSampler or GPUTextureView or GPUBufferBinding) GPUBindingResource;

dictionary GPUBindGroupBinding {
    required u32 binding;
    required GPUBindingResource resource;
};

dictionary GPUBindGroupDescriptor : GPUObjectDescriptorBase {
    required GPUBindGroupLayout layout;
    required sequence<GPUBindGroupBinding> bindings;
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUBindGroup {
};
GPUBindGroup includes GPUObjectBase;

// ****************************************************************************
// PIPELINE CREATION (blend state, DS state, ..., pipelines)
// ****************************************************************************

// BlendState
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
    "one-minus-blend-color",
};

enum GPUBlendOperation {
    "add",
    "subtract",
    "reverse-subtract",
    "min",
    "max"
};

dictionary GPUBlendDescriptor {
    GPUBlendFactor srcFactor = "one";
    GPUBlendFactor dstFactor = "zero";
    GPUBlendOperation operation = "add";
};

typedef u32 GPUColorWriteFlags;
[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUColorWrite {
    const u32 NONE = 0;
    const u32 RED = 1;
    const u32 GREEN = 2;
    const u32 BLUE = 4;
    const u32 ALPHA = 8;
    const u32 ALL = 15;
};

dictionary GPUColorStateDescriptor {
    required GPUTextureFormat format;

    GPUBlendDescriptor alpha;
    GPUBlendDescriptor color;
    GPUColorWriteFlags writeMask = 0xF;
};

// DepthStencilState
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

dictionary GPUStencilStateFaceDescriptor {
    GPUCompareFunction compare = "always";
    GPUStencilOperation failOp = "keep";
    GPUStencilOperation depthFailOp = "keep";
    GPUStencilOperation passOp = "keep";
};

dictionary GPUDepthStencilStateDescriptor {
    required GPUTextureFormat format;

    boolean depthWriteEnabled = false;
    GPUCompareFunction depthCompare = "always";

    required GPUStencilStateFaceDescriptor stencilFront;
    required GPUStencilStateFaceDescriptor stencilBack;

    u32 stencilReadMask = 0xFFFFFFFF;
    u32 stencilWriteMask = 0xFFFFFFFF;
};

// InputState
enum GPUIndexFormat {
    "uint16",
    "uint32",
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
    "int4",
};

enum GPUInputStepMode {
    "vertex",
    "instance",
};

dictionary GPUVertexAttributeDescriptor {
    u64 offset = 0;
    required GPUVertexFormat format;
    required u32 shaderLocation;
};

dictionary GPUVertexBufferDescriptor {
    required u64 stride;
    GPUInputStepMode stepMode = "vertex";
    required sequence<GPUVertexAttributeDescriptor> attributeSet;
};

dictionary GPUVertexInputDescriptor {
    GPUIndexFormat indexFormat = "uint32";
    required sequence<GPUVertexBufferDescriptor?> vertexBuffers;
};

// ShaderModule
typedef (Uint32Array or DOMString) GPUShaderCode;

dictionary GPUShaderModuleDescriptor : GPUObjectDescriptorBase {
    required GPUShaderCode code;
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUShaderModule {
};
GPUShaderModule includes GPUObjectBase;

// Common stuff for ComputePipeline and RenderPipeline
dictionary GPUPipelineDescriptorBase : GPUObjectDescriptorBase {
    required GPUPipelineLayout layout;
};

dictionary GPUProgrammableStageDescriptor {
    required GPUShaderModule module_;
    required DOMString entryPoint;
};

// ComputePipeline
dictionary GPUComputePipelineDescriptor : GPUPipelineDescriptorBase {
    required GPUProgrammableStageDescriptor computeStage;
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUComputePipeline {
};
GPUComputePipeline includes GPUObjectBase;

// GPURenderPipeline
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

    i32 depthBias = 0;
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

dictionary GPURenderPipelineDescriptor : GPUPipelineDescriptorBase {
    required GPUProgrammableStageDescriptor vertexStage;
    GPUProgrammableStageDescriptor fragmentStage;

    required GPUPrimitiveTopology primitiveTopology;
    GPURasterizationStateDescriptor rasterizationState;
    required sequence<GPUColorStateDescriptor> colorStates;
    GPUDepthStencilStateDescriptor depthStencilState;
    required GPUVertexInputDescriptor vertexInput;

    u32 sampleCount = 1;
    u32 sampleMask = 0xFFFFFFFF;
    boolean alphaToCoverageEnabled = false;
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPURenderPipeline {
};
GPURenderPipeline includes GPUObjectBase;

// ****************************************************************************
// COMMAND RECORDING (Command buffer and all relevant structures)
// ****************************************************************************

enum GPULoadOp {
    "load"
};

enum GPUStoreOp {
    "store",
    "clear"
};

dictionary GPURenderPassColorAttachmentDescriptor {
    required GPUTextureView attachment;
    GPUTextureView resolveTarget;

    required (GPULoadOp or GPUColor) loadValue;
    required GPUStoreOp storeOp;
};

dictionary GPURenderPassDepthStencilAttachmentDescriptor {
    required GPUTextureView attachment;

    required (GPULoadOp or float) depthLoadValue;
    required GPUStoreOp depthStoreOp;

    required (GPULoadOp or u32) stencilLoadValue;
    required GPUStoreOp stencilStoreOp;
};

dictionary GPURenderPassDescriptor : GPUObjectDescriptorBase {
    required sequence<GPURenderPassColorAttachmentDescriptor> colorAttachments;
    GPURenderPassDepthStencilAttachmentDescriptor depthStencilAttachment;
};

dictionary GPUBufferCopyView {
    required GPUBuffer buffer;
    u64 offset = 0;
    required u32 rowPitch;
    required u32 imageHeight;
};

dictionary GPUTextureCopyView {
    required GPUTexture texture;
    u32 mipLevel = 0;
    u32 arrayLayer = 0;
    GPUOrigin3D origin;
};

dictionary GPUImageBitmapCopyView {
    //required ImageBitmap imageBitmap; //TODO
    GPUOrigin2D origin;
};

dictionary GPUCommandEncoderDescriptor : GPUObjectDescriptorBase {
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUCommandEncoder {
    //GPURenderPassEncoder beginRenderPass(GPURenderPassDescriptor descriptor);
    //GPUComputePassEncoder beginComputePass(optional GPUComputePassDescriptor descriptor = {});

    /*
    void copyBufferToBuffer(
        GPUBuffer source,
        u64 sourceOffset,
        GPUBuffer destination,
        u64 destinationOffset,
        u64 size);

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

    void copyImageBitmapToTexture(
        GPUImageBitmapCopyView source,
        GPUTextureCopyView destination,
        GPUExtent3D copySize);
    */

    //void pushDebugGroup(DOMString groupLabel);
    //void popDebugGroup();
    //void insertDebugMarker(DOMString markerLabel);

    //GPUCommandBuffer finish(optional GPUCommandBufferDescriptor descriptor = {});
};
GPUCommandEncoder includes GPUObjectBase;

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUProgrammablePassEncoder {
    //void setBindGroup(u32 index, GPUBindGroup bindGroup,
    //                  optional sequence<u64> dynamicOffsets);

    //void pushDebugGroup(DOMString groupLabel);
    //void popDebugGroup();
    //void insertDebugMarker(DOMString markerLabel);
};
GPUProgrammablePassEncoder includes GPUObjectBase;

// Render Pass
[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPURenderEncoderBase : GPUProgrammablePassEncoder {
    //void setPipeline(GPURenderPipeline pipeline);

    //void setIndexBuffer(GPUBuffer buffer, u64 offset);
    //void setVertexBuffers(u32 startSlot,
    //                      sequence<GPUBuffer> buffers, sequence<u64> offsets);

    //void draw(u32 vertexCount, u32 instanceCount,
    //          u32 firstVertex, u32 firstInstance);
    //void drawIndexed(u32 indexCount, u32 instanceCount,
    //                 u32 firstIndex, i32 baseVertex, u32 firstInstance);

    //void drawIndirect(GPUBuffer indirectBuffer, u64 indirectOffset);
    //void drawIndexedIndirect(GPUBuffer indirectBuffer, u64 indirectOffset);
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPURenderPassEncoder : GPURenderEncoderBase {
    //void setViewport(float x, float y,
    //                 float width, float height,
    //                 float minDepth, float maxDepth);

    //void setScissorRect(u32 x, u32 y, u32 width, u32 height);

    //void setBlendColor(GPUColor color);
    //void setStencilReference(u32 reference);

    //void executeBundles(sequence<GPURenderBundle> bundles);
    //void endPass();
};

// Compute Pass
dictionary GPUComputePassDescriptor : GPUObjectDescriptorBase {
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUComputePassEncoder : GPUProgrammablePassEncoder {
    //void setPipeline(GPUComputePipeline pipeline);
    //void dispatch(u32 x, optional u32 y = 1, optional u32 z = 1);
    //void dispatchIndirect(GPUBuffer indirectBuffer, u64 indirectOffset);

    //void endPass();
};

// Command Buffer
dictionary GPUCommandBufferDescriptor : GPUObjectDescriptorBase {
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUCommandBuffer {
};
GPUCommandBuffer includes GPUObjectBase;

dictionary GPURenderBundleEncoderDescriptor : GPUObjectDescriptorBase {
    required sequence<GPUTextureFormat> colorFormats;
    GPUTextureFormat depthStencilFormat;
    u32 sampleCount = 1;
};

// Render Bundle
[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPURenderBundleEncoder : GPURenderEncoderBase {
    //GPURenderBundle finish(optional GPURenderBundleDescriptor descriptor = {});
};

dictionary GPURenderBundleDescriptor : GPUObjectDescriptorBase {
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPURenderBundle {
};
GPURenderBundle includes GPUObjectBase;

// ****************************************************************************
// OTHER (Fence, Queue SwapChain, Device)
// ****************************************************************************

// Fence
dictionary GPUFenceDescriptor : GPUObjectDescriptorBase {
    u64 initialValue = 0;
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUFence {
    //u64 getCompletedValue();
    //Promise<void> onCompletion(u64 completionValue);
};
GPUFence includes GPUObjectBase;

// Queue
[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUQueue {
    //void submit(sequence<GPUCommandBuffer> buffers);

    //GPUFence createFence(optional GPUFenceDescriptor descriptor = {});
    //void signal(GPUFence fence, u64 signalValue);
};
GPUQueue includes GPUObjectBase;

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUSwapChain {
    //GPUTexture getCurrentTexture();
};
GPUSwapChain includes GPUObjectBase;

dictionary GPUSwapChainDescriptor : GPUObjectDescriptorBase {
    required GPUDevice device;
    required GPUTextureFormat format;
    GPUTextureUsageFlags usage = 0x10;  // GPUTextureUsage.OUTPUT_ATTACHMENT
};

[Pref="dom.webgpu.enable",
 Exposed=Window]
interface GPUCanvasContext {
    // Calling configureSwapChain a second time invalidates the previous one,
    // and all of the textures it's produced.
    //GPUSwapChain configureSwapChain(GPUSwapChainDescriptor descriptor);

    //Promise<GPUTextureFormat> getSwapChainPreferredFormat(GPUDevice device);
};
