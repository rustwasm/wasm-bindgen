/* -*- Mode: IDL; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 4 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * https://github.com/gpuweb/gpuweb/blob/master/design/sketch.webidl
 */

typedef unsigned long u32;
typedef unsigned long long u64;

// ****************************************************************************
// ERROR HANDLING
// ****************************************************************************

enum WebGPULogEntryType {
    "device-lost",
    "validation-error",
    "recoverable-out-of-memory",
};

[Pref="dom.webgpu.enable"]
interface WebGPULogEntry {
    readonly attribute WebGPULogEntryType type;
    readonly attribute any obj;
    readonly attribute DOMString? reason;
};

enum WebGPUObjectStatus {
    "valid",
    "out-of-memory",
    "invalid",
};

typedef (WebGPUBuffer or WebGPUTexture) WebGPUStatusable;

callback WebGPULogCallback = void (WebGPULogEntry error);

// ****************************************************************************
// SHADER RESOURCES (buffer, textures, texture views, samples)
// ****************************************************************************

// Buffer
typedef u32 WebGPUBufferUsageFlags;
[Pref="dom.webgpu.enable"]
interface WebGPUBufferUsage {
    const u32 NONE = 0;
    const u32 MAP_READ = 1;
    const u32 MAP_WRITE = 2;
    const u32 TRANSFER_SRC = 4;
    const u32 TRANSFER_DST = 8;
    const u32 INDEX = 16;
    const u32 VERTEX = 32;
    const u32 UNIFORM = 64;
    const u32 STORAGE = 128;
};

dictionary WebGPUBufferDescriptor {
    u32 size;
    WebGPUBufferUsageFlags usage;
};

[Pref="dom.webgpu.enable"]
interface WebGPUBuffer {
    readonly attribute ArrayBuffer? mapping;
    void unmap();
};

// Texture view
dictionary WebGPUTextureViewDescriptor {
    // TODO Investigate what goes in there.
};

[Pref="dom.webgpu.enable"]
interface WebGPUTextureView {
};

// Texture
typedef u32 WebGPUTextureDimensionEnum;
[Pref="dom.webgpu.enable"]
interface WebGPUTextureDimension {
    const u32 e1D = 0;
    const u32 e2D = 1;
    const u32 e3D = 2;
    // TODO other dimensions (cube, arrays)
};

typedef u32 WebGPUTextureFormatEnum;
[Pref="dom.webgpu.enable"]
interface WebGPUTextureFormat {
    const u32 R8_G8_B8_A8_UNORM = 0;
    const u32 R8_G8_B8_A8_UINT = 1;
    const u32 B8_G8_R8_A8_UNORM = 2;
    const u32 D32_FLOAT_S8_UINT = 3;
    // TODO other formats
};

typedef u32 WebGPUTextureUsageFlags;
[Pref="dom.webgpu.enable"]
interface WebGPUTextureUsage {
    const u32 NONE = 0;
    const u32 TRANSFER_SRC = 1;
    const u32 TRANSFER_DST = 2;
    const u32 SAMPLED = 4;
    const u32 STORAGE = 8;
    const u32 OUTPUT_ATTACHMENT = 16;
    const u32 PRESENT = 32;
};

dictionary WebGPUTextureDescriptor {
    u32 width;
    u32 height;
    u32 depth;
    u32 arraySize;
    WebGPUTextureDimensionEnum dimension;
    WebGPUTextureFormatEnum format;
    WebGPUTextureUsageFlags usage;
};

[Pref="dom.webgpu.enable"]
interface WebGPUTexture {
    WebGPUTextureView createTextureView(optional WebGPUTextureViewDescriptor desc);
};

// Sampler
typedef u32 WebGPUFilterModeEnum;
[Pref="dom.webgpu.enable"]
interface WebGPUFilterMode {
    const u32 NEAREST = 0;
    const u32 LINEAR = 1;
};

dictionary WebGPUSamplerDescriptor {
    WebGPUFilterModeEnum magFilter;
    WebGPUFilterModeEnum minFilter;
    WebGPUFilterModeEnum mipmapFilter;
};

[Pref="dom.webgpu.enable"]
interface WebGPUSampler {
};

// ****************************************************************************
// BINDING MODEL (bindgroup layout, bindgroup)
// ****************************************************************************

