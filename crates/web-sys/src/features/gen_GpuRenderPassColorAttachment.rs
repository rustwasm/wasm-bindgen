#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPURenderPassColorAttachment)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuRenderPassColorAttachment` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassColorAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuRenderPassColorAttachment;
    #[wasm_bindgen(method, getter = "clearValue")]
    fn clear_value_shim(this: &GpuRenderPassColorAttachment) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "clearValue")]
    fn set_clear_value_shim(this: &GpuRenderPassColorAttachment, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "depthSlice")]
    fn depth_slice_shim(this: &GpuRenderPassColorAttachment) -> u32;
    #[wasm_bindgen(method, setter = "depthSlice")]
    fn set_depth_slice_shim(this: &GpuRenderPassColorAttachment, val: u32);
    #[cfg(feature = "GpuLoadOp")]
    #[wasm_bindgen(method, getter = "loadOp")]
    fn load_op_shim(this: &GpuRenderPassColorAttachment) -> GpuLoadOp;
    #[cfg(feature = "GpuLoadOp")]
    #[wasm_bindgen(method, setter = "loadOp")]
    fn set_load_op_shim(this: &GpuRenderPassColorAttachment, val: GpuLoadOp);
    #[cfg(feature = "GpuTextureView")]
    #[wasm_bindgen(method, getter = "resolveTarget")]
    fn resolve_target_shim(this: &GpuRenderPassColorAttachment) -> GpuTextureView;
    #[cfg(feature = "GpuTextureView")]
    #[wasm_bindgen(method, setter = "resolveTarget")]
    fn set_resolve_target_shim(this: &GpuRenderPassColorAttachment, val: &GpuTextureView);
    #[cfg(feature = "GpuStoreOp")]
    #[wasm_bindgen(method, getter = "storeOp")]
    fn store_op_shim(this: &GpuRenderPassColorAttachment) -> GpuStoreOp;
    #[cfg(feature = "GpuStoreOp")]
    #[wasm_bindgen(method, setter = "storeOp")]
    fn set_store_op_shim(this: &GpuRenderPassColorAttachment, val: GpuStoreOp);
    #[cfg(feature = "GpuTextureView")]
    #[wasm_bindgen(method, getter = "view")]
    fn view_shim(this: &GpuRenderPassColorAttachment) -> GpuTextureView;
    #[cfg(feature = "GpuTextureView")]
    #[wasm_bindgen(method, setter = "view")]
    fn set_view_shim(this: &GpuRenderPassColorAttachment, val: &GpuTextureView);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuRenderPassColorAttachment` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuRenderPassColorAttachment`*"]
pub trait GpuRenderPassColorAttachmentGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `clearValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassColorAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn clear_value(&self) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthSlice` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassColorAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_slice(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuLoadOp")]
    #[doc = "Get the `loadOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuLoadOp`, `GpuRenderPassColorAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn load_op(&self) -> GpuLoadOp;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureView")]
    #[doc = "Get the `resolveTarget` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassColorAttachment`, `GpuTextureView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn resolve_target(&self) -> GpuTextureView;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStoreOp")]
    #[doc = "Get the `storeOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassColorAttachment`, `GpuStoreOp`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn store_op(&self) -> GpuStoreOp;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureView")]
    #[doc = "Get the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassColorAttachment`, `GpuTextureView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn view(&self) -> GpuTextureView;
}
#[cfg(web_sys_unstable_apis)]
impl GpuRenderPassColorAttachmentGetters for GpuRenderPassColorAttachment {
    #[cfg(web_sys_unstable_apis)]
    fn clear_value(&self) -> ::wasm_bindgen::JsValue {
        self.clear_value_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn depth_slice(&self) -> u32 {
        self.depth_slice_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuLoadOp")]
    fn load_op(&self) -> GpuLoadOp {
        self.load_op_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureView")]
    fn resolve_target(&self) -> GpuTextureView {
        self.resolve_target_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStoreOp")]
    fn store_op(&self) -> GpuStoreOp {
        self.store_op_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureView")]
    fn view(&self) -> GpuTextureView {
        self.view_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuRenderPassColorAttachment {
    #[cfg(all(
        feature = "GpuLoadOp",
        feature = "GpuStoreOp",
        feature = "GpuTextureView",
    ))]
    #[doc = "Construct a new `GpuRenderPassColorAttachment`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuLoadOp`, `GpuRenderPassColorAttachment`, `GpuStoreOp`, `GpuTextureView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(load_op: GpuLoadOp, store_op: GpuStoreOp, view: &GpuTextureView) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::load_op(&mut ret, load_op);
        Self::store_op(&mut ret, store_op);
        Self::view(&mut ret, view);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `clearValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassColorAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn clear_value(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_clear_value_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthSlice` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassColorAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_slice(&mut self, val: u32) -> &mut Self {
        self.set_depth_slice_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuLoadOp")]
    #[doc = "Change the `loadOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuLoadOp`, `GpuRenderPassColorAttachment`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn load_op(&mut self, val: GpuLoadOp) -> &mut Self {
        self.set_load_op_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureView")]
    #[doc = "Change the `resolveTarget` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassColorAttachment`, `GpuTextureView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn resolve_target(&mut self, val: &GpuTextureView) -> &mut Self {
        self.set_resolve_target_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStoreOp")]
    #[doc = "Change the `storeOp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassColorAttachment`, `GpuStoreOp`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn store_op(&mut self, val: GpuStoreOp) -> &mut Self {
        self.set_store_op_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureView")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassColorAttachment`, `GpuTextureView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn view(&mut self, val: &GpuTextureView) -> &mut Self {
        self.set_view_shim(val);
        self
    }
}
