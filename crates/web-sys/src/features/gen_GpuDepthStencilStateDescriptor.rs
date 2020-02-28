use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUDepthStencilStateDescriptor ) ]
    #[doc = "The `GpuDepthStencilStateDescriptor` dictionary.\n\n*This API requires the following crate features to be activated: `GpuDepthStencilStateDescriptor`, `GpuTextureFormat`*"]
    pub type GpuDepthStencilStateDescriptor;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDepthStencilStateDescriptor {
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Construct a new `GpuDepthStencilStateDescriptor`.\n\n*This API requires the following crate features to be activated: `GpuDepthStencilStateDescriptor`, `GpuTextureFormat`*"]
    pub fn new(format: GpuTextureFormat) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.format(format);
        ret
    }
    #[cfg(feature = "GpuCompareFunction")]
    #[doc = "Change the `depthCompare` field of this object.\n\n*This API requires the following crate features to be activated: `GpuCompareFunction`, `GpuDepthStencilStateDescriptor`*"]
    pub fn depth_compare(&mut self, val: GpuCompareFunction) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("depthCompare"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `depthWriteEnabled` field of this object.\n\n*This API requires the following crate features to be activated: `GpuDepthStencilStateDescriptor`*"]
    pub fn depth_write_enabled(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("depthWriteEnabled"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Change the `format` field of this object.\n\n*This API requires the following crate features to be activated: `GpuDepthStencilStateDescriptor`, `GpuTextureFormat`*"]
    pub fn format(&mut self, val: GpuTextureFormat) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("format"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuStencilStateFaceDescriptor")]
    #[doc = "Change the `stencilBack` field of this object.\n\n*This API requires the following crate features to be activated: `GpuDepthStencilStateDescriptor`, `GpuStencilStateFaceDescriptor`*"]
    pub fn stencil_back(&mut self, val: &GpuStencilStateFaceDescriptor) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("stencilBack"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "GpuStencilStateFaceDescriptor")]
    #[doc = "Change the `stencilFront` field of this object.\n\n*This API requires the following crate features to be activated: `GpuDepthStencilStateDescriptor`, `GpuStencilStateFaceDescriptor`*"]
    pub fn stencil_front(&mut self, val: &GpuStencilStateFaceDescriptor) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("stencilFront"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `stencilReadMask` field of this object.\n\n*This API requires the following crate features to be activated: `GpuDepthStencilStateDescriptor`*"]
    pub fn stencil_read_mask(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("stencilReadMask"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `stencilWriteMask` field of this object.\n\n*This API requires the following crate features to be activated: `GpuDepthStencilStateDescriptor`*"]
    pub fn stencil_write_mask(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("stencilWriteMask"),
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