// BindGroupLayout
typedef u32 WebGPUShaderStageFlags;
[Pref="dom.webgpu.enable"]
interface WebGPUShaderStageBit {
    const u32 NONE = 0;
    const u32 VERTEX = 1;
    const u32 FRAGMENT = 2;
    const u32 COMPUTE = 4;
};

typedef u32 WebGPUBindingTypeEnum;
[Pref="dom.webgpu.enable"]
interface WebGPUBindingType {
    const u32 UNIFORM_BUFFER = 0;
    const u32 SAMPLER = 1;
    const u32 SAMPLED_TEXTURE = 2;
    const u32 STORAGE_BUFFER = 3;
    // TODO other binding types (storage images, ...)
};

dictionary WebGPUBindGroupBinding {
    WebGPUShaderStageFlags visibility;
    WebGPUBindingTypeEnum type;
    u32 start;
    u32 count;
};

dictionary WebGPUBindGroupLayoutDescriptor {
    sequence<WebGPUBindGroupBinding> bindingTypes;
};

[Pref="dom.webgpu.enable"]
interface WebGPUBindGroupLayout {
};

// PipelineLayout
dictionary WebGPUPipelineLayoutDescriptor {
    sequence<WebGPUBindGroupLayout> bindGroupLayouts;
};

[Pref="dom.webgpu.enable"]
interface WebGPUPipelineLayout {
};

// BindGroup
/* Moved to WebGPUExtras.webidl for now.
dictionary WebGPUBufferBinding {
    WebGPUBuffer buffer;
    u32 offset;
    u32 size;
};
*/

typedef (WebGPUSampler or WebGPUTextureView or WebGPUBufferBinding) WebGPUBindingResource;

dictionary WebGPUBinding {
    sequence<WebGPUBindingResource> resources;
    u32 start;
    u32 count;
};

dictionary WebGPUBindGroupDescriptor {
    WebGPUBindGroupLayout layout;
    sequence<WebGPUBinding> bindings;
};

[Pref="dom.webgpu.enable"]
interface WebGPUBindGroup {
};

// ****************************************************************************
// PIPELINE CREATION (blend state, DS state, ..., pipelines)
// ****************************************************************************

// BlendState
typedef u32 WebGPUBlendFactorEnum;
[Pref="dom.webgpu.enable"]
interface WebGPUBlendFactor {
    const u32 ZERO = 0;
    const u32 ONE = 1;
    const u32 SRC_COLOR = 2;
    const u32 ONE_MINUS_SRC_COLOR = 3;
    const u32 SRC_ALPHA = 4;
    const u32 ONE_MINUS_SRC_ALPHA = 5;
    const u32 DST_COLOR = 6;
    const u32 ONE_MINUS_DST_COLOR = 7;
    const u32 DST_ALPHA = 8;
    const u32 ONE_MINUS_DST_ALPHA = 9;
    const u32 SRC_ALPHA_SATURATED = 10;
    const u32 BLEND_COLOR = 11;
    const u32 ONE_MINUS_BLEND_COLOR = 12;
};

typedef u32 WebGPUBlendOperationEnum;
[Pref="dom.webgpu.enable"]
interface WebGPUBlendOperation {
    const u32 ADD = 0;
    const u32 SUBTRACT = 1;
    const u32 REVERSE_SUBTRACT = 2;
    const u32 MIN = 3;
    const u32 MAX = 4;
};

typedef u32 WebGPUColorWriteFlags;
[Pref="dom.webgpu.enable"]
interface WebGPUColorWriteBits {
    const u32 NONE = 0;
    const u32 RED = 1;
    const u32 GREEN = 2;
    const u32 BLUE = 4;
    const u32 ALPHA = 8;
    const u32 ALL = 15;
};

dictionary WebGPUBlendDescriptor {
    WebGPUBlendFactorEnum srcFactor;
    WebGPUBlendFactorEnum dstFactor;
    WebGPUBlendOperationEnum operation;
};

dictionary WebGPUBlendStateDescriptor {
    boolean blendEnabled;
    WebGPUBlendDescriptor alpha;
    WebGPUBlendDescriptor color;
    WebGPUColorWriteFlags writeMask;
};

