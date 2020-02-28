use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUStencilStateFaceDescriptor ) ]
    #[doc = "The `GpuStencilStateFaceDescriptor` dictionary.\n\n*This API requires the following crate features to be activated: `GpuStencilStateFaceDescriptor`*"]
    pub type GpuStencilStateFaceDescriptor;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuStencilStateFaceDescriptor {
    #[doc = "Construct a new `GpuStencilStateFaceDescriptor`.\n\n*This API requires the following crate features to be activated: `GpuStencilStateFaceDescriptor`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "GpuCompareFunction")]
    #[doc = "Change the `compare` field of this object.\n\n*This API requires the following crate features to be activated: `GpuCompareFunction`, `GpuStencilStateFaceDescriptor`*"]
    pub fn compare(&mut self, val: GpuCompareFunction) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("compare"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuStencilOperation")]
    #[doc = "Change the `depthFailOp` field of this object.\n\n*This API requires the following crate features to be activated: `GpuStencilOperation`, `GpuStencilStateFaceDescriptor`*"]
    pub fn depth_fail_op(&mut self, val: GpuStencilOperation) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("depthFailOp"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuStencilOperation")]
    #[doc = "Change the `failOp` field of this object.\n\n*This API requires the following crate features to be activated: `GpuStencilOperation`, `GpuStencilStateFaceDescriptor`*"]
    pub fn fail_op(&mut self, val: GpuStencilOperation) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("failOp"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuStencilOperation")]
    #[doc = "Change the `passOp` field of this object.\n\n*This API requires the following crate features to be activated: `GpuStencilOperation`, `GpuStencilStateFaceDescriptor`*"]
    pub fn pass_op(&mut self, val: GpuStencilOperation) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("passOp"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
