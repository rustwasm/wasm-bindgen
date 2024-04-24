#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPURenderPassDepthStencilAttachment)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuRenderPassDepthStencilAttachment` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuRenderPassDepthStencilAttachment;
    #[wasm_bindgen(method, getter = "depthClearValue")]
    fn depth_clear_value_shim(this: &GpuRenderPassDepthStencilAttachment) -> f32;
    #[wasm_bindgen(method, setter = "depthClearValue")]
    fn set_depth_clear_value_shim(this: &GpuRenderPassDepthStencilAttachment, val: f32);
    #[cfg(feature = "GpuLoadOp")]
    #[wasm_bindgen(method, getter = "depthLoadOp")]
    fn depth_load_op_shim(this: &GpuRenderPassDepthStencilAttachment) -> GpuLoadOp;
    #[cfg(feature = "GpuLoadOp")]
    #[wasm_bindgen(method, setter = "depthLoadOp")]
    fn set_depth_load_op_shim(this: &GpuRenderPassDepthStencilAttachment, val: GpuLoadOp);
    #[wasm_bindgen(method, getter = "depthReadOnly")]
    fn depth_read_only_shim(this: &GpuRenderPassDepthStencilAttachment) -> bool;
    #[wasm_bindgen(method, setter = "depthReadOnly")]
    fn set_depth_read_only_shim(this: &GpuRenderPassDepthStencilAttachment, val: bool);
    #[cfg(feature = "GpuStoreOp")]
    #[wasm_bindgen(method, getter = "depthStoreOp")]
    fn depth_store_op_shim(this: &GpuRenderPassDepthStencilAttachment) -> GpuStoreOp;
    #[cfg(feature = "GpuStoreOp")]
    #[wasm_bindgen(method, setter = "depthStoreOp")]
    fn set_depth_store_op_shim(this: &GpuRenderPassDepthStencilAttachment, val: GpuStoreOp);
    #[wasm_bindgen(method, getter = "stencilClearValue")]
    fn stencil_clear_value_shim(this: &GpuRenderPassDepthStencilAttachment) -> u32;
    #[wasm_bindgen(method, setter = "stencilClearValue")]
    fn set_stencil_clear_value_shim(this: &GpuRenderPassDepthStencilAttachment, val: u32);
    #[cfg(feature = "GpuLoadOp")]
    #[wasm_bindgen(method, getter = "stencilLoadOp")]
    fn stencil_load_op_shim(this: &GpuRenderPassDepthStencilAttachment) -> GpuLoadOp;
    #[cfg(feature = "GpuLoadOp")]
    #[wasm_bindgen(method, setter = "stencilLoadOp")]
    fn set_stencil_load_op_shim(this: &GpuRenderPassDepthStencilAttachment, val: GpuLoadOp);
    #[wasm_bindgen(method, getter = "stencilReadOnly")]
    fn stencil_read_only_shim(this: &GpuRenderPassDepthStencilAttachment) -> bool;
    #[wasm_bindgen(method, setter = "stencilReadOnly")]
    fn set_stencil_read_only_shim(this: &GpuRenderPassDepthStencilAttachment, val: bool);
    #[cfg(feature = "GpuStoreOp")]
    #[wasm_bindgen(method, getter = "stencilStoreOp")]
    fn stencil_store_op_shim(this: &GpuRenderPassDepthStencilAttachment) -> GpuStoreOp;
    #[cfg(feature = "GpuStoreOp")]
    #[wasm_bindgen(method, setter = "stencilStoreOp")]
    fn set_stencil_store_op_shim(this: &GpuRenderPassDepthStencilAttachment, val: GpuStoreOp);
    #[cfg(feature = "GpuTextureView")]
    #[wasm_bindgen(method, getter = "view")]
    fn view_shim(this: &GpuRenderPassDepthStencilAttachment) -> GpuTextureView;
    #[cfg(feature = "GpuTextureView")]
    #[wasm_bindgen(method, setter = "view")]
    fn set_view_shim(this: &GpuRenderPassDepthStencilAttachment, val: &GpuTextureView);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuRenderPassDepthStencilAttachment` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`*"]