[Pref="dom.webgpu.enable"]
interface WebGPUBlendState {
};

// DepthStencilState
typedef u32 WebGPUCompareFunctionEnum;
[Pref="dom.webgpu.enable"]
interface WebGPUCompareFunction {
    const u32 NEVER = 0;
    const u32 LESS = 1;
    const u32 LESS_EQUAL = 2;
    const u32 GREATER = 3;
    const u32 GREATER_EQUAL = 4;
    const u32 EQUAL = 5;
    const u32 NOT_EQUAL = 6;
    const u32 ALWAYS = 7;
};

typedef u32 WebGPUStencilOperationEnum;
[Pref="dom.webgpu.enable"]
interface WebGPUStencilOperation {
    const u32 KEEP = 0;
    const u32 ZERO = 1;
    const u32 REPLACE = 2;
    const u32 INVERT = 3;
    const u32 INCREMENT_CLAMP = 4;
    const u32 DECREMENT_CLAMP = 5;
    const u32 INCREMENT_WRAP = 6;
    const u32 DECREMENT_WRAP = 7;
};

dictionary WebGPUStencilStateFaceDescriptor {
    WebGPUCompareFunctionEnum compare;
    WebGPUStencilOperationEnum stencilFailOp;
    WebGPUStencilOperationEnum depthFailOp;
    WebGPUStencilOperationEnum passOp;
};

dictionary WebGPUDepthStencilStateDescriptor {
    boolean depthWriteEnabled;
    WebGPUCompareFunctionEnum depthCompare;

    WebGPUStencilStateFaceDescriptor front;
    WebGPUStencilStateFaceDescriptor back;

    u32 stencilReadMask;
    u32 stencilWriteMask;
};

[Pref="dom.webgpu.enable"]
interface WebGPUDepthStencilState {
};

// InputState
typedef u32 WebGPUIndexFormatEnum;
[Pref="dom.webgpu.enable"]
interface WebGPUIndexFormat {
    const u32 UINT16 = 0;
    const u32 UINT32 = 1;
};

typedef u32 WebGPUVertexFormatEnum;
[Pref="dom.webgpu.enable"]
interface WebGPUVertexFormat {
    const u32 FLOAT_R32_G32_B32_A32 = 0;
    const u32 FLOAT_R32_G32_B32 = 1;
    const u32 FLOAT_R32_G32 = 2;
    const u32 FLOAT_R32 = 3;
    // TODO other vertex formats
};

typedef u32 WebGPUInputStepModeEnum;
[Pref="dom.webgpu.enable"]
interface WebGPUInputStepMode {
    const u32 VERTEX = 0;
    const u32 INSTANCE = 1;
};

dictionary WebGPUVertexAttributeDescriptor {
    u32 shaderLocation;
    u32 inputSlot;
    u32 offset;
    WebGPUVertexFormatEnum format;
};

dictionary WebGPUVertexInputDescriptor {
    u32 inputSlot;
    u32 stride;
    WebGPUInputStepModeEnum stepMode;
};

dictionary WebGPUInputStateDescriptor {
    WebGPUIndexFormatEnum indexFormat;

    sequence<WebGPUVertexAttributeDescriptor> attributes;
    sequence<WebGPUVertexInputDescriptor> inputs;
};

[Pref="dom.webgpu.enable"]
interface WebGPUInputState {
};

// ShaderModule
dictionary WebGPUShaderModuleDescriptor {
    required ArrayBuffer code;
};

[Pref="dom.webgpu.enable"]
interface WebGPUShaderModule {
};

// AttachmentState
dictionary WebGPUAttachmentStateDescriptor {
    sequence<WebGPUTextureFormatEnum> formats;
    // TODO other stuff like sample count etc.
};

[Pref="dom.webgpu.enable"]
interface WebGPUAttachmentState {
};

// Common stuff for ComputePipeline and RenderPipeline
typedef u32 WebGPUShaderStageEnum;
[Pref="dom.webgpu.enable"]
interface WebGPUShaderStage {
    const u32 VERTEX = 0;
    const u32 FRAGMENT = 1;
    const u32 COMPUTE = 2;
};

dictionary WebGPUPipelineStageDescriptor {
    required WebGPUShaderModule shaderModule;
    required WebGPUShaderStageEnum stage;
    required DOMString entryPoint;
    // TODO other stuff like specialization constants?
};

