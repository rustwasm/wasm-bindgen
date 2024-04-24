#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPURenderPassDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuRenderPassDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuRenderPassDescriptor;
    #[wasm_bindgen(method, getter = "label")]
    fn label_shim(this: &GpuRenderPassDescriptor) -> String;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label_shim(this: &GpuRenderPassDescriptor, val: &str);
    #[wasm_bindgen(method, getter = "colorAttachments")]
    fn color_attachments_shim(this: &GpuRenderPassDescriptor) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "colorAttachments")]
    fn set_color_attachments_shim(this: &GpuRenderPassDescriptor, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "GpuRenderPassDepthStencilAttachment")]
    #[wasm_bindgen(method, getter = "depthStencilAttachment")]
    fn depth_stencil_attachment_shim(
        this: &GpuRenderPassDescriptor,
    ) -> GpuRenderPassDepthStencilAttachment;
    #[cfg(feature = "GpuRenderPassDepthStencilAttachment")]
    #[wasm_bindgen(method, setter = "depthStencilAttachment")]
    fn set_depth_stencil_attachment_shim(
        this: &GpuRenderPassDescriptor,
        val: &GpuRenderPassDepthStencilAttachment,
    );
    #[wasm_bindgen(method, getter = "maxDrawCount")]
    fn max_draw_count_shim(this: &GpuRenderPassDescriptor) -> f64;
    #[wasm_bindgen(method, setter = "maxDrawCount")]
    fn set_max_draw_count_shim(this: &GpuRenderPassDescriptor, val: f64);
    #[cfg(feature = "GpuQuerySet")]
    #[wasm_bindgen(method, getter = "occlusionQuerySet")]
    fn occlusion_query_set_shim(this: &GpuRenderPassDescriptor) -> GpuQuerySet;
    #[cfg(feature = "GpuQuerySet")]
    #[wasm_bindgen(method, setter = "occlusionQuerySet")]
    fn set_occlusion_query_set_shim(this: &GpuRenderPassDescriptor, val: &GpuQuerySet);
    #[cfg(feature = "GpuRenderPassTimestampWrites")]
    #[wasm_bindgen(method, getter = "timestampWrites")]
    fn timestamp_writes_shim(this: &GpuRenderPassDescriptor) -> GpuRenderPassTimestampWrites;
    #[cfg(feature = "GpuRenderPassTimestampWrites")]
    #[wasm_bindgen(method, setter = "timestampWrites")]
    fn set_timestamp_writes_shim(
        this: &GpuRenderPassDescriptor,
        val: &GpuRenderPassTimestampWrites,
    );
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuRenderPassDescriptor` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
pub trait GpuRenderPassDescriptorGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn label(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `colorAttachments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn color_attachments(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPassDepthStencilAttachment")]
    #[doc = "Get the `depthStencilAttachment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`, `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_stencil_attachment(&self) -> GpuRenderPassDepthStencilAttachment;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `maxDrawCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn max_draw_count(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQuerySet")]
    #[doc = "Get the `occlusionQuerySet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQuerySet`, `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn occlusion_query_set(&self) -> GpuQuerySet;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPassTimestampWrites")]
    #[doc = "Get the `timestampWrites` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`, `GpuRenderPassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn timestamp_writes(&self) -> GpuRenderPassTimestampWrites;
}
#[cfg(web_sys_unstable_apis)]
impl GpuRenderPassDescriptorGetters for GpuRenderPassDescriptor {
    #[cfg(web_sys_unstable_apis)]
    fn label(&self) -> String {
        self.label_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn color_attachments(&self) -> ::js_sys::Array {
        self.color_attachments_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPassDepthStencilAttachment")]
    fn depth_stencil_attachment(&self) -> GpuRenderPassDepthStencilAttachment {
        self.depth_stencil_attachment_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn max_draw_count(&self) -> f64 {
        self.max_draw_count_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQuerySet")]
    fn occlusion_query_set(&self) -> GpuQuerySet {
        self.occlusion_query_set_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPassTimestampWrites")]
    fn timestamp_writes(&self) -> GpuRenderPassTimestampWrites {
        self.timestamp_writes_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuRenderPassDescriptor {
    #[doc = "Construct a new `GpuRenderPassDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(color_attachments: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.color_attachments(color_attachments);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `colorAttachments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn color_attachments(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_color_attachments_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPassDepthStencilAttachment")]
    #[doc = "Change the `depthStencilAttachment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`, `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_stencil_attachment(
        &mut self,
        val: &GpuRenderPassDepthStencilAttachment,
    ) -> &mut Self {
        self.set_depth_stencil_attachment_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `maxDrawCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_draw_count(&mut self, val: f64) -> &mut Self {
        self.set_max_draw_count_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQuerySet")]
    #[doc = "Change the `occlusionQuerySet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQuerySet`, `GpuRenderPassDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn occlusion_query_set(&mut self, val: &GpuQuerySet) -> &mut Self {
        self.set_occlusion_query_set_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPassTimestampWrites")]
    #[doc = "Change the `timestampWrites` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`, `GpuRenderPassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn timestamp_writes(&mut self, val: &GpuRenderPassTimestampWrites) -> &mut Self {
        self.set_timestamp_writes_shim(val);
        self
    }
}