pub trait GpuRenderPassDepthStencilAttachmentGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthClearValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_clear_value(&self) -> f32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuLoadOp")]
    #[doc = "Get the `depthLoadOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuLoadOp`, `GpuRenderPassDepthStencilAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_load_op(&self) -> GpuLoadOp;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthReadOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_read_only(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStoreOp")]
    #[doc = "Get the `depthStoreOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`, `GpuStoreOp`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_store_op(&self) -> GpuStoreOp;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `stencilClearValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn stencil_clear_value(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuLoadOp")]
    #[doc = "Get the `stencilLoadOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuLoadOp`, `GpuRenderPassDepthStencilAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn stencil_load_op(&self) -> GpuLoadOp;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `stencilReadOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn stencil_read_only(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStoreOp")]
    #[doc = "Get the `stencilStoreOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`, `GpuStoreOp`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn stencil_store_op(&self) -> GpuStoreOp;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureView")]
    #[doc = "Get the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`, `GpuTextureView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn view(&self) -> GpuTextureView;
}
#[cfg(web_sys_unstable_apis)]
impl GpuRenderPassDepthStencilAttachmentGetters for GpuRenderPassDepthStencilAttachment {
    #[cfg(web_sys_unstable_apis)]
    fn depth_clear_value(&self) -> f32 {
        self.depth_clear_value_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuLoadOp")]
    fn depth_load_op(&self) -> GpuLoadOp {
        self.depth_load_op_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn depth_read_only(&self) -> bool {
        self.depth_read_only_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStoreOp")]
    fn depth_store_op(&self) -> GpuStoreOp {
        self.depth_store_op_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn stencil_clear_value(&self) -> u32 {
        self.stencil_clear_value_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuLoadOp")]
    fn stencil_load_op(&self) -> GpuLoadOp {
        self.stencil_load_op_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn stencil_read_only(&self) -> bool {
        self.stencil_read_only_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStoreOp")]
    fn stencil_store_op(&self) -> GpuStoreOp {
        self.stencil_store_op_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureView")]
    fn view(&self) -> GpuTextureView {
        self.view_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuRenderPassDepthStencilAttachment {
    #[cfg(feature = "GpuTextureView")]
    #[doc = "Construct a new `GpuRenderPassDepthStencilAttachment`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`, `GpuTextureView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(view: &GpuTextureView) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::view(&mut ret, view);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthClearValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_clear_value(&mut self, val: f32) -> &mut Self {
        self.set_depth_clear_value_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuLoadOp")]
    #[doc = "Change the `depthLoadOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuLoadOp`, `GpuRenderPassDepthStencilAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_load_op(&mut self, val: GpuLoadOp) -> &mut Self {
        self.set_depth_load_op_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthReadOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_read_only(&mut self, val: bool) -> &mut Self {
        self.set_depth_read_only_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStoreOp")]
    #[doc = "Change the `depthStoreOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`, `GpuStoreOp`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_store_op(&mut self, val: GpuStoreOp) -> &mut Self {
        self.set_depth_store_op_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `stencilClearValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn stencil_clear_value(&mut self, val: u32) -> &mut Self {
        self.set_stencil_clear_value_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuLoadOp")]
    #[doc = "Change the `stencilLoadOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuLoadOp`, `GpuRenderPassDepthStencilAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn stencil_load_op(&mut self, val: GpuLoadOp) -> &mut Self {
        self.set_stencil_load_op_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `stencilReadOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn stencil_read_only(&mut self, val: bool) -> &mut Self {
        self.set_stencil_read_only_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStoreOp")]
    #[doc = "Change the `stencilStoreOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`, `GpuStoreOp`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn stencil_store_op(&mut self, val: GpuStoreOp) -> &mut Self {
        self.set_stencil_store_op_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureView")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachment`, `GpuTextureView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn view(&mut self, val: &GpuTextureView) -> &mut Self {
        self.set_view_shim(val);
        self
    }
}