dictionary WebGPUPipelineDescriptorBase {
    required WebGPUPipelineLayout layout;
    sequence<WebGPUPipelineStageDescriptor> stages;
};

// ComputePipeline
dictionary WebGPUComputePipelineDescriptor : WebGPUPipelineDescriptorBase {
};

[Pref="dom.webgpu.enable"]
interface WebGPUComputePipeline {
};

// WebGPURenderPipeline
typedef u32 WebGPUPrimitiveTopologyEnum;
[Pref="dom.webgpu.enable"]
interface WebGPUPrimitiveTopology {
    const u32 POINT_LIST = 0;
    const u32 LINE_LIST = 1;
    const u32 LINE_STRIP = 2;
    const u32 TRIANGLE_LIST = 3;
    const u32 TRIANGLE_STRIP = 4;
};

dictionary WebGPURenderPipelineDescriptor : WebGPUPipelineDescriptorBase {
    WebGPUPrimitiveTopologyEnum primitiveTopology;
    sequence<WebGPUBlendState> blendState;
    WebGPUDepthStencilState depthStencilState;
    WebGPUInputState inputState;
    WebGPUAttachmentState attachmentState;
    // TODO other properties
};

[Pref="dom.webgpu.enable"]
interface WebGPURenderPipeline {
};
// ****************************************************************************
// COMMAND RECORDING (Command buffer and all relevant structures)
// ****************************************************************************

typedef u32 WebGPULoadOpEnum;
[Pref="dom.webgpu.enable"]
interface WebGPULoadOp {
    const u32 CLEAR = 0;
    const u32 LOAD = 1;
};

typedef u32 WebGPUStoreOpEnum;
[Pref="dom.webgpu.enable"]
interface WebGPUStoreOp {
    const u32 STORE = 0;
};

dictionary WebGPURenderPassAttachmentDescriptor {
    WebGPUTextureView attachment;
    WebGPULoadOpEnum loadOp;
    WebGPUStoreOpEnum storeOp;
};

dictionary WebGPURenderPassDescriptor {
    sequence<WebGPURenderPassAttachmentDescriptor> colorAttachments;
    WebGPURenderPassAttachmentDescriptor depthStencilAttachment;
};

[Pref="dom.webgpu.enable"]
interface WebGPUCommandBuffer {
};

dictionary WebGPUCommandEncoderDescriptor {
};

[Pref="dom.webgpu.enable"]
interface WebGPUCommandEncoder {
    WebGPUCommandBuffer finishEncoding();

    // Commands allowed outside of "passes"
    void copyBufferToBuffer(WebGPUBuffer src,
                            u32 srcOffset,
                            WebGPUBuffer dst,
                            u32 dstOffset,
                            u32 size);
    // TODO figure out all the arguments required for these
    void copyBufferToTexture();
    void copyTextureToBuffer();
    void copyTextureToTexture();
    void blit();

    void transitionBuffer(WebGPUBuffer b, WebGPUBufferUsageFlags f);

    // Allowed in both compute and render passes
    void setPushConstants(WebGPUShaderStageFlags stage,
                          u32 offset,
                          u32 count,
                          ArrayBuffer data);
    void setBindGroup(u32 index, WebGPUBindGroup bindGroup);
    void setPipeline((WebGPUComputePipeline or WebGPURenderPipeline) pipeline);

    // Compute pass commands
    void beginComputePass();
    void endComputePass();

    void dispatch(u32 x, u32 y, u32 z);

    // Render pass commands
    void beginRenderPass(optional WebGPURenderPassDescriptor descriptor);
    void endRenderPass();

    void setBlendColor(float r, float g, float b, float a);
    void setIndexBuffer(WebGPUBuffer buffer, u32 offset);
    void setVertexBuffers(u32 startSlot, sequence<WebGPUBuffer> buffers, sequence<u32> offsets);

    void draw(u32 vertexCount, u32 instanceCount, u32 firstVertex, u32 firstInstance);
    void drawIndexed(u32 indexCount, u32 instanceCount, u32 firstIndex, u32 firstInstance, u32 firstVertex);

    // TODO add missing commands
};

