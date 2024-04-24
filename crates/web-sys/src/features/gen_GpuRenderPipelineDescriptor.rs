#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPURenderPipelineDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuRenderPipelineDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuRenderPipelineDescriptor;
    #[wasm_bindgen(method, getter = "label")]
    fn label_shim(this: &GpuRenderPipelineDescriptor) -> String;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label_shim(this: &GpuRenderPipelineDescriptor, val: &str);
    #[wasm_bindgen(method, getter = "layout")]
    fn layout_shim(this: &GpuRenderPipelineDescriptor) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "layout")]
    fn set_layout_shim(this: &GpuRenderPipelineDescriptor, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "GpuDepthStencilState")]
    #[wasm_bindgen(method, getter = "depthStencil")]
    fn depth_stencil_shim(this: &GpuRenderPipelineDescriptor) -> GpuDepthStencilState;
    #[cfg(feature = "GpuDepthStencilState")]
    #[wasm_bindgen(method, setter = "depthStencil")]
    fn set_depth_stencil_shim(this: &GpuRenderPipelineDescriptor, val: &GpuDepthStencilState);
    #[cfg(feature = "GpuFragmentState")]
    #[wasm_bindgen(method, getter = "fragment")]
    fn fragment_shim(this: &GpuRenderPipelineDescriptor) -> GpuFragmentState;
    #[cfg(feature = "GpuFragmentState")]
    #[wasm_bindgen(method, setter = "fragment")]
    fn set_fragment_shim(this: &GpuRenderPipelineDescriptor, val: &GpuFragmentState);
    #[cfg(feature = "GpuMultisampleState")]
    #[wasm_bindgen(method, getter = "multisample")]
    fn multisample_shim(this: &GpuRenderPipelineDescriptor) -> GpuMultisampleState;
    #[cfg(feature = "GpuMultisampleState")]
    #[wasm_bindgen(method, setter = "multisample")]
    fn set_multisample_shim(this: &GpuRenderPipelineDescriptor, val: &GpuMultisampleState);
    #[cfg(feature = "GpuPrimitiveState")]
    #[wasm_bindgen(method, getter = "primitive")]
    fn primitive_shim(this: &GpuRenderPipelineDescriptor) -> GpuPrimitiveState;
    #[cfg(feature = "GpuPrimitiveState")]
    #[wasm_bindgen(method, setter = "primitive")]
    fn set_primitive_shim(this: &GpuRenderPipelineDescriptor, val: &GpuPrimitiveState);
    #[cfg(feature = "GpuVertexState")]
    #[wasm_bindgen(method, getter = "vertex")]
    fn vertex_shim(this: &GpuRenderPipelineDescriptor) -> GpuVertexState;
    #[cfg(feature = "GpuVertexState")]
    #[wasm_bindgen(method, setter = "vertex")]
    fn set_vertex_shim(this: &GpuRenderPipelineDescriptor, val: &GpuVertexState);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuRenderPipelineDescriptor` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
pub trait GpuRenderPipelineDescriptorGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn label(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn layout(&self) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDepthStencilState")]
    #[doc = "Get the `depthStencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_stencil(&self) -> GpuDepthStencilState;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFragmentState")]
    #[doc = "Get the `fragment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn fragment(&self) -> GpuFragmentState;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuMultisampleState")]
    #[doc = "Get the `multisample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuMultisampleState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn multisample(&self) -> GpuMultisampleState;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPrimitiveState")]
    #[doc = "Get the `primitive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn primitive(&self) -> GpuPrimitiveState;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexState")]
    #[doc = "Get the `vertex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`, `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn vertex(&self) -> GpuVertexState;
}
#[cfg(web_sys_unstable_apis)]
impl GpuRenderPipelineDescriptorGetters for GpuRenderPipelineDescriptor {
    #[cfg(web_sys_unstable_apis)]
    fn label(&self) -> String {
        self.label_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn layout(&self) -> ::wasm_bindgen::JsValue {
        self.layout_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDepthStencilState")]
    fn depth_stencil(&self) -> GpuDepthStencilState {
        self.depth_stencil_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFragmentState")]
    fn fragment(&self) -> GpuFragmentState {
        self.fragment_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuMultisampleState")]
    fn multisample(&self) -> GpuMultisampleState {
        self.multisample_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPrimitiveState")]
    fn primitive(&self) -> GpuPrimitiveState {
        self.primitive_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexState")]
    fn vertex(&self) -> GpuVertexState {
        self.vertex_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuRenderPipelineDescriptor {
    #[cfg(feature = "GpuVertexState")]
    #[doc = "Construct a new `GpuRenderPipelineDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`, `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(layout: &::wasm_bindgen::JsValue, vertex: &GpuVertexState) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.layout(layout);
        ret.vertex(vertex);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn layout(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_layout_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDepthStencilState")]
    #[doc = "Change the `depthStencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_stencil(&mut self, val: &GpuDepthStencilState) -> &mut Self {
        self.set_depth_stencil_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFragmentState")]
    #[doc = "Change the `fragment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn fragment(&mut self, val: &GpuFragmentState) -> &mut Self {
        self.set_fragment_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuMultisampleState")]
    #[doc = "Change the `multisample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuMultisampleState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn multisample(&mut self, val: &GpuMultisampleState) -> &mut Self {
        self.set_multisample_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPrimitiveState")]
    #[doc = "Change the `primitive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn primitive(&mut self, val: &GpuPrimitiveState) -> &mut Self {
        self.set_primitive_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexState")]
    #[doc = "Change the `vertex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`, `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn vertex(&mut self, val: &GpuVertexState) -> &mut Self {
        self.set_vertex_shim(val);
        self
    }
}
