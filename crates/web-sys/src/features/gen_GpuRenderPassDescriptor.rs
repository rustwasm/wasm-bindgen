use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPURenderPassDescriptor ) ]
    #[doc = "The `GpuRenderPassDescriptor` dictionary.\n\n*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    pub type GpuRenderPassDescriptor;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassDescriptor {
    #[doc = "Construct a new `GpuRenderPassDescriptor`.\n\n*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    pub fn new(color_attachments: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.color_attachments(color_attachments);
        ret
    }
    #[doc = "Change the `label` field of this object.\n\n*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("label"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `colorAttachments` field of this object.\n\n*This API requires the following crate features to be activated: `GpuRenderPassDescriptor`*"]
    pub fn color_attachments(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("colorAttachments"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuRenderPassDepthStencilAttachmentDescriptor")]
    #[doc = "Change the `depthStencilAttachment` field of this object.\n\n*This API requires the following crate features to be activated: `GpuRenderPassDepthStencilAttachmentDescriptor`, `GpuRenderPassDescriptor`*"]
    pub fn depth_stencil_attachment(
        &mut self,
        val: &GpuRenderPassDepthStencilAttachmentDescriptor,
    ) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("depthStencilAttachment"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