// ****************************************************************************
// OTHER (Fence, Queue SwapChain, Device)
// ****************************************************************************

// Fence
[Pref="dom.webgpu.enable"]
interface WebGPUFence {
    boolean wait(double milliseconds);
    readonly attribute Promise<void> promise;
};

// Queue
[Pref="dom.webgpu.enable"]
interface WebGPUQueue {
    void submit(sequence<WebGPUCommandBuffer> buffers);
    WebGPUFence insertFence();
};

// SwapChain / RenderingContext
dictionary WebGPUSwapChainDescriptor {
    WebGPUTextureUsageFlags usage;
    WebGPUTextureFormatEnum format;
    u32 width;
    u32 height;
};

[Pref="dom.webgpu.enable"]
interface WebGPUSwapChain {
    void configure(optional WebGPUSwapChainDescriptor descriptor);
    WebGPUTexture getNextTexture();
    void present();
};

//[Pref="dom.webgpu.enable"]
//interface WebGPURenderingContext : WebGPUSwapChain {
//};

// WebGPU "namespace" used for device creation
dictionary WebGPUExtensions {
    boolean anisotropicFiltering;
    boolean logicOp; // Previously a "Feature".
};

dictionary WebGPULimits {
    u32 maxBindGroups;
};

// Device
[Pref="dom.webgpu.enable"]
interface WebGPUDevice {
    readonly attribute WebGPUAdapter adapter;
    WebGPUExtensions extensions();
    WebGPULimits limits();

    WebGPUBuffer createBuffer(optional WebGPUBufferDescriptor descriptor);
    WebGPUTexture createTexture(optional WebGPUTextureDescriptor descriptor);
    WebGPUSampler createSampler(optional WebGPUSamplerDescriptor descriptor);

    WebGPUBindGroupLayout createBindGroupLayout(optional WebGPUBindGroupLayoutDescriptor descriptor);
    WebGPUPipelineLayout createPipelineLayout(optional WebGPUPipelineLayoutDescriptor descriptor);
    WebGPUBindGroup createBindGroup(optional WebGPUBindGroupDescriptor descriptor);

    WebGPUBlendState createBlendState(optional WebGPUBlendStateDescriptor descriptor);
    WebGPUDepthStencilState createDepthStencilState(optional WebGPUDepthStencilStateDescriptor descriptor);
    WebGPUInputState createInputState(optional WebGPUInputStateDescriptor descriptor);
    WebGPUShaderModule createShaderModule(WebGPUShaderModuleDescriptor descriptor);
    WebGPUAttachmentState createAttachmentState(optional WebGPUAttachmentStateDescriptor descriptor);
    WebGPUComputePipeline createComputePipeline(WebGPUComputePipelineDescriptor descriptor);
    WebGPURenderPipeline createRenderPipeline(WebGPURenderPipelineDescriptor descriptor);

    WebGPUCommandEncoder createCommandEncoder(optional WebGPUCommandEncoderDescriptor descriptor);

    WebGPUQueue getQueue();

    attribute WebGPULogCallback onLog;
    Promise<WebGPUObjectStatus> getObjectStatus(WebGPUStatusable obj);
};

dictionary WebGPUDeviceDescriptor {
    WebGPUExtensions extensions;
    //WebGPULimits limits; Don't expose higher limits for now.

    // TODO are other things configurable like queues?
};

[Pref="dom.webgpu.enable"]
interface WebGPUAdapter {
    readonly attribute DOMString name;
    WebGPUExtensions extensions();
    //WebGPULimits limits(); Don't expose higher limits for now.

    WebGPUDevice createDevice(optional WebGPUDeviceDescriptor descriptor);
};

enum WebGPUPowerPreference { "default", "low-power", "high-performance" };

dictionary WebGPUAdapterDescriptor {
    WebGPUPowerPreference powerPreference;
};

[Pref="dom.webgpu.enable"]
interface WebGPU {
    WebGPUAdapter getAdapter(optional WebGPUAdapterDescriptor desc);
};

// Add a "webgpu" member to Window that contains the global instance of a "WebGPU"
[NoInterfaceObject]
interface WebGPUProvider {
    [SameObject, Replaceable, Pref="dom.webgpu.enable"] readonly attribute WebGPU webgpu;
};
//Window includes WebGPUProvider;
